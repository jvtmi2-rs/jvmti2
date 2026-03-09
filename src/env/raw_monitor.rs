//! Raw monitor creation method on Env.

use core::ffi::CStr;

use super::Env;
use crate::{monitor::RawMonitor, sys};

impl<'local> Env<'local> {
    /// Creates a new raw monitor.
    ///
    /// The returned [`RawMonitor`] calls `DestroyRawMonitor` on drop.
    pub fn create_raw_monitor(&self, name: &CStr) -> crate::Result<RawMonitor> {
        let mut monitor_id: sys::jrawMonitorID = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, CreateRawMonitor, name.as_ptr(), &mut monitor_id)
        };
        Ok(unsafe { RawMonitor::new(monitor_id, self.raw) })
    }
}
