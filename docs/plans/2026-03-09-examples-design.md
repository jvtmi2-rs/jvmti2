# jvmti2 Examples — Design Document

## Goal

Ship 4 new examples (5 total) demonstrating security/monitoring use cases
before bumping the crate version to 0.1.0. Each example is a self-contained
Cargo crate under `examples/` with a cdylib agent and a runner binary.

## Architecture

Every example follows the same two-target pattern:

```
examples/<name>/
  Cargo.toml          # cdylib lib + bin targets
  src/
    lib.rs            # JVMTI agent (cdylib loaded via -agentpath:)
    main.rs           # Runner binary that creates JVM + exercises Java code
```

The runner binary locates the agent DLL/so next to itself (Cargo puts both
in `target/{profile}/`) and passes it via `-agentpath:`.

Dependencies shared by all examples:
- `jvmti2 = { path = "../.." }`
- `jni = { version = "0.22", features = ["invocation"] }` (runner only)
- `jni-sys = "0.4"` (raw types in EventHandler signatures)
- `tracing` + `tracing-subscriber` (structured logging)

## Examples

### 1. jvmti-tracing (existing)

Basic method entry/exit tracer. Already implemented and compiling.

### 2. jvmti-class-watcher — Class Load Monitor

**Events:** `ClassLoad`, `ClassPrepare`
**Capabilities:** none required (these events are always available)

**Agent (lib.rs):**
- `ClassLoad`: log class signature via `get_class_signature(klass)`
- `ClassPrepare`: log class status (`get_class_status`), source file
  (`get_source_file_name`), method count (`get_class_methods().len()`),
  and class loader (`get_class_loader`)

**Runner (main.rs):**
- Load several classes to trigger events: ArrayList, HashMap, Thread,
  URL, and use reflection (`Class.forName`) to load a class dynamically
- This exercises both normal loading and reflective loading paths

### 3. jvmti-native-spy — Native Method Bind Watcher

**Events:** `NativeMethodBind`
**Capabilities:** `can_generate_native_method_bind_events`

**Agent (lib.rs):**
- `native_method_bind`: resolve method name + declaring class, log the
  bind address. Show the `new_address_ptr` parameter (the hook point
  where you could redirect the native call)

**Runner (main.rs):**
- Call methods that trigger native binds: `System.arraycopy`,
  `System.currentTimeMillis`, `Object.hashCode`, `Thread.currentThread`
- These are all native methods that will trigger bind events

### 4. jvmti-field-guard — Field Access/Modification Watchdog

**Events:** `FieldAccess`, `FieldModification`
**Capabilities:** `can_generate_field_modification_events`,
`can_generate_field_access_events`

**Agent (lib.rs):**
- On `VMInit`: look up `java/lang/System` class, find `out` and `err`
  fields, set access watches on both
- `field_access`: log which thread/method is reading the watched field
- `field_modification`: log what the new value is

**Runner (main.rs):**
- Access `System.out` and `System.err` to trigger field access events
- Create a simple Java class scenario using reflection that reads these
  fields through different code paths

### 5. jvmti-android-recon — Attack Surface Reconnaissance

**Events:** `MethodEntry`, `NativeMethodBind`
**Capabilities:** `can_generate_method_entry_events`,
`can_generate_native_method_bind_events`

**Agent (lib.rs):**
- `method_entry`: resolve class + method name, check against a set of
  security-interesting prefixes:
  - `javax.crypto.` — crypto operations
  - `java.security.` — security framework
  - `java.net.` — network I/O
  - `java.io.File` — file I/O
  - `java.lang.reflect.` — reflection
  - `java.lang.Class.forName` — dynamic class loading
  - `java.lang.Runtime.exec` — process execution
  - `java.lang.ProcessBuilder` — process execution
- `native_method_bind`: log all native method registrations
- Categorize and tag each hit (CRYPTO, NET, FILE, REFLECT, EXEC, NATIVE)

**Runner (main.rs):**
- Exercise security-interesting APIs:
  - Create a `MessageDigest` (crypto)
  - Open a `URL` connection (network)
  - Create a `File` and check `exists()` (file I/O)
  - Use `Class.forName` (reflection/dynamic loading)
  - Access `Runtime.getRuntime()` (exec surface)
- This simulates what a reverse engineer would see when attaching to
  an app that uses these APIs

Note: This agent is designed to work on both standard JVM and Android ART
(which supports JVMTI since Android 8/API 26). On Android, it would be
loaded via `Debug.attachJvmtiAgent()` or via startup agent config. The
prefix list can be extended with Android-specific classes like
`android.content.Intent`, `android.os.Binder`, `dalvik.system.DexClassLoader`.

## Runner Pattern

All runners share the same `find_agent_library()` helper. The runner code
uses jni 0.22 API:
- `InitArgsBuilder::new().option("-agentpath:...").build()`
- `JavaVM::new(args)` (requires `invocation` feature)
- `jvm.attach_current_thread(|env| { ... })` (callback-based)
- `jni_str!()` for class/method names, `jni_sig!()` for signatures

## Success Criteria

- All 5 examples compile with `cargo check`
- All 5 examples pass `cargo clippy` with zero warnings
- Each example can be run with `cargo run` (requires JDK installed)
