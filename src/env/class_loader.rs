//! Class loader search and native method prefix methods.

use core::ffi::{c_char, CStr};

use jni_sys::{jint, JNINativeInterface_};

use super::Env;

impl<'local> Env<'local> {
    /// Adds a path segment to the bootstrap class loader search.
    ///
    /// # Phase
    /// May only be called during the `OnLoad` phase.
    pub fn add_to_bootstrap_class_loader_search(&self, segment: &CStr) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(self, v1, AddToBootstrapClassLoaderSearch, segment.as_ptr())
        };
        Ok(())
    }

    /// Adds a path segment to the system class loader search.
    ///
    /// # Phase
    /// May only be called during the `OnLoad` or `Live` phase.
    pub fn add_to_system_class_loader_search(&self, segment: &CStr) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(self, v1_1, AddToSystemClassLoaderSearch, segment.as_ptr())
        };
        Ok(())
    }

    /// Sets the native method prefix for this environment.
    ///
    /// # Required Capabilities
    /// - `can_set_native_method_prefix`
    pub fn set_native_method_prefix(&self, prefix: &CStr) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(self, v1_1, SetNativeMethodPrefix, prefix.as_ptr())
        };
        Ok(())
    }

    /// Sets the native method prefixes for this environment.
    ///
    /// # Required Capabilities
    /// - `can_set_native_method_prefix`
    ///
    /// # Safety
    ///
    /// The prefix pointers must remain valid until replaced.
    pub unsafe fn set_native_method_prefixes(
        &self,
        prefixes: &mut [*mut c_char],
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                SetNativeMethodPrefixes,
                prefixes.len() as jint,
                prefixes.as_mut_ptr()
            )
        };
        Ok(())
    }

    /// Sets the JNI function table.
    ///
    /// # Safety
    ///
    /// The function table must remain valid for the lifetime of the environment.
    pub unsafe fn set_jni_function_table(
        &self,
        function_table: &JNINativeInterface_,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                SetJNIFunctionTable,
                function_table as *const JNINativeInterface_
            )
        };
        Ok(())
    }

    /// Gets the JNI function table.
    pub fn get_jni_function_table(&self) -> crate::Result<*mut JNINativeInterface_> {
        let mut table: *mut JNINativeInterface_ = core::ptr::null_mut();
        unsafe { jvmti_call_check!(self, v1, GetJNIFunctionTable, &mut table) };
        Ok(table)
    }
}
