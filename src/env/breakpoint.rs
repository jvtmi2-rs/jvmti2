//! Breakpoint and watchpoint methods.

use jni_sys::{jclass, jfieldID, jmethodID};

use super::Env;
use crate::sys;

impl<'local> Env<'local> {
    /// Sets a breakpoint at a given location.
    ///
    /// # Required Capabilities
    /// - `can_generate_breakpoint_events`
    pub fn set_breakpoint(
        &self,
        method: jmethodID,
        location: sys::jlocation,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetBreakpoint, method, location) };
        Ok(())
    }

    /// Clears a breakpoint at a given location.
    ///
    /// # Required Capabilities
    /// - `can_generate_breakpoint_events`
    pub fn clear_breakpoint(
        &self,
        method: jmethodID,
        location: sys::jlocation,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ClearBreakpoint, method, location) };
        Ok(())
    }

    /// Sets a field access watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_access_events`
    pub fn set_field_access_watch(
        &self,
        klass: jclass,
        field: jfieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetFieldAccessWatch, klass, field) };
        Ok(())
    }

    /// Clears a field access watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_access_events`
    pub fn clear_field_access_watch(
        &self,
        klass: jclass,
        field: jfieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ClearFieldAccessWatch, klass, field) };
        Ok(())
    }

    /// Sets a field modification watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_modification_events`
    pub fn set_field_modification_watch(
        &self,
        klass: jclass,
        field: jfieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetFieldModificationWatch, klass, field) };
        Ok(())
    }

    /// Clears a field modification watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_modification_events`
    pub fn clear_field_modification_watch(
        &self,
        klass: jclass,
        field: jfieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ClearFieldModificationWatch, klass, field) };
        Ok(())
    }
}
