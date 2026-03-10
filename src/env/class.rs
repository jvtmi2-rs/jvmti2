//! Class inspection methods.

use core::ffi::{c_char, c_uchar};

use jni_sys::{jboolean, jclass, jfieldID, jint, jmethodID, jobject};

use super::Env;
use crate::{
    flags::ClassStatus,
    memory::{JvmtiArray, JvmtiString},
    sys, JClass, JObject,
};

impl<'local> Env<'local> {
    /// Returns the JNI class signature and generic signature of a class.
    ///
    /// Returns `(signature, generic_signature)`. The generic signature may be
    /// `None` if the class has no generic signature.
    pub fn get_class_signature(
        &self,
        klass: &JClass<'_>,
    ) -> crate::Result<(JvmtiString, Option<JvmtiString>)> {
        let mut sig_ptr: *mut c_char = core::ptr::null_mut();
        let mut gen_ptr: *mut c_char = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetClassSignature,
                klass.as_raw(),
                &mut sig_ptr,
                &mut gen_ptr
            )
        };
        let sig = unsafe { JvmtiString::new(sig_ptr, self.raw) };
        let gen = if gen_ptr.is_null() {
            None
        } else {
            Some(unsafe { JvmtiString::new(gen_ptr, self.raw) })
        };
        Ok((sig, gen))
    }

    /// Returns the status of a class.
    pub fn get_class_status(&self, klass: &JClass<'_>) -> crate::Result<ClassStatus> {
        let mut status: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetClassStatus, klass.as_raw(), &mut status) };
        Ok(ClassStatus::from_bits_truncate(status as u32))
    }

    /// Returns the source file name of a class.
    ///
    /// # Required Capabilities
    /// - `can_get_source_file_name`
    pub fn get_source_file_name(&self, klass: &JClass<'_>) -> crate::Result<JvmtiString> {
        let mut name_ptr: *mut c_char = core::ptr::null_mut();
        unsafe { jvmti_call_check!(self, v1, GetSourceFileName, klass.as_raw(), &mut name_ptr) };
        Ok(unsafe { JvmtiString::new(name_ptr, self.raw) })
    }

    /// Returns the modifiers of a class (as a bitmask of JVM access flags).
    pub fn get_class_modifiers(&self, klass: &JClass<'_>) -> crate::Result<jint> {
        let mut modifiers: jint = 0;
        unsafe { jvmti_call_check!(self, v1, GetClassModifiers, klass.as_raw(), &mut modifiers) };
        Ok(modifiers)
    }

    /// Returns the methods declared in a class.
    pub fn get_class_methods(&self, klass: &JClass<'_>) -> crate::Result<JvmtiArray<jmethodID>> {
        let mut count: jint = 0;
        let mut methods_ptr: *mut jmethodID = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, GetClassMethods, klass.as_raw(), &mut count, &mut methods_ptr)
        };
        Ok(unsafe { JvmtiArray::new(methods_ptr, count, self.raw) })
    }

    /// Returns the fields declared in a class.
    pub fn get_class_fields(&self, klass: &JClass<'_>) -> crate::Result<JvmtiArray<jfieldID>> {
        let mut count: jint = 0;
        let mut fields_ptr: *mut jfieldID = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, GetClassFields, klass.as_raw(), &mut count, &mut fields_ptr)
        };
        Ok(unsafe { JvmtiArray::new(fields_ptr, count, self.raw) })
    }

    /// Returns the interfaces directly implemented by a class.
    pub fn get_implemented_interfaces(
        &self,
        klass: &JClass<'_>,
    ) -> crate::Result<JvmtiArray<jclass>> {
        let mut count: jint = 0;
        let mut interfaces_ptr: *mut jclass = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetImplementedInterfaces,
                klass.as_raw(),
                &mut count,
                &mut interfaces_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(interfaces_ptr, count, self.raw) })
    }

    /// Returns whether a class is an interface.
    pub fn is_interface(&self, klass: &JClass<'_>) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v1, IsInterface, klass.as_raw(), &mut result) };
        Ok(result)
    }

    /// Returns whether a class is an array class.
    pub fn is_array_class(&self, klass: &JClass<'_>) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v1, IsArrayClass, klass.as_raw(), &mut result) };
        Ok(result)
    }

    /// Returns the class loader of a class, or `None` for the bootstrap
    /// class loader.
    pub fn get_class_loader(&self, klass: &JClass<'_>) -> crate::Result<Option<jobject>> {
        let mut loader: jobject = core::ptr::null_mut();
        unsafe { jvmti_call_check!(self, v1, GetClassLoader, klass.as_raw(), &mut loader) };
        Ok(if loader.is_null() { None } else { Some(loader) })
    }

    /// Returns all classes currently loaded.
    pub fn get_loaded_classes(&self) -> crate::Result<JvmtiArray<jclass>> {
        let mut count: jint = 0;
        let mut classes_ptr: *mut jclass = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, GetLoadedClasses, &mut count, &mut classes_ptr)
        };
        Ok(unsafe { JvmtiArray::new(classes_ptr, count, self.raw) })
    }

    /// Returns all classes loaded by a specific class loader.
    pub fn get_class_loader_classes(
        &self,
        loader: &JObject<'_>,
    ) -> crate::Result<JvmtiArray<jclass>> {
        let mut count: jint = 0;
        let mut classes_ptr: *mut jclass = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GetClassLoaderClasses,
                loader.as_raw(),
                &mut count,
                &mut classes_ptr
            )
        };
        Ok(unsafe { JvmtiArray::new(classes_ptr, count, self.raw) })
    }

    /// Redefines one or more classes.
    ///
    /// # Required Capabilities
    /// - `can_redefine_classes`
    pub fn redefine_classes(
        &self,
        definitions: &[sys::jvmtiClassDefinition],
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                RedefineClasses,
                definitions.len() as jint,
                definitions.as_ptr()
            )
        };
        Ok(())
    }

    /// Retransforms one or more classes.
    ///
    /// # Required Capabilities
    /// - `can_retransform_classes`
    pub fn retransform_classes(&self, classes: &[&JClass<'_>]) -> crate::Result<()> {
        let raw_classes: Vec<jclass> = classes.iter().map(|c| c.as_raw()).collect();
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                RetransformClasses,
                raw_classes.len() as jint,
                raw_classes.as_ptr()
            )
        };
        Ok(())
    }

    /// Returns whether a class can be modified.
    pub fn is_modifiable_class(&self, klass: &JClass<'_>) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v1_1, IsModifiableClass, klass.as_raw(), &mut result) };
        Ok(result)
    }

    /// Returns the class version numbers.
    ///
    /// Returns `(minor_version, major_version)`.
    pub fn get_class_version_numbers(
        &self,
        klass: &JClass<'_>,
    ) -> crate::Result<(jint, jint)> {
        let mut minor: jint = 0;
        let mut major: jint = 0;
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                GetClassVersionNumbers,
                klass.as_raw(),
                &mut minor,
                &mut major
            )
        };
        Ok((minor, major))
    }

    /// Returns the constant pool of a class as raw bytes.
    ///
    /// Returns `(constant_pool_count, constant_pool_bytes)`.
    ///
    /// # Required Capabilities
    /// - `can_get_constant_pool`
    pub fn get_constant_pool(
        &self,
        klass: &JClass<'_>,
    ) -> crate::Result<(jint, JvmtiArray<c_uchar>)> {
        let mut cp_count: jint = 0;
        let mut byte_count: jint = 0;
        let mut bytes_ptr: *mut c_uchar = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v1_1,
                GetConstantPool,
                klass.as_raw(),
                &mut cp_count,
                &mut byte_count,
                &mut bytes_ptr
            )
        };
        Ok((cp_count, unsafe {
            JvmtiArray::new(bytes_ptr, byte_count, self.raw)
        }))
    }

    /// Returns the source debug extension of a class.
    ///
    /// # Required Capabilities
    /// - `can_get_source_debug_extension`
    pub fn get_source_debug_extension(&self, klass: &JClass<'_>) -> crate::Result<JvmtiString> {
        let mut ptr: *mut c_char = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v1, GetSourceDebugExtension, klass.as_raw(), &mut ptr)
        };
        Ok(unsafe { JvmtiString::new(ptr, self.raw) })
    }
}
