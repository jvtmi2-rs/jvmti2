# Changelog

All notable changes to this project will be documented in this file.

## 0.1.0 (unreleased)

Initial public release.

- Safe, lifetime-tracked `Env` wrapping all 143 JVMTI functions
- RAII memory management with `JvmtiString` and `JvmtiArray`
- Idiomatic Rust enums for JVMTI C enumerations
- Builder-based `Capabilities` API with 45 capability flags
- `EventHandler` trait with 35+ event callbacks
- Raw fn-pointer event API via `EventCallbacksBuilder`
- `RawMonitor` with RAII locking
- Agent entry-point macros: `agent_onload!`, `agent_onattach!`, `agent_onunload!`
- Integration with the `jni` crate (`Env::get_java_vm()`)
- Five example agents: tracing, class-watcher, native-spy, field-guard, android-recon
