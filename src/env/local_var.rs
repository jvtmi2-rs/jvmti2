//! Local variable access methods.

use jni_sys::{jdouble, jfloat, jint, jlong, jobject};

use super::Env;
use crate::{JObject, JThread};

impl<'local> Env<'local> {
    /// Gets a local object variable, or `None` if the value is Java `null`.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn get_local_object(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
    ) -> crate::Result<Option<jobject>> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut value: jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetLocalObject,
                thread_raw,
                depth,
                slot,
                &mut value
            )
        };
        Ok(if value.is_null() { None } else { Some(value) })
    }

    /// Gets a local int variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn get_local_int(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
    ) -> crate::Result<jint> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut value: jint = 0;
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetLocalInt,
                thread_raw,
                depth,
                slot,
                &mut value
            )
        };
        Ok(value)
    }

    /// Gets a local long variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn get_local_long(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
    ) -> crate::Result<jlong> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut value: jlong = 0;
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetLocalLong,
                thread_raw,
                depth,
                slot,
                &mut value
            )
        };
        Ok(value)
    }

    /// Gets a local float variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn get_local_float(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
    ) -> crate::Result<jfloat> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut value: jfloat = 0.0;
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetLocalFloat,
                thread_raw,
                depth,
                slot,
                &mut value
            )
        };
        Ok(value)
    }

    /// Gets a local double variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn get_local_double(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
    ) -> crate::Result<jdouble> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut value: jdouble = 0.0;
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetLocalDouble,
                thread_raw,
                depth,
                slot,
                &mut value
            )
        };
        Ok(value)
    }

    /// Sets a local object variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn set_local_object(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
        value: &JObject<'_>,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1, SetLocalObject, thread_raw, depth, slot, value.as_raw())
        };
        Ok(())
    }

    /// Sets a local int variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn set_local_int(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
        value: jint,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1, SetLocalInt, thread_raw, depth, slot, value)
        };
        Ok(())
    }

    /// Sets a local long variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn set_local_long(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
        value: jlong,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1, SetLocalLong, thread_raw, depth, slot, value)
        };
        Ok(())
    }

    /// Sets a local float variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn set_local_float(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
        value: jfloat,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1, SetLocalFloat, thread_raw, depth, slot, value)
        };
        Ok(())
    }

    /// Sets a local double variable.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn set_local_double(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
        slot: jint,
        value: jdouble,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1, SetLocalDouble, thread_raw, depth, slot, value)
        };
        Ok(())
    }

    /// Gets the `this` object of the current frame, or `None` if the frame
    /// is in a static method.
    ///
    /// Pass `None` for `thread` to use the current thread.
    pub fn get_local_instance(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
    ) -> crate::Result<Option<jobject>> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut value: jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1_2,
                GetLocalInstance,
                thread_raw,
                depth,
                &mut value
            )
        };
        Ok(if value.is_null() { None } else { Some(value) })
    }
}
