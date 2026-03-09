//! System, version, phase, properties, and timer methods.

use core::ffi::{c_char, CStr};

use jni::sys::{jint, jlong};

use super::Env;
use crate::{
    enums::{JLocationFormat, Phase, VerboseFlag},
    memory::JvmtiString,
    sys,
    version::JvmtiVersion,
};

impl<'local> Env<'local> {
    /// Returns the JVMTI version.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn get_version(&self) -> crate::Result<JvmtiVersion> {
        let mut version: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetVersionNumber, &mut version) };
        Ok(JvmtiVersion::from(version))
    }

    /// Returns the current phase of the VM.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn get_phase(&self) -> crate::Result<Phase> {
        let mut phase: sys::jvmtiPhase = sys::jvmtiPhase::JVMTI_PHASE_DEAD;
        unsafe { jvmti_call_check!(self, v1, GetPhase, &mut phase) };
        Phase::try_from(phase)
    }

    /// Returns the current time in nanoseconds (monotonic).
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn get_time(&self) -> crate::Result<jlong> {
        let mut nanos: jlong = 0;
        unsafe { jvmti_call_check!(self, v1, GetTime, &mut nanos) };
        Ok(nanos)
    }

    /// Returns information about the timer.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn get_timer_info(&self) -> crate::Result<sys::jvmtiTimerInfo> {
        let mut info = unsafe { core::mem::zeroed::<sys::jvmtiTimerInfo>() };
        unsafe { jvmti_call_check!(self, v1, GetTimerInfo, &mut info) };
        Ok(info)
    }

    /// Returns the list of system property keys.
    ///
    /// # Phase
    /// May be called during the **OnLoad** or **live** phase.
    pub fn get_system_properties(&self) -> crate::Result<Vec<JvmtiString>> {
        let mut count: jint = 0;
        let mut property_ptr: *mut *mut c_char = core::ptr::null_mut();
        unsafe { jvmti_call_check!(self, v1, GetSystemProperties, &mut count, &mut property_ptr) };

        let mut result = Vec::with_capacity(count as usize);
        for i in 0..count as usize {
            let ptr = unsafe { *property_ptr.add(i) };
            result.push(unsafe { JvmtiString::new(ptr, self.raw) });
        }
        // Deallocate the array of pointers itself (not the strings — those are owned by JvmtiString).
        unsafe {
            let interface = *self.raw;
            ((*interface).v1.Deallocate)(self.raw, property_ptr as *mut u8);
        }
        Ok(result)
    }

    /// Returns the value of a system property.
    ///
    /// # Phase
    /// May be called during the **OnLoad** or **live** phase.
    pub fn get_system_property(&self, property: &CStr) -> crate::Result<JvmtiString> {
        let mut value_ptr: *mut c_char = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetSystemProperty,
                property.as_ptr(),
                &mut value_ptr
            )
        };
        Ok(unsafe { JvmtiString::new(value_ptr, self.raw) })
    }

    /// Sets the value of a system property.
    ///
    /// # Phase
    /// May only be called during the `OnLoad` phase.
    pub fn set_system_property(
        &self,
        property: &CStr,
        value: Option<&CStr>,
    ) -> crate::Result<()> {
        let value_ptr = value.map_or(core::ptr::null(), |v| v.as_ptr());
        unsafe { jvmti_call_check!(self, v1, SetSystemProperty, property.as_ptr(), value_ptr) };
        Ok(())
    }

    /// Returns the number of processors available to the JVM.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn get_available_processors(&self) -> crate::Result<jint> {
        let mut count: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetAvailableProcessors, &mut count) };
        Ok(count)
    }

    /// Sets a verbose flag.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn set_verbose_flag(&self, flag: VerboseFlag, value: bool) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                SetVerboseFlag,
                sys::jvmtiVerboseFlag::from(flag),
                value
            )
        };
        Ok(())
    }

    /// Returns the error name for a given error code.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn get_error_name(&self, error: sys::jvmtiError) -> crate::Result<JvmtiString> {
        let mut name_ptr: *mut c_char = core::ptr::null_mut();
        unsafe { jvmti_call_check!(self, v1, GetErrorName, error, &mut name_ptr) };
        Ok(unsafe { JvmtiString::new(name_ptr, self.raw) })
    }

    /// Returns the JLocation format.
    pub fn get_jlocation_format(&self) -> crate::Result<JLocationFormat> {
        let mut format: sys::jvmtiJlocationFormat =
            sys::jvmtiJlocationFormat::JVMTI_JLOCATION_OTHER;
        unsafe { jvmti_call_check!(self, v1, GetJLocationFormat, &mut format) };
        JLocationFormat::try_from(format)
    }

    /// Disposes of this JVMTI environment.
    ///
    /// After this call, the environment is no longer valid. Normally you would
    /// let the `Env` drop instead.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn dispose_environment(self) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, DisposeEnvironment) };
        core::mem::forget(self); // don't run Drop
        Ok(())
    }

    /// Returns the environment-local storage pointer.
    ///
    /// # Phase
    /// May be called during any phase.
    pub fn get_environment_local_storage(&self) -> crate::Result<*mut core::ffi::c_void> {
        let mut data: *mut core::ffi::c_void = core::ptr::null_mut();
        unsafe { jvmti_call_check!(self, v1, GetEnvironmentLocalStorage, &mut data) };
        Ok(data)
    }

    /// Sets the environment-local storage pointer.
    ///
    /// # Phase
    /// May be called during any phase.
    ///
    /// # Safety
    ///
    /// The caller must ensure the data pointer remains valid for the lifetime
    /// of the JVMTI environment or until replaced.
    pub unsafe fn set_environment_local_storage(
        &self,
        data: *const core::ffi::c_void,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetEnvironmentLocalStorage, data) };
        Ok(())
    }

    /// Returns the CPU timer info for the current thread.
    ///
    /// # Required Capabilities
    /// - `can_get_current_thread_cpu_time`
    pub fn get_current_thread_cpu_timer_info(&self) -> crate::Result<sys::jvmtiTimerInfo> {
        let mut info = unsafe { core::mem::zeroed::<sys::jvmtiTimerInfo>() };
        unsafe { jvmti_call_check!(self, v1, GetCurrentThreadCpuTimerInfo, &mut info) };
        Ok(info)
    }

    /// Returns CPU time for the current thread in nanoseconds.
    ///
    /// # Required Capabilities
    /// - `can_get_current_thread_cpu_time`
    pub fn get_current_thread_cpu_time(&self) -> crate::Result<jlong> {
        let mut nanos: jlong = 0;
        unsafe { jvmti_call_check!(self, v1, GetCurrentThreadCpuTime, &mut nanos) };
        Ok(nanos)
    }

    /// Returns the CPU timer info for threads.
    ///
    /// # Required Capabilities
    /// - `can_get_thread_cpu_time`
    pub fn get_thread_cpu_timer_info(&self) -> crate::Result<sys::jvmtiTimerInfo> {
        let mut info = unsafe { core::mem::zeroed::<sys::jvmtiTimerInfo>() };
        unsafe { jvmti_call_check!(self, v1, GetThreadCpuTimerInfo, &mut info) };
        Ok(info)
    }

    /// Returns CPU time for a thread in nanoseconds.
    ///
    /// # Required Capabilities
    /// - `can_get_thread_cpu_time`
    pub fn get_thread_cpu_time(&self, thread: &crate::JThread<'_>) -> crate::Result<jlong> {
        let mut nanos: jlong = 0;
        unsafe { jvmti_call_check!(self, v1, GetThreadCpuTime, thread.as_raw(), &mut nanos) };
        Ok(nanos)
    }
}
