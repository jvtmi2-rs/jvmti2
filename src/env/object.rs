//! Object operations methods.

use jni_sys::{jint, jlong, jobject};

use super::Env;
use crate::{memory::JvmtiArray, sys, JObject};

impl<'local> Env<'local> {
    /// Returns the tag associated with an object.
    ///
    /// # Required Capabilities
    /// - `can_tag_objects`
    pub fn get_tag(&self, object: &JObject<'_>) -> crate::Result<jlong> {
        let mut tag: jlong = 0;
        unsafe { jvmti_call_check!(self, v1, GetTag, object.as_raw(), &mut tag) };
        Ok(tag)
    }

    /// Sets the tag associated with an object.
    ///
    /// # Required Capabilities
    /// - `can_tag_objects`
    pub fn set_tag(&self, object: &JObject<'_>, tag: jlong) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v1, SetTag, object.as_raw(), tag) };
        Ok(())
    }

    /// Returns the hash code of an object.
    pub fn get_object_hash_code(&self, object: &JObject<'_>) -> crate::Result<jint> {
        let mut hash: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetObjectHashCode, object.as_raw(), &mut hash) };
        Ok(hash)
    }

    /// Returns the size of an object in bytes.
    pub fn get_object_size(&self, object: &JObject<'_>) -> crate::Result<jlong> {
        let mut size: jlong = 0;
        unsafe { jvmti_call_check!(self, v1, GetObjectSize, object.as_raw(), &mut size) };
        Ok(size)
    }

    /// Returns the monitor usage information for an object.
    ///
    /// # Required Capabilities
    /// - `can_get_monitor_info`
    pub fn get_object_monitor_usage(
        &self,
        object: &JObject<'_>,
    ) -> crate::Result<sys::jvmtiMonitorUsage> {
        let mut usage = unsafe { core::mem::zeroed::<sys::jvmtiMonitorUsage>() };
        unsafe { jvmti_call_check!(self, v1, GetObjectMonitorUsage, object.as_raw(), &mut usage) };
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
