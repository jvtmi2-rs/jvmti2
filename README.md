# jvmti2

[![crates.io](https://img.shields.io/crates/v/jvmti2.svg)](https://crates.io/crates/jvmti2)
[![docs.rs](https://docs.rs/jvmti2/badge.svg)](https://docs.rs/jvmti2)
[![License](https://img.shields.io/crates/l/jvmti2.svg)](https://github.com/jvtmi2-rs/jvmti2#license)

Safe, idiomatic Rust bindings for the JVM Tool Interface (JVMTI). Build
profiling, debugging, and security agents that hook into the JVM's internals --
method tracing, class loading, field watchpoints, native method binding, and
more.

## Quick Start

Add `jvmti2` to your agent crate (must be a `cdylib`):

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
jvmti2 = "0.1"
jni-sys = "0.4"
```

Write a minimal agent in `src/lib.rs`:

```rust
use std::ffi::CStr;
use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode};

struct MyHandler;

impl EventHandler for MyHandler {
    fn method_entry(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
    ) {
        let name = env.get_method_name(method).map(|(n, _, _)| n.to_string());
        if let Ok(name) = name {
            println!(">> {name}");
        }
    }
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    let caps = Capabilities::new()
        .can_generate_method_entry_events();
    env.add_capabilities(&caps)?;

    let guard = env.install_event_handler(MyHandler)?;
    env.set_event_notification_mode(EventMode::Enable, Event::MethodEntry, None)?;

    // Leak the guard so callbacks stay active for the JVM's lifetime.
    std::mem::forget(guard);
    Ok(())
}

agent_onload!(on_load);
```

Build and load the agent:

```sh
cargo build --release
java -agentpath:target/release/libmy_agent.so MyApp
```

## Features

- Lifetime-tracked `Env` for safe JVMTI calls
- RAII memory management (`JvmtiString`, `JvmtiArray`)
- Idiomatic Rust enums for JVMTI C enumerations
- Builder-based `Capabilities` API
- Trait-based event handler (`EventHandler`) with 35+ callbacks
- Agent entry-point macros (`agent_onload!`, `agent_onattach!`, `agent_onunload!`)
- Integration with the [`jni`](https://crates.io/crates/jni) crate

## Examples

| Example | Description |
|---------|-------------|
| [jvmti-tracing](examples/jvmti-tracing) | Method entry/exit tracer |
| [jvmti-class-watcher](examples/jvmti-class-watcher) | Class load/prepare monitor |
| [jvmti-native-spy](examples/jvmti-native-spy) | Native method bind watcher |
| [jvmti-field-guard](examples/jvmti-field-guard) | Field access/modification watchdog |
| [jvmti-android-recon](examples/jvmti-android-recon) | Attack surface reconnaissance agent |

## Requirements

- Rust 1.70+
- A JDK installation (for running agents)

## See Also

- [JVMTI Specification](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html)
- [jvmti2-sys](https://crates.io/crates/jvmti2-sys) -- raw FFI bindings
- [jni](https://crates.io/crates/jni) -- JNI bindings for Rust

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.
