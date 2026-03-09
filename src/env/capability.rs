//! Capability management methods on Env.

use super::Env;
use crate::{capabilities::Capabilities, sys};

impl<'local> Env<'local> {
    /// Returns the current capabilities.
    pub fn get_capabilities(&self) -> crate::Result<Capabilities> {
        let mut caps = sys::jvmtiCapabilities::empty();
        unsafe { jvmti_call_check!(self, v1, GetCapabilities, &mut caps) };
        Ok(Capabilities::from_raw(caps))
    }

    /// Returns the potentially available capabilities.
    pub fn get_potential_capabilities(&self) -> crate::Result<Capabilities> {
        let mut caps = sys::jvmtiCapabilities::empty();
        unsafe { jvmti_call_check!(self, v1, GetPotentialCapabilities, &mut caps) };
        Ok(Capabilities::from_raw(caps))
    }

    /// Adds capabilities to this environment.
    pub fn add_capabilities(&self, caps: &Capabilities) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, AddCapabilities, caps.as_raw()) };
        Ok(())
    }

    /// Relinquishes previously added capabilities.
    pub fn relinquish_capabilities(&self, caps: &Capabilities) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, RelinquishCapabilities, caps.as_raw()) };
        Ok(())
    }
}
