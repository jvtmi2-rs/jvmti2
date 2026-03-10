//! Module operations (Java 9+).

use core::ffi::CStr;

use jni_sys::{jboolean, jint, jobject};

use super::Env;
use crate::{memory::JvmtiArray, JClass, JObject};

impl<'local> Env<'local> {
    /// Returns all modules loaded in the VM.
    pub fn get_all_modules(&self) -> crate::Result<JvmtiArray<jobject>> {
        let mut count: jint = 0;
        let mut modules_ptr: *mut jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(self, v9, GetAllModules, &mut count, &mut modules_ptr)
        };
        Ok(unsafe { JvmtiArray::new(modules_ptr, count, self.raw) })
    }

    /// Returns a named module for a class loader and package, or `None` if
    /// no such module exists.
    pub fn get_named_module(
        &self,
        class_loader: &JObject<'_>,
        package_name: &CStr,
    ) -> crate::Result<Option<jobject>> {
        let mut module: jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v9,
                GetNamedModule,
                class_loader.as_raw(),
                package_name.as_ptr(),
                &mut module
            )
        };
        Ok(if module.is_null() { None } else { Some(module) })
    }

    /// Adds a reads edge from `module` to `to_module`.
    pub fn add_module_reads(
        &self,
        module: &JObject<'_>,
        to_module: &JObject<'_>,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v9, AddModuleReads, module.as_raw(), to_module.as_raw()) };
        Ok(())
    }

    /// Adds a module export.
    pub fn add_module_exports(
        &self,
        module: &JObject<'_>,
        pkg_name: &CStr,
        to_module: &JObject<'_>,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v9,
                AddModuleExports,
                module.as_raw(),
                pkg_name.as_ptr(),
                to_module.as_raw()
            )
        };
        Ok(())
    }

    /// Adds a module opens.
    pub fn add_module_opens(
        &self,
        module: &JObject<'_>,
        pkg_name: &CStr,
        to_module: &JObject<'_>,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v9,
                AddModuleOpens,
                module.as_raw(),
                pkg_name.as_ptr(),
                to_module.as_raw()
            )
        };
        Ok(())
    }

    /// Adds a service to a module's uses.
    pub fn add_module_uses(&self, module: &JObject<'_>, service: &JClass<'_>) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v9, AddModuleUses, module.as_raw(), service.as_raw()) };
        Ok(())
    }

    /// Adds a service implementation to a module.
    pub fn add_module_provides(
        &self,
        module: &JObject<'_>,
        service: &JClass<'_>,
        impl_class: &JClass<'_>,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(self, v9, AddModuleProvides, module.as_raw(), service.as_raw(), impl_class.as_raw())
        };
        Ok(())
    }

    /// Returns whether a module can be modified.
    pub fn is_modifiable_module(&self, module: &JObject<'_>) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v9, IsModifiableModule, module.as_raw(), &mut result) };
        Ok(result)
    }
}
