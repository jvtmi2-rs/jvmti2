# jvmti2 Examples Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement 4 new JVMTI agent examples (security/monitoring focus) and verify all 5 compile cleanly.

**Architecture:** Each example is a standalone Cargo crate under `examples/` with two targets: a cdylib agent (`lib.rs`) loaded via `-agentpath:` and a runner binary (`main.rs`) that creates a JVM and exercises Java code. All agents use `tracing` for output and follow the established pattern from `jvmti-tracing`.

**Tech Stack:** Rust, jvmti2 (path dep), jni 0.22 (invocation feature), jni-sys 0.4, tracing/tracing-subscriber

---

## Conventions

**Every agent lib.rs follows this skeleton:**
```rust
use std::ffi::CStr;
use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode};

struct XxxHandler;
impl EventHandler for XxxHandler { /* callbacks */ }

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    // 1. init tracing
    // 2. add capabilities
    // 3. install handler
    // 4. enable events
    // 5. leak handler guard
    Ok(())
}
agent_onload!(on_load);
```

**Every runner main.rs follows this skeleton:**
```rust
use jni::{jni_sig, jni_str, objects::*, InitArgsBuilder, JavaVM};

fn main() {
    let agent_path = find_agent_library();
    let agent_opt = format!("-agentpath:{agent_path}");
    let args = InitArgsBuilder::new().option(&agent_opt).build().expect("...");
    let jvm = JavaVM::new(args).expect("...");
    jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
        // exercise Java code
        Ok(())
    }).expect("...");
}

fn find_agent_library() -> String { /* OS-aware DLL lookup */ }
```

**Every Cargo.toml follows this template** (change `name` fields only):
```toml
[package]
name = "jvmti-xxx"
version = "0.0.1"
edition = "2021"

[lib]
name = "jvmti_xxx_agent"
crate-type = ["cdylib"]

[[bin]]
name = "jvmti-xxx"
path = "src/main.rs"

[dependencies]
jvmti2 = { path = "../.." }
jni = { version = "0.22", features = ["invocation"] }
jni-sys = "0.4"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**Shared helper `describe_method`** — resolves `jmethodID` to `"ClassName.methodName"`:
```rust
fn describe_method(env: &Env<'_>, method: jni_sys::jmethodID) -> jvmti2::Result<String> {
    let (name, _sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(klass)?;
    let class_name = class_sig.to_string_lossy();
    let class_name = class_name.strip_prefix('L').unwrap_or(&class_name);
    let class_name = class_name.strip_suffix(';').unwrap_or(class_name).replace('/', ".");
    Ok(format!("{class_name}.{name}"))
}
```

**Shared helper `format_class_name`** — formats a raw JVM class signature:
```rust
fn format_class_name(sig: &str) -> String {
    let s = sig.strip_prefix('L').unwrap_or(sig);
    let s = s.strip_suffix(';').unwrap_or(s);
    s.replace('/', ".")
}
```

---

## Task 1: jvmti-class-watcher

**Files:**
- Create: `examples/jvmti-class-watcher/Cargo.toml`
- Create: `examples/jvmti-class-watcher/src/lib.rs`
- Create: `examples/jvmti-class-watcher/src/main.rs`

### Step 1: Create Cargo.toml

```toml
[package]
name = "jvmti-class-watcher"
version = "0.0.1"
edition = "2021"

[lib]
name = "jvmti_class_watcher_agent"
crate-type = ["cdylib"]

[[bin]]
name = "jvmti-class-watcher"
path = "src/main.rs"

[dependencies]
jvmti2 = { path = "../.." }
jni = { version = "0.22", features = ["invocation"] }
jni-sys = "0.4"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

### Step 2: Create lib.rs — Class Load/Prepare agent

```rust
//! JVMTI agent that monitors class loading.
//!
//! Logs every class load and class prepare event, showing the class name,
//! status flags, source file, method count, and class loader info.

use std::ffi::CStr;

use jvmti2::{agent_onload, Env, Event, EventHandler, EventMode};

struct ClassWatcherHandler;

impl EventHandler for ClassWatcherHandler {
    fn class_load(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        klass: jni_sys::jclass,
    ) {
        if let Ok((sig, _generic)) = env.get_class_signature(klass) {
            let name = format_class_name(&sig.to_string_lossy());
            tracing::info!("[LOAD] {name}");
        }
    }

    fn class_prepare(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        klass: jni_sys::jclass,
    ) {
        let Ok((sig, _generic)) = env.get_class_signature(klass) else { return };
        let name = format_class_name(&sig.to_string_lossy());

        let status = env.get_class_status(klass).map(|s| format!("{s:?}")).unwrap_or_default();
        let methods = env.get_class_methods(klass).map(|m| m.len()).unwrap_or(0);
        let source = env.get_source_file_name(klass)
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "<unknown>".into());
        let loader = match env.get_class_loader(klass) {
            Ok(Some(loader)) => {
                // Try to get the loader's class name for display.
                format!("{loader:?}")
            }
            Ok(None) => "bootstrap".into(),
            Err(_) => "<error>".into(),
        };

        tracing::info!(
            "[PREPARED] {name} | status={status} methods={methods} source={source} loader={loader}"
        );
    }
}

fn format_class_name(sig: &str) -> String {
    let s = sig.strip_prefix('L').unwrap_or(sig);
    let s = s.strip_suffix(';').unwrap_or(s);
    s.replace('/', ".")
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    tracing::info!("jvmti-class-watcher agent loaded");

    // ClassLoad and ClassPrepare don't require extra capabilities.
    let handler = env.install_event_handler(ClassWatcherHandler)?;

    env.set_event_notification_mode(EventMode::Enable, Event::ClassLoad, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::ClassPrepare, None)?;

    std::mem::forget(handler);
    tracing::info!("class load/prepare monitoring enabled");
    Ok(())
}

agent_onload!(on_load);
```

### Step 3: Create main.rs — Runner that loads various classes

```rust
//! Exercises various class-loading paths to demonstrate the class watcher agent.

use jni::{jni_sig, jni_str, objects::JValue, InitArgsBuilder, JavaVM};

fn main() {
    let agent_path = find_agent_library();
    println!("Loading agent from: {agent_path}");

    let agent_opt = format!("-agentpath:{agent_path}");
    let args = InitArgsBuilder::new()
        .option(&agent_opt)
        .build()
        .expect("Failed to build JVM init args");

    let jvm = JavaVM::new(args).expect("Failed to create JVM");

    jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
        // Trigger class loading for several standard library classes.
        println!("--- Loading ArrayList ---");
        env.find_class(jni_str!("java/util/ArrayList"))?;

        println!("--- Loading HashMap ---");
        env.find_class(jni_str!("java/util/HashMap"))?;

        println!("--- Loading Thread ---");
        env.find_class(jni_str!("java/lang/Thread"))?;

        // Use Class.forName to trigger reflective class loading.
        println!("--- Reflective load: java.net.URL ---");
        let class_cls = env.find_class(jni_str!("java/lang/Class"))?;
        let url_name = env.new_string("java.net.URL")?;
        env.call_static_method(
            &class_cls,
            jni_str!("forName"),
            jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
            &[JValue::Object(&url_name)],
        )?;

        println!("--- Done ---");
        Ok(())
    })
    .expect("JNI call failed");
}

fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");
    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_class_watcher_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_class_watcher_agent.dylib"
    } else {
        "libjvmti_class_watcher_agent.so"
    };
    let path = dir.join(lib_name);
    assert!(path.exists(), "Agent library not found at {path:?}. Build first with `cargo build`.");
    path.to_string_lossy().into_owned()
}
```

### Step 4: Verify

Run: `cd examples/jvmti-class-watcher && cargo check && cargo clippy`
Expected: zero errors, zero warnings

### Step 5: Commit

```bash
git add examples/jvmti-class-watcher/
git commit -m "Add jvmti-class-watcher example: class load monitoring agent"
```

---

## Task 2: jvmti-native-spy

**Files:**
- Create: `examples/jvmti-native-spy/Cargo.toml`
- Create: `examples/jvmti-native-spy/src/lib.rs`
- Create: `examples/jvmti-native-spy/src/main.rs`

### Step 1: Create Cargo.toml

Same template, names: `jvmti-native-spy` / `jvmti_native_spy_agent`.

### Step 2: Create lib.rs — Native method bind agent

```rust
//! JVMTI agent that watches native method binding.
//!
//! Logs every native method bind event, showing which Java native method
//! is being linked to which native function address. This is how security
//! tools detect JNI library loading and potential native code injection.

use std::ffi::{c_void, CStr};

use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode};

struct NativeSpyHandler;

impl EventHandler for NativeSpyHandler {
    fn native_method_bind(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        address: *mut c_void,
        new_address_ptr: *mut *mut c_void,
    ) {
        if let Ok(desc) = describe_method(env, method) {
            let can_redirect = !new_address_ptr.is_null();
            tracing::info!(
                "[BIND] {desc} -> {address:?} (redirect={can_redirect})"
            );
        }
    }
}

fn describe_method(env: &Env<'_>, method: jni_sys::jmethodID) -> jvmti2::Result<String> {
    let (name, sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(klass)?;
    let class_name = class_sig.to_string_lossy();
    let class_name = class_name.strip_prefix('L').unwrap_or(&class_name);
    let class_name = class_name.strip_suffix(';').unwrap_or(class_name).replace('/', ".");
    Ok(format!("{class_name}.{name}{sig}"))
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    tracing::info!("jvmti-native-spy agent loaded");

    let caps = Capabilities::new().can_generate_native_method_bind_events();
    env.add_capabilities(&caps)?;

    let handler = env.install_event_handler(NativeSpyHandler)?;
    env.set_event_notification_mode(EventMode::Enable, Event::NativeMethodBind, None)?;

    std::mem::forget(handler);
    tracing::info!("native method bind monitoring enabled");
    Ok(())
}

agent_onload!(on_load);
```

### Step 3: Create main.rs — Runner that triggers native method binds

```rust
//! Exercises Java native methods to demonstrate the native spy agent.

use jni::{jni_sig, jni_str, objects::JValue, InitArgsBuilder, JavaVM};

fn main() {
    let agent_path = find_agent_library();
    println!("Loading agent from: {agent_path}");

    let agent_opt = format!("-agentpath:{agent_path}");
    let args = InitArgsBuilder::new()
        .option(&agent_opt)
        .build()
        .expect("Failed to build JVM init args");

    let jvm = JavaVM::new(args).expect("Failed to create JVM");

    jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
        // System.currentTimeMillis() — native method
        println!("--- System.currentTimeMillis() ---");
        let system = env.find_class(jni_str!("java/lang/System"))?;
        let time = env.call_static_method(
            &system,
            jni_str!("currentTimeMillis"),
            jni_sig!("()J"),
            &[],
        )?;
        println!("Time: {:?}", time.j());

        // System.arraycopy — native method
        println!("--- System.arraycopy() ---");
        let src = env.new_int_array(4)?;
        let dst = env.new_int_array(4)?;
        env.call_static_method(
            &system,
            jni_str!("arraycopy"),
            jni_sig!("(Ljava/lang/Object;ILjava/lang/Object;II)V"),
            &[
                JValue::Object(&src),
                JValue::Int(0),
                JValue::Object(&dst),
                JValue::Int(0),
                JValue::Int(4),
            ],
        )?;

        // Thread.currentThread() — native method
        println!("--- Thread.currentThread() ---");
        let thread_cls = env.find_class(jni_str!("java/lang/Thread"))?;
        env.call_static_method(
            &thread_cls,
            jni_str!("currentThread"),
            jni_sig!("()Ljava/lang/Thread;"),
            &[],
        )?;

        println!("--- Done ---");
        Ok(())
    })
    .expect("JNI call failed");
}

fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");
    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_native_spy_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_native_spy_agent.dylib"
    } else {
        "libjvmti_native_spy_agent.so"
    };
    let path = dir.join(lib_name);
    assert!(path.exists(), "Agent library not found at {path:?}. Build first with `cargo build`.");
    path.to_string_lossy().into_owned()
}
```

### Step 4: Verify

Run: `cd examples/jvmti-native-spy && cargo check && cargo clippy`
Expected: zero errors, zero warnings

### Step 5: Commit

```bash
git add examples/jvmti-native-spy/
git commit -m "Add jvmti-native-spy example: native method bind watcher"
```

---

## Task 3: jvmti-field-guard

**Files:**
- Create: `examples/jvmti-field-guard/Cargo.toml`
- Create: `examples/jvmti-field-guard/src/lib.rs`
- Create: `examples/jvmti-field-guard/src/main.rs`

### Step 1: Create Cargo.toml

Same template, names: `jvmti-field-guard` / `jvmti_field_guard_agent`.

### Step 2: Create lib.rs — Field access/modification watchdog

Key design: We set field watches during the `vm_init` event (JNI is fully available).
We use JVMTI's `get_class_fields` + `get_field_name` to find the `out` and `err` fields
on `java.lang.System`, then call `set_field_access_watch` on each.

The `vm_init` callback receives `&Env` (not `&mut Env`), but `set_field_access_watch`
only needs `&self`, so this works.

For getting the `jclass` during `vm_init`, we use `env.get_java_vm()` -> attach ->
JNI `FindClass`. Alternatively we can use the `class_prepare` event to catch
`java.lang.System` and grab its `jclass`. The simplest approach: use the JNI route
during `vm_init`.

```rust
//! JVMTI agent that guards field access on sensitive static fields.
//!
//! Sets access watches on `System.out` and `System.err`, logging every
//! read with the calling method and thread. Demonstrates field watchpoints
//! for tamper detection on security-sensitive fields.

use std::ffi::CStr;

use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode};

struct FieldGuardHandler;

impl EventHandler for FieldGuardHandler {
    fn vm_init(&self, env: &Env<'_>, _thread: jni_sys::jobject) {
        if let Err(e) = setup_watches(env) {
            tracing::error!("Failed to set field watches: {e}");
        }
    }

    fn field_access(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        _location: jvmti2::sys::jlocation,
        field_klass: jni_sys::jclass,
        _object: jni_sys::jobject,
        field: jni_sys::jfieldID,
    ) {
        let field_desc = describe_field(env, field_klass, field);
        let caller = if method.is_null() {
            "<unknown>".into()
        } else {
            describe_method(env, method).unwrap_or_else(|_| "<unknown>".into())
        };
        tracing::warn!("[ACCESS] {field_desc} read by {caller}");
    }

    fn field_modification(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        _location: jvmti2::sys::jlocation,
        field_klass: jni_sys::jclass,
        _object: jni_sys::jobject,
        field: jni_sys::jfieldID,
        _signature_type: std::ffi::c_char,
        _new_value: jni_sys::jvalue,
    ) {
        let field_desc = describe_field(env, field_klass, field);
        let caller = if method.is_null() {
            "<unknown>".into()
        } else {
            describe_method(env, method).unwrap_or_else(|_| "<unknown>".into())
        };
        tracing::error!("[MODIFY] {field_desc} written by {caller}");
    }
}

fn setup_watches(env: &Env<'_>) -> jvmti2::Result<()> {
    // Use JNI to find java.lang.System class.
    let jvm = env.get_java_vm();
    jvm.attach_current_thread(|jni_env| -> jvmti2::Result<()> {
        let system_cls = jni_env
            .find_class(jni::jni_str!("java/lang/System"))
            .map_err(jvmti2::JvmtiError::Jni)?;
        let system_jclass: jni_sys::jclass = system_cls.as_raw();

        // Iterate JVMTI fields to find "out" and "err".
        let fields = env.get_class_fields(system_jclass)?;
        for &fid in fields.as_slice() {
            let (name, _sig, _generic) = env.get_field_name(system_jclass, fid)?;
            let name_str = name.to_string_lossy();
            if name_str == "out" || name_str == "err" {
                tracing::info!("Setting access watch on System.{name_str}");
                env.set_field_access_watch(system_jclass, fid)?;
            }
        }
        Ok(())
    })
    .map_err(|e| jvmti2::JvmtiError::Jni(jni::errors::Error::from(e)))?
}

fn describe_field(env: &Env<'_>, klass: jni_sys::jclass, field: jni_sys::jfieldID) -> String {
    let class_name = env
        .get_class_signature(klass)
        .map(|(sig, _)| format_class_name(&sig.to_string_lossy()))
        .unwrap_or_else(|_| "<unknown>".into());
    let field_name = env
        .get_field_name(klass, field)
        .map(|(name, _sig, _)| name.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "<unknown>".into());
    format!("{class_name}.{field_name}")
}

fn describe_method(env: &Env<'_>, method: jni_sys::jmethodID) -> jvmti2::Result<String> {
    let (name, _sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(klass)?;
    let class_name = format_class_name(&class_sig.to_string_lossy());
    Ok(format!("{class_name}.{name}"))
}

fn format_class_name(sig: &str) -> String {
    let s = sig.strip_prefix('L').unwrap_or(sig);
    let s = s.strip_suffix(';').unwrap_or(s);
    s.replace('/', ".")
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    tracing::info!("jvmti-field-guard agent loaded");

    let caps = Capabilities::new()
        .can_generate_field_access_events()
        .can_generate_field_modification_events();
    env.add_capabilities(&caps)?;

    let handler = env.install_event_handler(FieldGuardHandler)?;

    env.set_event_notification_mode(EventMode::Enable, Event::VmInit, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::FieldAccess, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::FieldModification, None)?;

    std::mem::forget(handler);
    tracing::info!("field guard enabled (watches set on VMInit)");
    Ok(())
}

agent_onload!(on_load);
```

**Important note:** The `setup_watches` function uses `env.get_java_vm()` to get a
`jni::JavaVM` and then calls `attach_current_thread` for JNI FindClass. This works
because during `vm_init` the current thread is already attached, so the nested attach
is a no-op that gives us the existing JNI env. We need the `jclass` from JNI to pass
to JVMTI's field methods.

If the nested `attach_current_thread` during `vm_init` causes issues (the thread is
already attached by the JVM), the fallback approach is to use JVMTI's
`get_loaded_classes()` to iterate all loaded classes and find System by signature.
This alternative doesn't require JNI at all:

```rust
// Fallback if nested attach doesn't work:
fn setup_watches_fallback(env: &Env<'_>) -> jvmti2::Result<()> {
    let classes = env.get_loaded_classes()?;
    for &klass in classes.as_slice() {
        let (sig, _) = env.get_class_signature(klass)?;
        if sig.to_string_lossy() == "Ljava/lang/System;" {
            let fields = env.get_class_fields(klass)?;
            for &fid in fields.as_slice() {
                let (name, _, _) = env.get_field_name(klass, fid)?;
                let name_str = name.to_string_lossy();
                if name_str == "out" || name_str == "err" {
                    tracing::info!("Setting access watch on System.{name_str}");
                    env.set_field_access_watch(klass, fid)?;
                }
            }
            break;
        }
    }
    Ok(())
}
```

### Step 3: Create main.rs — Runner that accesses guarded fields

```rust
//! Accesses System.out and System.err to trigger field guard events.

use jni::{jni_sig, jni_str, objects::JValue, InitArgsBuilder, JavaVM};

fn main() {
    let agent_path = find_agent_library();
    println!("Loading agent from: {agent_path}");

    let agent_opt = format!("-agentpath:{agent_path}");
    let args = InitArgsBuilder::new()
        .option(&agent_opt)
        .build()
        .expect("Failed to build JVM init args");

    let jvm = JavaVM::new(args).expect("Failed to create JVM");

    jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
        // Access System.out — triggers field access event.
        println!("--- Accessing System.out ---");
        let system = env.find_class(jni_str!("java/lang/System"))?;
        let out = env
            .get_static_field(&system, jni_str!("out"), jni_sig!("Ljava/io/PrintStream;"))?
            .l()?;

        // Use System.out.println — triggers another access to out.
        let msg = env.new_string("Hello from field-guard runner!")?;
        env.call_method(
            out,
            jni_str!("println"),
            jni_sig!("(Ljava/lang/Object;)V"),
            &[JValue::Object(&msg)],
        )?;

        // Access System.err — triggers field access event.
        println!("--- Accessing System.err ---");
        let err = env
            .get_static_field(&system, jni_str!("err"), jni_sig!("Ljava/io/PrintStream;"))?
            .l()?;

        let msg2 = env.new_string("Hello from System.err!")?;
        env.call_method(
            err,
            jni_str!("println"),
            jni_sig!("(Ljava/lang/Object;)V"),
            &[JValue::Object(&msg2)],
        )?;

        println!("--- Done ---");
        Ok(())
    })
    .expect("JNI call failed");
}

fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");
    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_field_guard_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_field_guard_agent.dylib"
    } else {
        "libjvmti_field_guard_agent.so"
    };
    let path = dir.join(lib_name);
    assert!(path.exists(), "Agent library not found at {path:?}. Build first with `cargo build`.");
    path.to_string_lossy().into_owned()
}
```

### Step 4: Verify

Run: `cd examples/jvmti-field-guard && cargo check && cargo clippy`
Expected: zero errors, zero warnings

If compilation issues with `setup_watches` (nested attach), switch to `setup_watches_fallback`.

### Step 5: Commit

```bash
git add examples/jvmti-field-guard/
git commit -m "Add jvmti-field-guard example: field access watchdog agent"
```

---

## Task 4: jvmti-android-recon

**Files:**
- Create: `examples/jvmti-android-recon/Cargo.toml`
- Create: `examples/jvmti-android-recon/src/lib.rs`
- Create: `examples/jvmti-android-recon/src/main.rs`

### Step 1: Create Cargo.toml

Same template, names: `jvmti-android-recon` / `jvmti_android_recon_agent`.

### Step 2: Create lib.rs — Attack surface reconnaissance agent

```rust
//! JVMTI agent for attack surface reconnaissance.
//!
//! Monitors method entry events and native method binds, filtering for
//! security-interesting API calls: crypto, networking, file I/O, reflection,
//! process execution, and dynamic class loading.
//!
//! Designed for both standard JVM and Android ART (JVMTI supported since
//! Android 8 / API 26). On Android, extend the prefix list with
//! `android.content.Intent`, `android.os.Binder`, `dalvik.system.DexClassLoader`.

use std::ffi::{c_void, CStr};

use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode};

/// Security-interesting class prefixes and their category tags.
const WATCH_PREFIXES: &[(&str, &str)] = &[
    ("javax.crypto.", "CRYPTO"),
    ("java.security.", "SECURITY"),
    ("java.net.", "NET"),
    ("javax.net.", "NET"),
    ("java.io.File", "FILE"),
    ("java.nio.file.", "FILE"),
    ("java.lang.reflect.", "REFLECT"),
    ("java.lang.Class.forName", "DYNLOAD"),
    ("java.lang.Class.getMethod", "REFLECT"),
    ("java.lang.Runtime.exec", "EXEC"),
    ("java.lang.Runtime.getRuntime", "EXEC"),
    ("java.lang.ProcessBuilder", "EXEC"),
    // Android-specific (no-op on standard JVM, but ready for ART):
    ("dalvik.system.DexClassLoader", "DYNLOAD"),
    ("dalvik.system.PathClassLoader", "DYNLOAD"),
    ("android.content.Intent", "IPC"),
    ("android.os.Binder", "IPC"),
    ("android.app.Activity.startActivity", "IPC"),
];

struct ReconHandler;

impl EventHandler for ReconHandler {
    fn method_entry(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
    ) {
        let Ok(full_name) = describe_method(env, method) else { return };

        for &(prefix, tag) in WATCH_PREFIXES {
            if full_name.starts_with(prefix) {
                tracing::warn!("[{tag}] >> {full_name}");
                return;
            }
        }
    }

    fn native_method_bind(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        address: *mut c_void,
        _new_address_ptr: *mut *mut c_void,
    ) {
        if let Ok(desc) = describe_method(env, method) {
            tracing::info!("[NATIVE] {desc} -> {address:?}");
        }
    }
}

fn describe_method(env: &Env<'_>, method: jni_sys::jmethodID) -> jvmti2::Result<String> {
    let (name, _sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(klass)?;
    let class_name = class_sig.to_string_lossy();
    let class_name = class_name.strip_prefix('L').unwrap_or(&class_name);
    let class_name = class_name.strip_suffix(';').unwrap_or(class_name).replace('/', ".");
    Ok(format!("{class_name}.{name}"))
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    tracing::info!("jvmti-android-recon agent loaded");
    tracing::info!("Watching {} security-interesting prefixes", WATCH_PREFIXES.len());

    let caps = Capabilities::new()
        .can_generate_method_entry_events()
        .can_generate_native_method_bind_events();
    env.add_capabilities(&caps)?;

    let handler = env.install_event_handler(ReconHandler)?;

    env.set_event_notification_mode(EventMode::Enable, Event::MethodEntry, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::NativeMethodBind, None)?;

    std::mem::forget(handler);
    tracing::info!("attack surface reconnaissance enabled");
    Ok(())
}

agent_onload!(on_load);
```

### Step 3: Create main.rs — Runner that exercises security-interesting APIs

```rust
//! Exercises security-interesting Java APIs to demonstrate the recon agent.
//!
//! Simulates what a reverse engineer would see when attaching to an app
//! that uses crypto, networking, file I/O, reflection, and process APIs.

use jni::{jni_sig, jni_str, objects::JValue, InitArgsBuilder, JavaVM};

fn main() {
    let agent_path = find_agent_library();
    println!("Loading agent from: {agent_path}");

    let agent_opt = format!("-agentpath:{agent_path}");
    let args = InitArgsBuilder::new()
        .option(&agent_opt)
        .build()
        .expect("Failed to build JVM init args");

    let jvm = JavaVM::new(args).expect("Failed to create JVM");

    jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
        // --- CRYPTO: MessageDigest.getInstance("SHA-256") ---
        println!("=== CRYPTO ===");
        let md_cls = env.find_class(jni_str!("java/security/MessageDigest"))?;
        let algo = env.new_string("SHA-256")?;
        let md = env.call_static_method(
            &md_cls,
            jni_str!("getInstance"),
            jni_sig!("(Ljava/lang/String;)Ljava/security/MessageDigest;"),
            &[JValue::Object(&algo)],
        )?.l()?;
        let data = env.new_string("hello")?;
        let bytes = env.call_method(
            &data,
            jni_str!("getBytes"),
            jni_sig!("()[B"),
            &[],
        )?.l()?;
        env.call_method(
            &md,
            jni_str!("update"),
            jni_sig!("([B)V"),
            &[JValue::Object(&bytes)],
        )?;
        println!("  MessageDigest SHA-256 updated");

        // --- NET: new URL("https://example.com") ---
        println!("=== NET ===");
        let url_cls = env.find_class(jni_str!("java/net/URL"))?;
        let url_str = env.new_string("https://example.com")?;
        env.new_object(
            &url_cls,
            jni_sig!("(Ljava/lang/String;)V"),
            &[JValue::Object(&url_str)],
        )?;
        println!("  URL object created");

        // --- FILE: new File("/tmp/test").exists() ---
        println!("=== FILE ===");
        let file_cls = env.find_class(jni_str!("java/io/File"))?;
        let path = env.new_string("/tmp/test")?;
        let file = env.new_object(
            &file_cls,
            jni_sig!("(Ljava/lang/String;)V"),
            &[JValue::Object(&path)],
        )?;
        let exists = env.call_method(
            &file,
            jni_str!("exists"),
            jni_sig!("()Z"),
            &[],
        )?.z()?;
        println!("  File exists: {exists}");

        // --- REFLECT: Class.forName("java.util.HashMap") ---
        println!("=== REFLECT ===");
        let class_cls = env.find_class(jni_str!("java/lang/Class"))?;
        let class_name = env.new_string("java.util.HashMap")?;
        env.call_static_method(
            &class_cls,
            jni_str!("forName"),
            jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
            &[JValue::Object(&class_name)],
        )?;
        println!("  Class.forName loaded HashMap");

        // --- EXEC: Runtime.getRuntime() ---
        println!("=== EXEC ===");
        let runtime_cls = env.find_class(jni_str!("java/lang/Runtime"))?;
        env.call_static_method(
            &runtime_cls,
            jni_str!("getRuntime"),
            jni_sig!("()Ljava/lang/Runtime;"),
            &[],
        )?;
        println!("  Runtime.getRuntime() called");

        println!("=== RECON COMPLETE ===");
        Ok(())
    })
    .expect("JNI call failed");
}

fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");
    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_android_recon_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_android_recon_agent.dylib"
    } else {
        "libjvmti_android_recon_agent.so"
    };
    let path = dir.join(lib_name);
    assert!(path.exists(), "Agent library not found at {path:?}. Build first with `cargo build`.");
    path.to_string_lossy().into_owned()
}
```

### Step 4: Verify

Run: `cd examples/jvmti-android-recon && cargo check && cargo clippy`
Expected: zero errors, zero warnings

### Step 5: Commit

```bash
git add examples/jvmti-android-recon/
git commit -m "Add jvmti-android-recon example: attack surface reconnaissance agent"
```

---

## Task 5: Final verification

### Step 1: Check all examples compile

```bash
cd examples/jvmti-class-watcher && cargo check && cargo clippy 2>&1
cd ../jvmti-native-spy && cargo check && cargo clippy 2>&1
cd ../jvmti-field-guard && cargo check && cargo clippy 2>&1
cd ../jvmti-android-recon && cargo check && cargo clippy 2>&1
```

### Step 2: Also verify the main crate still compiles

```bash
cd ../.. && cargo check && cargo clippy 2>&1
```

### Step 3: Commit any fixups

If any compilation fixes were needed, commit them.
