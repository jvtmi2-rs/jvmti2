//! Object operations methods.

use jni::sys::jint;
use jni_sys::{jlong, jobject};

use super::Env;
use crate::{memory::JvmtiArray, sys};

impl<'local> Env<'local> {
    /// Returns the tag associated with an object.
    ///
    /// # Required Capabilities
    /// - `can_tag_objects`
    pub fn get_tag(&self, object: jobject) -> crate::Result<jlong> {
        let mut tag: jlong = 0;
        unsafe { jvmti_call_check!(self, v1, GetTag, object, &mut tag) };
        Ok(tag)
    }

    /// Sets the tag associated with an object.
    ///
    /// # Required Capabilities
    /// - `can_tag_objects`
    pub fn set_tag(&self, object: jobject, tag: jlong) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetTag, object, tag) };
        Ok(())
    }

    /// Returns the hash code of an object.
    pub fn get_object_hash_code(&self, object: jobject) -> crate::Result<jint> {
        let mut hash: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetObjectHashCode, object, &mut hash) };
        Ok(hash)
    }

    /// Returns the size of an object in bytes.
    pub fn get_object_size(&self, object: jobject) -> crate::Result<jlong> {
        let mut size: jlong = 0;
        unsafe { jvmti_call_check!(self, v1, GetObjectSize, object, &mut size) };
        Ok(size)
    }

    /// Returns the monitor usage information for an object.
    ///
    /// # Required Capabilities
    /// - `can_get_monitor_info`
    pub fn get_object_monitor_usage(
        &self,
        object: jobject,
    ) -> crate::Result<sys::jvmtiMonitorUsage> {
        let mut usage = unsafe { core::mem::zeroed::<sys::jvmtiMonitorUsage>() };
        unsafe { jvmti_call_check!(self, v1, GetObjectMonitorUsage, object, &mut usage) };
        Ok(usage)
    }

    /// Returns objects with specific tags.
    ///
    /// Returns `(objects, tags)` arrays.
    ///
    /// # Required Capabilities
    /// - `can_tag_objects`
    pub fn get_objects_with_tags(
        &self,
        tags: &[jlong],
    ) -> crate::Result<(JvmtiArray<jobject>, JvmtiArray<jlong>)> {
        let mut count: jint = 0;
        let mut object_result_ptr: *mut jobject = core::ptr::null_mut();
        let mut tag_result_ptr: *mut jlong = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetObjectsWithTags,
                tags.len() as jint,
                tags.as_ptr(),
                &mut count,
                &mut object_result_ptr,
                &mut tag_result_ptr
            )
        };
        Ok((
            unsafe { JvmtiArray::new(object_result_ptr, count, self.raw) },
            unsafe { JvmtiArray::new(tag_result_ptr, count, self.raw) },
        ))
    }
}
