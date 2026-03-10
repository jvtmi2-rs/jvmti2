//! RAII wrappers for JVMTI-allocated memory.
//!
//! The JVM allocates memory through `Allocate` and expects it to be freed
//! through `Deallocate`. These wrappers ensure proper cleanup on drop.

use core::ffi::{c_char, c_uchar, CStr};
use std::marker::PhantomData;

use jni_sys::jint;

use crate::sys;

/// Deallocates JVMTI-allocated memory using the env's function table.
///
/// # Safety
///
/// `ptr` must have been allocated by the JVMTI `Allocate` function on `env`.
unsafe fn deallocate(env: *mut sys::jvmtiEnv, ptr: *mut c_uchar) {
    let interface = unsafe { *env };
    unsafe { ((*interface).v1.Deallocate)(env, ptr) };
}

/// RAII wrapper for a JVMTI-allocated C string.
///
/// Calls `Deallocate` on drop. Provides access to the string as a `&CStr`.
#[derive(Debug)]
pub struct JvmtiString {
    ptr: *mut c_char,
    env_raw: *mut sys::jvmtiEnv,
    _not_send_sync: PhantomData<*mut ()>,
}

impl JvmtiString {
    /// Creates a new `JvmtiString` from a raw JVMTI-allocated char pointer.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid, non-null, nul-terminated C string allocated by
    ///   the JVMTI `Allocate` function.
    /// - `env_raw` must be a valid JVMTI environment pointer.
    pub(crate) unsafe fn new(ptr: *mut c_char, env_raw: *mut sys::jvmtiEnv) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert!(!env_raw.is_null());
        Self {
            ptr,
            env_raw,
            _not_send_sync: PhantomData,
        }
    }

    /// Returns the string as a `&CStr`.
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr) }
    }

    /// Converts to a lossy `String`.
    pub fn to_string_lossy(&self) -> String {
        self.as_cstr().to_string_lossy().into_owned()
    }
}

impl Drop for JvmtiString {
    fn drop(&mut self) {
        unsafe { deallocate(self.env_raw, self.ptr as *mut c_uchar) };
    }
}

impl std::fmt::Display for JvmtiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

/// RAII wrapper for a JVMTI-allocated array.
///
/// Calls `Deallocate` on drop. Provides access to the data as `&[T]`.
#[derive(Debug)]
pub struct JvmtiArray<T: Copy> {
    ptr: *mut T,
    len: usize,
    env_raw: *mut sys::jvmtiEnv,
    _not_send_sync: PhantomData<*mut ()>,
}

impl<T: Copy> JvmtiArray<T> {
    /// Creates a new `JvmtiArray` from a raw JVMTI-allocated pointer and count.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid pointer to `len` elements of type `T`, allocated
    ///   by the JVMTI `Allocate` function.
    /// - `env_raw` must be a valid JVMTI environment pointer.
    pub(crate) unsafe fn new(ptr: *mut T, len: jint, env_raw: *mut sys::jvmtiEnv) -> Self {
        debug_assert!(!ptr.is_null() || len == 0);
        debug_assert!(!env_raw.is_null());
        Self {
            ptr,
            len: len as usize,
            env_raw,
            _not_send_sync: PhantomData,
        }
    }

    /// Returns the contents as a slice.
    pub fn as_slice(&self) -> &[T] {
        if self.ptr.is_null() || self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Copy> core::ops::Deref for JvmtiArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Copy> Drop for JvmtiArray<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { deallocate(self.env_raw, self.ptr as *mut c_uchar) };
        }
    }
}
