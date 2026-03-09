#![warn(missing_docs)]
#![allow(clippy::upper_case_acronyms)]
// Same rationale as jni-rs: many Env methods pass raw pointers through macros
// without being marked `unsafe` themselves.
// See: https://github.com/jni-rs/jni-rs/issues/348
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![deny(missing_debug_implementations)]

//! Safe, idiomatic Rust bindings for the JVM Tool Interface (JVMTI).
//!
//! This crate wraps `jvmti2-sys` (raw FFI bindings) and provides:
//!
//! - Lifetime-tracked [`Env`] for safe JVMTI function calls
//! - RAII memory management ([`JvmtiString`], [`JvmtiArray`])
//! - Idiomatic Rust enums for all JVMTI C enumerations
//! - A builder-based [`Capabilities`] API
//! - Both raw fn-pointer and generic/monomorphized event callback APIs
//! - Agent entry-point macros ([`agent_onload!`], [`agent_onattach!`], [`agent_onunload!`])
//! - Tight integration with the [`jni`] crate
//!
//! # Getting Started
//!
//! A JVMTI agent is a native shared library that the JVM loads at startup (or
//! attaches to a running process). The library exports a well-known entry point
//! (`Agent_OnLoad`) where you configure capabilities, install event callbacks,
//! and enable the events you care about.
//!
//! The example below builds a minimal agent that prints every method name as it
//! is entered.
//!
//! ```no_run
//! use std::ffi::CStr;
//! use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode};
//!
//! // 1. Define a handler struct that implements the EventHandler trait.
//! //    You only need to override the callbacks you care about; the rest
//! //    are no-ops by default.
//! struct MyHandler;
//!
//! impl EventHandler for MyHandler {
//!     fn method_entry(
//!         &self,
//!         env: &Env<'_>,
//!         _thread: jni_sys::jobject,
//!         method: jni_sys::jmethodID,
//!     ) {
//!         // Resolve the method name for display.
//!         if let Ok((name, _sig, _generic)) = env.get_method_name(method) {
//!             eprintln!(">> {name}");
//!         }
//!     }
//! }
//!
//! // 2. Write the on_load function that the macro will wire up.
//! fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
//!     // Request the capability we need.
//!     let caps = Capabilities::new().can_generate_method_entry_events();
//!     env.add_capabilities(&caps)?;
//!
//!     // Install our handler and enable the event.
//!     let handler = env.install_event_handler(MyHandler)?;
//!     env.set_event_notification_mode(EventMode::Enable, Event::MethodEntry, None)?;
//!
//!     // Leak the guard -- the agent lives for the JVM's lifetime.
//!     std::mem::forget(handler);
//!     Ok(())
//! }
//!
//! // 3. Generate the Agent_OnLoad entry point.
//! agent_onload!(on_load);
//! ```
//!
//! Key concepts demonstrated above:
//!
//! - **`agent_onload!`** generates the `extern "system" fn Agent_OnLoad` symbol
//!   that the JVM calls when the agent is loaded. There are corresponding
//!   [`agent_onattach!`] and [`agent_onunload!`] macros for dynamic attach and
//!   shutdown hooks.
//!
//! - **[`EventHandler`]** is a trait with 35+ callback methods (one per JVMTI
//!   event). Every method has a default no-op implementation, so you only
//!   override the ones you need.
//!
//! - **[`InstalledHandler`]** is the RAII guard returned by
//!   [`Env::install_event_handler`]. Dropping it clears the callbacks and frees
//!   the handler. Leaking it (via `std::mem::forget`) keeps the callbacks
//!   active for the lifetime of the JVM, which is the typical pattern for
//!   agents.
//!
//! - **[`Env`]** is the central entry point for all JVMTI operations:
//!   capabilities, events, class and method introspection, thread management,
//!   heap walking, and more.
//!
//! # Building and Loading
//!
//! A JVMTI agent must be compiled as a C-compatible shared library
//! (`cdylib`). Your `Cargo.toml` should contain:
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [dependencies]
//! jvmti2 = "0.1"
//! jni-sys = "0.4"
//! ```
//!
//! Build and load the agent:
//!
//! ```sh
//! cargo build --release
//! java -agentpath:target/release/libmy_agent.so MyApp
//! ```
//!
//! The shared library extension depends on the platform: `.so` on Linux,
//! `.dylib` on macOS, and `.dll` on Windows.
//!
//! # Examples
//!
//! The repository ships five complete example agents:
//!
//! - `examples/jvmti-tracing` -- Method entry/exit tracer
//! - `examples/jvmti-class-watcher` -- Class load/prepare monitor
//! - `examples/jvmti-native-spy` -- Native method bind watcher
//! - `examples/jvmti-field-guard` -- Field access/modification watchdog
//! - `examples/jvmti-android-recon` -- Attack surface reconnaissance agent
//!
//! Each example is a standalone Cargo project with its own `Cargo.toml` and
//! a README explaining how to build and run it.
//!
//! # See Also
//!
//! - [JVMTI Specification](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html) --
//!   the authoritative reference for all JVMTI functions, events, and capabilities.
//! - [`jvmti2-sys`](https://crates.io/crates/jvmti2-sys) -- raw FFI bindings
//!   generated from the JVMTI C headers. Re-exported by this crate as [`sys`].
//! - [`jni`](https://crates.io/crates/jni) -- JNI bindings for JVM creation
//!   and Java interop, used alongside this crate when you need to call Java
//!   methods or manipulate Java objects from your agent.

/// Raw FFI bindings re-exported from `jvmti2-sys`.
pub use jvmti2_sys as sys;

/// Error types for JVMTI operations.
pub mod errors;
pub mod enums;
pub mod flags;
pub mod memory;
pub mod objects;
pub mod capabilities;
pub mod monitor;
pub mod event;
pub mod agent;

mod version;
pub use version::JvmtiVersion;

#[macro_use]
mod macros;

mod env;
pub use env::Env;

pub use errors::{JvmtiError, Result};
pub use enums::*;
pub use flags::*;
pub use memory::{JvmtiString, JvmtiArray};
pub use objects::*;
pub use capabilities::Capabilities;
pub use monitor::RawMonitor;
pub use event::{EventHandler, EventCallbacksBuilder, InstalledHandler};
