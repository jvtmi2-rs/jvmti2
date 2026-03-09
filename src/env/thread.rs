//! Thread management methods.

use core::ffi::c_void;

use jni::sys::jint;
use jni_sys::jobject;

use super::Env;
use crate::{
    flags::ThreadState,
    memory::JvmtiArray,
    objects::{JThread, JThreadGroup},
    sys,
};

impl<'local> Env<'local> {
    /// Returns all live threads.
    ///
    /// # Required Capabilities
    /// None.
    pub fn get_all_threads(&self) -> crate::Result<JvmtiArray<jobject>> {
        let mut count: jint = 0;
        let mut threads_ptr: *mut jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, GetAllThreads, &mut count, &mut threads_ptr)
        };
        Ok(unsafe { JvmtiArray::new(threads_ptr, count, self.raw) })
    }

    /// Returns the current thread.
    ///
    /// # Phase
    /// May be called during the start or live phase.
    pub fn get_current_thread(&self) -> crate::Result<JThread<'local>> {
        let mut thread: jobject = core::ptr::null_mut();
        unsafe { jvmti_call_check!(self, v1_1, GetCurrentThread, &mut thread) };
        Ok(unsafe { JThread::from_raw(thread) })
    }

    /// Returns the state of a thread.
    ///
    /// Pass `None` for `thread` to query the current thread.
    pub fn get_thread_state(&self, thread: Option<&JThread<'_>>) -> crate::Result<ThreadState> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut state: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetThreadState, thread_raw, &mut state) };
        Ok(ThreadState::from_bits_truncate(state as u32))
    }

    /// Suspends a thread.
    ///
    /// # Required Capabilities
    /// - `can_suspend`
    pub fn suspend_thread(&self, thread: &JThread<'_>) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SuspendThread, thread.as_raw()) };
        Ok(())
    }

    /// Resumes a suspended thread.
    ///
    /// # Required Capabilities
    /// - `can_suspend`
    pub fn resume_thread(&self, thread: &JThread<'_>) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ResumeThread, thread.as_raw()) };
        Ok(())
    }

    /// Sends an asynchronous exception to a thread.
    ///
    /// # Required Capabilities
    /// - `can_signal_thread`
    pub fn stop_thread(
        &self,
        thread: &JThread<'_>,
        exception: jobject,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, StopThread, thread.as_raw(), exception) };
        Ok(())
    }

    /// Interrupts a thread.
    ///
    /// # Required Capabilities
    /// - `can_signal_thread`
    pub fn interrupt_thread(&self, thread: &JThread<'_>) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, InterruptThread, thread.as_raw()) };
        Ok(())
    }

    /// Returns information about a thread.
    ///
    /// Pass `None` for `thread` to query the current thread.
    ///
    /// The caller receives a raw `jvmtiThreadInfo` — allocated strings must be
    /// deallocated manually or wrapped in [`JvmtiString`](crate::JvmtiString).
    pub fn get_thread_info_raw(
        &self,
        thread: Option<&JThread<'_>>,
    ) -> crate::Result<sys::jvmtiThreadInfo> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut info = unsafe { core::mem::zeroed::<sys::jvmtiThreadInfo>() };
        unsafe { jvmti_call_check!(self, v1, GetThreadInfo, thread_raw, &mut info) };
        Ok(info)
    }

    /// Returns the monitors owned by a thread.
    ///
    /// Pass `None` for `thread` to query the current thread.
    ///
    /// # Required Capabilities
    /// - `can_get_owned_monitor_info`
    pub fn get_owned_monitor_info(
        &self,
        thread: Option<&JThread<'_>>,
    ) -> crate::Result<JvmtiArray<jobject>> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut count: jint = 0;
        let mut monitors_ptr: *mut jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetOwnedMonitorInfo,
                thread_raw,
                &mut count,
                &mut monitors_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(monitors_ptr, count, self.raw) })
    }

    /// Returns the monitor a thread is waiting to enter, or `None` if the
    /// thread is not waiting on a monitor.
    ///
    /// Pass `None` for `thread` to query the current thread.
    ///
    /// # Required Capabilities
    /// - `can_get_current_contended_monitor`
    pub fn get_current_contended_monitor(
        &self,
        thread: Option<&JThread<'_>>,
    ) -> crate::Result<Option<jobject>> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut monitor: jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetCurrentContendedMonitor,
                thread_raw,
                &mut monitor
            )
        };
        Ok(if monitor.is_null() { None } else { Some(monitor) })
    }

    /// Starts a new agent thread.
    ///
    /// # Safety
    ///
    /// The `proc` function pointer must be valid and the `arg` data must remain
    /// valid for the lifetime of the agent thread.
    pub unsafe fn run_agent_thread(
        &self,
        thread: &JThread<'_>,
        proc_fn: sys::jvmtiStartFunction,
        arg: *const c_void,
        priority: jint,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                RunAgentThread,
                thread.as_raw(),
                proc_fn,
                arg,
                priority
            )
        };
        Ok(())
    }

    /// Returns the top-level thread groups.
    pub fn get_top_thread_groups(&self) -> crate::Result<JvmtiArray<jobject>> {
        let mut count: jint = 0;
        let mut groups_ptr: *mut jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, GetTopThreadGroups, &mut count, &mut groups_ptr)
        };
        Ok(unsafe { JvmtiArray::new(groups_ptr, count, self.raw) })
    }

    /// Returns information about a thread group.
    pub fn get_thread_group_info_raw(
        &self,
        group: &JThreadGroup<'_>,
    ) -> crate::Result<sys::jvmtiThreadGroupInfo> {
        let mut info = unsafe { core::mem::zeroed::<sys::jvmtiThreadGroupInfo>() };
        unsafe {
            jvmti_call_check!(self, v1, GetThreadGroupInfo, group.as_raw(), &mut info)
        };
        Ok(info)
    }

    /// Returns the children of a thread group.
    ///
    /// Returns `(threads, groups)`.
    pub fn get_thread_group_children(
        &self,
        group: &JThreadGroup<'_>,
    ) -> crate::Result<(JvmtiArray<jobject>, JvmtiArray<jobject>)> {
        let mut thread_count: jint = 0;
        let mut threads_ptr: *mut jobject = core::ptr::null_mut();
        let mut group_count: jint = 0;
        let mut groups_ptr: *mut jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetThreadGroupChildren,
                group.as_raw(),
                &mut thread_count,
                &mut threads_ptr,
                &mut group_count,
                &mut groups_ptr
            )
        };
        Ok((
            unsafe { JvmtiArray::new(threads_ptr, thread_count, self.raw) },
            unsafe { JvmtiArray::new(groups_ptr, group_count, self.raw) },
        ))
    }

    /// Returns the thread-local storage for a thread.
    ///
    /// Pass `None` for `thread` to query the current thread.
    pub fn get_thread_local_storage(
        &self,
        thread: Option<&JThread<'_>>,
    ) -> crate::Result<*mut c_void> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut data: *mut c_void = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetThreadLocalStorage,
                thread_raw,
                &mut data
            )
        };
        Ok(data)
    }

    /// Sets the thread-local storage for a thread.
    ///
    /// Pass `None` for `thread` to set storage for the current thread.
    ///
    /// # Safety
    ///
    /// The data pointer must remain valid until replaced or the thread ends.
    pub unsafe fn set_thread_local_storage(
        &self,
        thread: Option<&JThread<'_>>,
        data: *const c_void,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(self, v1, SetThreadLocalStorage, thread_raw, data)
        };
        Ok(())
    }

    /// Suspends a list of threads. Returns per-thread error codes.
    ///
    /// # Required Capabilities
    /// - `can_suspend`
    pub fn suspend_thread_list(
        &self,
        threads: &[jobject],
    ) -> crate::Result<Vec<sys::jvmtiError>> {
        let count = threads.len() as jint;
        let mut results = vec![sys::jvmtiError::JVMTI_ERROR_NONE; threads.len()];
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                SuspendThreadList,
                count,
                threads.as_ptr(),
                results.as_mut_ptr()
            )
        };
        Ok(results)
    }

    /// Resumes a list of threads. Returns per-thread error codes.
    ///
    /// # Required Capabilities
    /// - `can_suspend`
    pub fn resume_thread_list(
        &self,
        threads: &[jobject],
    ) -> crate::Result<Vec<sys::jvmtiError>> {
        let count = threads.len() as jint;
        let mut results = vec![sys::jvmtiError::JVMTI_ERROR_NONE; threads.len()];
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                ResumeThreadList,
                count,
                threads.as_ptr(),
                results.as_mut_ptr()
            )
        };
        Ok(results)
    }

    /// Suspends all virtual threads except those in the exclusion list.
    ///
    /// # Required Capabilities
    /// - `can_suspend`
    /// - `can_support_virtual_threads`
    pub fn suspend_all_virtual_threads(
        &self,
        except: &[jobject],
    ) -> crate::Result<()> {
        let count = except.len() as jint;
        unsafe {
            jvmti_call_check!(
                self,
                v21,
                SuspendAllVirtualThreads,
                count,
                except.as_ptr()
            )
        };
        Ok(())
    }

    /// Resumes all virtual threads except those in the exclusion list.
    ///
    /// # Required Capabilities
    /// - `can_suspend`
    /// - `can_support_virtual_threads`
    pub fn resume_all_virtual_threads(
        &self,
        except: &[jobject],
    ) -> crate::Result<()> {
        let count = except.len() as jint;
        unsafe {
            jvmti_call_check!(
                self,
                v21,
                ResumeAllVirtualThreads,
                count,
                except.as_ptr()
            )
        };
        Ok(())
    }

    /// Returns owned monitor stack depth information.
    ///
    /// Pass `None` for `thread` to query the current thread.
    ///
    /// # Required Capabilities
    /// - `can_get_owned_monitor_stack_depth_info`
    pub fn get_owned_monitor_stack_depth_info(
        &self,
        thread: Option<&JThread<'_>>,
    ) -> crate::Result<JvmtiArray<sys::jvmtiMonitorStackDepthInfo>> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut count: jint = 0;
        let mut info_ptr: *mut sys::jvmtiMonitorStackDepthInfo = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                GetOwnedMonitorStackDepthInfo,
                thread_raw,
                &mut count,
                &mut info_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(info_ptr, count, self.raw) })
    }
}
