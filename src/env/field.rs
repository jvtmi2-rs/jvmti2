//! Field inspection methods.

use core::ffi::c_char;

use jni_sys::{jboolean, jint};

use super::Env;
use crate::{memory::JvmtiString, JClass, JFieldID};

impl<'local> Env<'local> {
    /// Returns the name, signature, and generic signature of a field.
    ///
    /// Returns `(name, signature, generic_signature)`. The generic signature
    /// may be `None`.
    pub fn get_field_name(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<(JvmtiString, JvmtiString, Option<JvmtiString>)> {
        let mut name_ptr: *mut c_char = core::ptr::null_mut();
        let mut sig_ptr: *mut c_char = core::ptr::null_mut();
        let mut gen_ptr: *mut c_char = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetFieldName,
                klass.as_raw(),
                field.into_raw(),
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

    /// Returns the class that declared a field.
    pub fn get_field_declaring_class(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<JClass<'local>> {
        let mut declaring_class: jni_sys::jclass = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetFieldDeclaringClass,
                klass.as_raw(),
                field.into_raw(),
                &mut declaring_class
            )
        };
        Ok(unsafe { crate::objects::jclass_from_raw(declaring_class) })
    }

    /// Returns the modifiers of a field.
    pub fn get_field_modifiers(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<jint> {
        let mut modifiers: jint = 0;
        unsafe {
            jvmti_call_check!(self, v1, GetFieldModifiers, klass.as_raw(), field.into_raw(), &mut modifiers)
        };
        Ok(modifiers)
    }

    /// Returns whether a field is synthetic.
    ///
    /// # Required Capabilities
    /// - `can_get_synthetic_attribute`
    pub fn is_field_synthetic(
        &self,
        klass: &JClass<'_>,
        field: JFieldID,
    ) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe {
            jvmti_call_check!(self, v1, IsFieldSynthetic, klass.as_raw(), field.into_raw(), &mut result)
        };
        Ok(result)
    }
}
