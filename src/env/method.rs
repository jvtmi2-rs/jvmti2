//! Method inspection methods.

use core::ffi::{c_char, c_uchar};

use jni::sys::jint;
use jni_sys::{jboolean, jclass, jmethodID};

use super::Env;
use crate::{memory::{JvmtiArray, JvmtiString}, sys};

impl<'local> Env<'local> {
    /// Returns the name, signature, and generic signature of a method.
    ///
    /// Returns `(name, signature, generic_signature)`. The generic signature
    /// may be `None`.
    pub fn get_method_name(
        &self,
        method: jmethodID,
    ) -> crate::Result<(JvmtiString, JvmtiString, Option<JvmtiString>)> {
        let mut name_ptr: *mut c_char = core::ptr::null_mut();
        let mut sig_ptr: *mut c_char = core::ptr::null_mut();
        let mut gen_ptr: *mut c_char = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetMethodName,
                method,
                &mut name_ptr,
                &mut sig_ptr,
                &mut gen_ptr
            )
        };
        let name = unsafe { JvmtiString::new(name_ptr, self.raw) };
        let sig = unsafe { JvmtiString::new(sig_ptr, self.raw) };
        let gen = if gen_ptr.is_null() {
            None
        } else {
            Some(unsafe { JvmtiString::new(gen_ptr, self.raw) })
        };
        Ok((name, sig, gen))
    }

    /// Returns the class that declared a method.
    pub fn get_method_declaring_class(&self, method: jmethodID) -> crate::Result<jclass> {
        let mut klass: jclass = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, GetMethodDeclaringClass, method, &mut klass)
        };
        Ok(klass)
    }

    /// Returns the modifiers of a method.
    pub fn get_method_modifiers(&self, method: jmethodID) -> crate::Result<jint> {
        let mut modifiers: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetMethodModifiers, method, &mut modifiers) };
        Ok(modifiers)
    }

    /// Returns the maximum number of local variable slots used by a method.
    pub fn get_max_locals(&self, method: jmethodID) -> crate::Result<jint> {
        let mut max: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetMaxLocals, method, &mut max) };
        Ok(max)
    }

    /// Returns the number of words used by the method's arguments.
    pub fn get_arguments_size(&self, method: jmethodID) -> crate::Result<jint> {
        let mut size: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetArgumentsSize, method, &mut size) };
        Ok(size)
    }

    /// Returns the line number table for a method.
    ///
    /// # Required Capabilities
    /// - `can_get_line_numbers`
    pub fn get_line_number_table(
        &self,
        method: jmethodID,
    ) -> crate::Result<JvmtiArray<sys::jvmtiLineNumberEntry>> {
        let mut count: jint = 0;
        let mut table_ptr: *mut sys::jvmtiLineNumberEntry = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetLineNumberTable,
                method,
                &mut count,
                &mut table_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(table_ptr, count, self.raw) })
    }

    /// Returns the start and end location of a method.
    ///
    /// Returns `(start_location, end_location)`.
    pub fn get_method_location(
        &self,
        method: jmethodID,
    ) -> crate::Result<(jni_sys::jlong, jni_sys::jlong)> {
        let mut start: jni_sys::jlong = 0;
        let mut end: jni_sys::jlong = 0;
        unsafe {
            jvmti_call_check!(self, v1, GetMethodLocation, method, &mut start, &mut end)
        };
        Ok((start, end))
    }

    /// Returns the local variable table for a method.
    ///
    /// # Required Capabilities
    /// - `can_access_local_variables`
    pub fn get_local_variable_table(
        &self,
        method: jmethodID,
    ) -> crate::Result<JvmtiArray<sys::jvmtiLocalVariableEntry>> {
        let mut count: jint = 0;
        let mut table_ptr: *mut sys::jvmtiLocalVariableEntry = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetLocalVariableTable,
                method,
                &mut count,
                &mut table_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(table_ptr, count, self.raw) })
    }

    /// Returns the bytecodes of a method.
    ///
    /// # Required Capabilities
    /// - `can_get_bytecodes`
    pub fn get_bytecodes(&self, method: jmethodID) -> crate::Result<JvmtiArray<c_uchar>> {
        let mut count: jint = 0;
        let mut bytecodes_ptr: *mut c_uchar = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetBytecodes,
                method,
                &mut count,
                &mut bytecodes_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(bytecodes_ptr, count, self.raw) })
    }

    /// Returns whether a method is native.
    pub fn is_method_native(&self, method: jmethodID) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v1, IsMethodNative, method, &mut result) };
        Ok(result)
    }

    /// Returns whether a method is synthetic.
    ///
    /// # Required Capabilities
    /// - `can_get_synthetic_attribute`
    pub fn is_method_synthetic(&self, method: jmethodID) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v1, IsMethodSynthetic, method, &mut result) };
        Ok(result)
    }

    /// Returns whether a method is obsolete (has been replaced by
    /// `RedefineClasses`).
    pub fn is_method_obsolete(&self, method: jmethodID) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v1, IsMethodObsolete, method, &mut result) };
        Ok(result)
    }
}
