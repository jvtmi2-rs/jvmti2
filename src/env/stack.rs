//! Stack trace methods.

use jni::sys::jint;
use jni_sys::{jmethodID, jobject};

use super::Env;
use crate::{memory::JvmtiArray, objects::JThread, sys};

impl<'local> Env<'local> {
    /// Returns the number of frames on a thread's stack.
    ///
    /// Pass `None` for `thread` to query the current thread.
    pub fn get_frame_count(&self, thread: Option<&JThread<'_>>) -> crate::Result<jint> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut count: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetFrameCount, thread_raw, &mut count) };
        Ok(count)
    }

    /// Returns the location of the current frame.
    ///
    /// Pass `None` for `thread` to query the current thread.
    ///
    /// Returns `(method, location)`.
    pub fn get_frame_location(
        &self,
        thread: Option<&JThread<'_>>,
        depth: jint,
    ) -> crate::Result<(jmethodID, jni_sys::jlong)> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut method: jmethodID = core::ptr::null_mut();
        let mut location: jni_sys::jlong = 0;
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetFrameLocation,
                thread_raw,
                depth,
                &mut method,
                &mut location
            )
        };
        Ok((method, location))
    }

    /// Returns a thread's stack trace.
    ///
    /// Pass `None` for `thread` to query the current thread.
    ///
    /// `start_depth` is the depth to start from (0 = current frame, negative
    /// counts from bottom). `max_frame_count` is the maximum frames to return.
    pub fn get_stack_trace(
        &self,
        thread: Option<&JThread<'_>>,
        start_depth: jint,
        max_frame_count: jint,
    ) -> crate::Result<Vec<sys::jvmtiFrameInfo>> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        let mut frames = vec![
            sys::jvmtiFrameInfo {
                method: core::ptr::null_mut(),
                location: 0,
            };
            max_frame_count as usize
        ];
        let mut count: jint = 0;
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetStackTrace,
                thread_raw,
                start_depth,
                max_frame_count,
                frames.as_mut_ptr(),
                &mut count
            )
        };
        frames.truncate(count as usize);
        Ok(frames)
    }

    /// Returns stack traces for all live threads.
    ///
    /// Returns the raw `jvmtiStackInfo` array — the caller is responsible
    /// for deallocating it (via the returned `JvmtiArray`).
    pub fn get_all_stack_traces(
        &self,
        max_frame_count: jint,
    ) -> crate::Result<JvmtiArray<sys::jvmtiStackInfo>> {
        let mut stack_info_ptr: *mut sys::jvmtiStackInfo = core::ptr::null_mut();
        let mut thread_count: jint = 0;
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetAllStackTraces,
                max_frame_count,
                &mut stack_info_ptr,
                &mut thread_count
            )
        };
        Ok(unsafe { JvmtiArray::new(stack_info_ptr, thread_count, self.raw) })
    }

    /// Returns stack traces for a list of threads.
    pub fn get_thread_list_stack_traces(
        &self,
        threads: &[jobject],
        max_frame_count: jint,
    ) -> crate::Result<JvmtiArray<sys::jvmtiStackInfo>> {
        let mut stack_info_ptr: *mut sys::jvmtiStackInfo = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetThreadListStackTraces,
                threads.len() as jint,
                threads.as_ptr(),
                max_frame_count,
                &mut stack_info_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(stack_info_ptr, threads.len() as jint, self.raw) })
    }

    /// Registers for notification when a frame is popped.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_generate_frame_pop_events`
    pub fn notify_frame_pop(&self, thread: Option<&JThread<'_>>, depth: jint) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe { jvmti_call_check!(self, v1, NotifyFramePop, thread_raw, depth) };
        Ok(())
    }

    /// Pops the current frame of a thread's stack.
    ///
    /// Pass `None` for `thread` to use the current thread.
    ///
    /// # Required Capabilities
    /// - `can_pop_frame`
    pub fn pop_frame(&self, thread: Option<&JThread<'_>>) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe { jvmti_call_check!(self, v1, PopFrame, thread_raw) };
        Ok(())
    }
}
