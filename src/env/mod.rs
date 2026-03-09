//! The JVMTI environment wrapper.
//!
//! [`Env`] is the central type — it wraps a raw `jvmtiEnv` pointer and
//! provides safe methods for all ~156 JVMTI functions.

use std::marker::PhantomData;

use crate::sys;

mod system;
mod thread;
mod stack;
mod class;
mod method;
mod field;
mod object;
mod heap;
mod local_var;
mod breakpoint;
mod force_return;
mod raw_monitor;
mod event;
mod capability;
mod module;
mod class_loader;

/// A JVMTI environment handle.
///
/// This wraps a raw `jvmtiEnv*` with a phantom lifetime `'local` to tie local
/// references to the current scope. It is intentionally `!Send` and `!Sync`
/// (JVMTI environments are thread-local).
///
/// Created via [`agent_onload!`](crate::agent_onload) or
/// [`Env::from_raw`].
#[derive(Debug)]
pub struct Env<'local> {
    raw: *mut sys::jvmtiEnv,
    vm: *mut jni_sys::JavaVM,
    _lifetime: PhantomData<&'local ()>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl Drop for Env<'_> {
    fn drop(&mut self) {
        // Intentionally empty — prevents the compiler from assuming
        // this type is FFI-safe (same pattern as jni-rs).
    }
}

impl<'local> Env<'local> {
    /// Creates an `Env` from a raw JVMTI environment pointer.
    ///
    /// # Safety
    ///
    /// - `raw` must be a valid, non-null `jvmtiEnv` pointer.
    /// - `vm` must be a valid, non-null `JavaVM` pointer.
    /// - The caller must ensure the environment outlives `'local`.
    pub unsafe fn from_raw(raw: *mut sys::jvmtiEnv, vm: *mut jni_sys::JavaVM) -> Self {
        debug_assert!(!raw.is_null(), "jvmtiEnv pointer must not be null");
        debug_assert!(!vm.is_null(), "JavaVM pointer must not be null");
        Self {
            raw,
            vm,
            _lifetime: PhantomData,
            _not_send_sync: PhantomData,
        }
    }

    /// Returns the raw `jvmtiEnv` pointer.
    pub fn as_raw(&self) -> *mut sys::jvmtiEnv {
        self.raw
    }

    /// Returns the raw `JavaVM` pointer.
    pub fn as_java_vm_raw(&self) -> *mut jni_sys::JavaVM {
        self.vm
    }

    /// Returns a [`jni::JavaVM`] handle.
    ///
    /// In jni 0.22, thread attachment uses scoped callbacks:
    /// ```no_run
    /// # use jvmti2::Env;
    /// # fn example(env: &Env<'_>) {
    /// let vm = env.get_java_vm();
    /// vm.attach_current_thread(|jni_env| {
    ///     // use jni_env here
    ///     Ok(())
    /// }).unwrap();
    /// # }
    /// ```
    pub fn get_java_vm(&self) -> jni::JavaVM {
        // JavaVM::from_raw returns Self directly in jni 0.22.
        unsafe { jni::JavaVM::from_raw(self.vm) }
    }
}
