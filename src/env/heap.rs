//! Heap iteration methods.

use core::ffi::c_void;

use jni_sys::jint;

use super::Env;
use crate::{sys, JClass, JObject};

impl<'local> Env<'local> {
    /// Forces a garbage collection.
    pub fn force_garbage_collection(&self) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, ForceGarbageCollection) };
        Ok(())
    }

    /// Iterates over all objects in the heap (legacy API).
    ///
    /// # Safety
    ///
    /// The callback function pointer and user_data must remain valid for the
    /// duration of the iteration.
    pub unsafe fn iterate_over_heap(
        &self,
        object_filter: sys::jvmtiHeapObjectFilter,
        callback: sys::jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(self, v1, IterateOverHeap, object_filter, callback, user_data)
        };
        Ok(())
    }

    /// Iterates over instances of a specific class (legacy API).
    ///
    /// # Safety
    ///
    /// The callback function pointer and user_data must remain valid for the
    /// duration of the iteration.
    pub unsafe fn iterate_over_instances_of_class(
        &self,
        klass: &JClass<'_>,
        object_filter: sys::jvmtiHeapObjectFilter,
        callback: sys::jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                IterateOverInstancesOfClass,
                klass.as_raw(),
                object_filter,
                callback,
                user_data
            )
        };
        Ok(())
    }

    /// Follows references from an object (modern heap traversal API).
    ///
    /// Pass `None` for `klass` to visit objects of all classes.
    /// Pass `None` for `initial_object` to follow references from all GC roots.
    ///
    /// # Safety
    ///
    /// The callbacks and user_data must remain valid for the duration of the
    /// traversal.
    pub unsafe fn follow_references(
        &self,
        heap_filter: jint,
        klass: Option<&JClass<'_>>,
        initial_object: Option<&JObject<'_>>,
        callbacks: &sys::jvmtiHeapCallbacks,
        user_data: *const c_void,
    ) -> crate::Result<()> {
        let klass_raw = klass.map_or(core::ptr::null_mut(), |c| c.as_raw());
        let initial_raw = initial_object.map_or(core::ptr::null_mut(), |o| o.as_raw());
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                FollowReferences,
                heap_filter,
                klass_raw,
                initial_raw,
                callbacks as *const sys::jvmtiHeapCallbacks,
                user_data
            )
        };
        Ok(())
    }

    /// Iterates through the heap (modern API).
    ///
    /// Pass `None` for `klass` to visit objects of all classes.
    ///
    /// # Safety
    ///
    /// The callbacks and user_data must remain valid for the duration of the
    /// iteration.
    pub unsafe fn iterate_through_heap(
        &self,
        heap_filter: jint,
        klass: Option<&JClass<'_>>,
        callbacks: &sys::jvmtiHeapCallbacks,
        user_data: *const c_void,
    ) -> crate::Result<()> {
        let klass_raw = klass.map_or(core::ptr::null_mut(), |c| c.as_raw());
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                IterateThroughHeap,
                heap_filter,
                klass_raw,
                callbacks as *const sys::jvmtiHeapCallbacks,
                user_data
            )
        };
        Ok(())
    }

    /// Sets the heap sampling interval.
    pub fn set_heap_sampling_interval(&self, interval: jint) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v11, SetHeapSamplingInterval, interval) };
        Ok(())
    }
}
