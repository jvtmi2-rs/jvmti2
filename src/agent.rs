//! Agent entry-point macros.
//!
//! JVMTI agents are loaded by the JVM through well-known C entry points.
//! These macros generate the required `#[no_mangle] extern "system"` functions
//! and wire them to idiomatic Rust handlers.
//!
//! # Example
//!
//! ```no_run
//! use jvmti2::{agent_onload, Env};
//! use std::ffi::CStr;
//!
//! fn on_load(env: &mut Env<'_>, options: Option<&CStr>) -> jvmti2::Result<()> {
//!     // agent setup here
//!     Ok(())
//! }
//!
//! agent_onload!(on_load);
//! ```

/// Generates the `Agent_OnLoad` entry point.
///
/// The handler function must have the signature:
/// `fn(&mut Env<'_>, Option<&CStr>) -> jvmti2::Result<()>`
#[macro_export]
macro_rules! agent_onload {
    ($handler:path) => {
        #[no_mangle]
        pub extern "system" fn Agent_OnLoad(
            vm: *mut jni_sys::JavaVM,
            options: *mut ::core::ffi::c_char,
            _reserved: *mut ::core::ffi::c_void,
        ) -> jni_sys::jint {
            $crate::agent::__agent_load_impl(vm, options, $handler)
        }
    };
}

/// Generates the `Agent_OnAttach` entry point (dynamic attach).
///
/// Same handler signature as [`agent_onload!`].
#[macro_export]
macro_rules! agent_onattach {
    ($handler:path) => {
        #[no_mangle]
        pub extern "system" fn Agent_OnAttach(
            vm: *mut jni_sys::JavaVM,
            options: *mut ::core::ffi::c_char,
            _reserved: *mut ::core::ffi::c_void,
        ) -> jni_sys::jint {
            $crate::agent::__agent_load_impl(vm, options, $handler)
        }
    };
}

/// Generates the `Agent_OnUnload` entry point.
///
/// The handler function must have the signature: `fn()`
#[macro_export]
macro_rules! agent_onunload {
    ($handler:path) => {
        #[no_mangle]
        pub extern "system" fn Agent_OnUnload(_vm: *mut jni_sys::JavaVM) {
            $handler();
        }
    };
}

/// Internal implementation shared by `Agent_OnLoad` and `Agent_OnAttach`.
///
/// Obtains a JVMTI environment from the JVM, wraps it in `Env`, and calls
/// the user's handler.
#[doc(hidden)]
pub fn __agent_load_impl(
    vm: *mut jni_sys::JavaVM,
    options: *mut core::ffi::c_char,
    handler: fn(&mut crate::Env<'_>, Option<&core::ffi::CStr>) -> crate::Result<()>,
) -> jni_sys::jint {
    use core::ffi::CStr;

    let options = if options.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(options) })
    };

    // Request JVMTI env from the VM.
    let mut jvmti_env: *mut crate::sys::jvmtiEnv = core::ptr::null_mut();
    let res = unsafe {
        ((**vm).v1_2.GetEnv)(
            vm,
            &mut jvmti_env as *mut *mut crate::sys::jvmtiEnv as *mut *mut core::ffi::c_void,
            crate::sys::JVMTI_VERSION_1_2,
        )
    };

    if res != jni_sys::JNI_OK {
        return res;
    }

    let mut env = unsafe { crate::Env::from_raw(jvmti_env, vm) };

    match handler(&mut env, options) {
        Ok(()) => 0,
        Err(_) => jni_sys::JNI_ERR,
    }
}
