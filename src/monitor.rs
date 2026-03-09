//! RAII wrapper for JVMTI raw monitors.

use std::marker::PhantomData;

use jni::sys::jlong;

use crate::{errors::JvmtiError, sys};

/// A JVMTI raw monitor, analogous to a `Mutex`.
///
/// Calls `DestroyRawMonitor` on drop. Created via [`Env::create_raw_monitor`](crate::Env::create_raw_monitor).
#[derive(Debug)]
pub struct RawMonitor {
    id: sys::jrawMonitorID,
    env_raw: *mut sys::jvmtiEnv,
    _not_send_sync: PhantomData<*mut ()>,
}

impl RawMonitor {
    /// Creates a new `RawMonitor` from raw parts.
    ///
    /// # Safety
    ///
    /// `id` must be a valid raw monitor created by `CreateRawMonitor`.
    pub(crate) unsafe fn new(id: sys::jrawMonitorID, env_raw: *mut sys::jvmtiEnv) -> Self {
        Self {
            id,
            env_raw,
            _not_send_sync: PhantomData,
        }
    }

    /// Returns the raw monitor ID.
    pub fn as_raw(&self) -> sys::jrawMonitorID {
        self.id
    }

    /// Enter (lock) this monitor.
    pub fn enter(&self) -> crate::Result<()> {
        let ret = unsafe {
            let interface = *self.env_raw;
            ((*interface).v1.RawMonitorEnter)(self.env_raw, self.id)
        };
        if ret != sys::jvmtiError::JVMTI_ERROR_NONE {
            return Err(JvmtiError::from(ret));
        }
        Ok(())
    }

    /// Exit (unlock) this monitor.
    pub fn exit(&self) -> crate::Result<()> {
        let ret = unsafe {
            let interface = *self.env_raw;
            ((*interface).v1.RawMonitorExit)(self.env_raw, self.id)
        };
        if ret != sys::jvmtiError::JVMTI_ERROR_NONE {
            return Err(JvmtiError::from(ret));
        }
        Ok(())
    }

    /// Wait on this monitor. Pass `0` for indefinite wait.
    pub fn wait(&self, millis: jlong) -> crate::Result<()> {
        let ret = unsafe {
            let interface = *self.env_raw;
            ((*interface).v1.RawMonitorWait)(self.env_raw, self.id, millis)
        };
        if ret != sys::jvmtiError::JVMTI_ERROR_NONE {
            return Err(JvmtiError::from(ret));
        }
        Ok(())
    }

    /// Notify a single thread waiting on this monitor.
    pub fn notify(&self) -> crate::Result<()> {
        let ret = unsafe {
            let interface = *self.env_raw;
            ((*interface).v1.RawMonitorNotify)(self.env_raw, self.id)
        };
        if ret != sys::jvmtiError::JVMTI_ERROR_NONE {
            return Err(JvmtiError::from(ret));
        }
        Ok(())
    }

    /// Notify all threads waiting on this monitor.
    pub fn notify_all(&self) -> crate::Result<()> {
        let ret = unsafe {
            let interface = *self.env_raw;
            ((*interface).v1.RawMonitorNotifyAll)(self.env_raw, self.id)
        };
        if ret != sys::jvmtiError::JVMTI_ERROR_NONE {
            return Err(JvmtiError::from(ret));
        }
        Ok(())
    }

    /// Enter the monitor and return a guard that exits on drop.
    pub fn lock(&self) -> crate::Result<RawMonitorGuard<'_>> {
        self.enter()?;
        Ok(RawMonitorGuard { monitor: self })
    }
}

impl Drop for RawMonitor {
    fn drop(&mut self) {
        unsafe {
            let interface = *self.env_raw;
            ((*interface).v1.DestroyRawMonitor)(self.env_raw, self.id);
        }
    }
}

/// A guard that exits the raw monitor when dropped, similar to `MutexGuard`.
#[derive(Debug)]
pub struct RawMonitorGuard<'a> {
    monitor: &'a RawMonitor,
}

impl Drop for RawMonitorGuard<'_> {
    fn drop(&mut self) {
        let _ = self.monitor.exit();
    }
}
