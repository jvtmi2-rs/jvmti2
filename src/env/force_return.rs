//! Force early return methods.


use jni_sys::{jdouble, jfloat, jint, jlong};

use super::Env;
use crate::{JObject, JThread};

impl<'local> Env<'local> {
    /// Forces early return of an object from the currently executing method.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_force_early_return`
    pub fn force_early_return_object(
        &self,
        thread: Option<&JThread<'_>>,
        value: &JObject<'_>,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1_1, ForceEarlyReturnObject, thread_raw, value.as_raw())
        };
        Ok(())
    }

    /// Forces early return of an int from the currently executing method.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_force_early_return`
    pub fn force_early_return_int(
        &self,
        thread: Option<&JThread<'_>>,
        value: jint,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1_1, ForceEarlyReturnInt, thread_raw, value)
        };
        Ok(())
    }

    /// Forces early return of a long from the currently executing method.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_force_early_return`
    pub fn force_early_return_long(
        &self,
        thread: Option<&JThread<'_>>,
        value: jlong,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1_1, ForceEarlyReturnLong, thread_raw, value)
        };
        Ok(())
    }

    /// Forces early return of a float from the currently executing method.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_force_early_return`
    pub fn force_early_return_float(
        &self,
        thread: Option<&JThread<'_>>,
        value: jfloat,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1_1, ForceEarlyReturnFloat, thread_raw, value)
        };
        Ok(())
    }

    /// Forces early return of a double from the currently executing method.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_force_early_return`
    pub fn force_early_return_double(
        &self,
        thread: Option<&JThread<'_>>,
        value: jdouble,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1_1, ForceEarlyReturnDouble, thread_raw, value)
        };
        Ok(())
    }

    /// Forces early return of void from the currently executing method.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_force_early_return`
    pub fn force_early_return_void(&self, thread: Option<&JThread<'_>>) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1_1, ForceEarlyReturnVoid, thread_raw)
        };
        Ok(())
    }
}
