//! Breakpoint and watchpoint methods.

use super::Env;
use crate::{sys, JClass, JFieldID, JMethodID};

impl<'local> Env<'local> {
    /// Sets a breakpoint at a given location.
    ///
    /// # Required Capabilities
    /// - `can_generate_breakpoint_events`
    pub fn set_breakpoint(
        &self,
        method: JMethodID,
        location: sys::jlocation,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetBreakpoint, method.into_raw(), location) };
        Ok(())
    }

    /// Clears a breakpoint at a given location.
    ///
    /// # Required Capabilities
    /// - `can_generate_breakpoint_events`
    pub fn clear_breakpoint(
        &self,
        method: JMethodID,
        location: sys::jlocation,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ClearBreakpoint, method.into_raw(), location) };
        Ok(())
    }

    /// Sets a field access watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_access_events`
    pub fn set_field_access_watch(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetFieldAccessWatch, klass.as_raw(), field.into_raw()) };
        Ok(())
    }

    /// Clears a field access watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_access_events`
    pub fn clear_field_access_watch(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ClearFieldAccessWatch, klass.as_raw(), field.into_raw()) };
        Ok(())
    }

    /// Sets a field modification watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_modification_events`
    pub fn set_field_modification_watch(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetFieldModificationWatch, klass.as_raw(), field.into_raw()) };
        Ok(())
    }

    /// Clears a field modification watch.
    ///
    /// # Required Capabilities
    /// - `can_generate_field_modification_events`
    pub fn clear_field_modification_watch(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ClearFieldModificationWatch, klass.as_raw(), field.into_raw()) };
        Ok(())
    }
}
