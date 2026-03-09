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
