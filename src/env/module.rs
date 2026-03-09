//! Module operations (Java 9+).

use core::ffi::CStr;

use jni::sys::jint;
use jni_sys::{jboolean, jclass, jobject};

use super::Env;
use crate::memory::JvmtiArray;

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
        class_loader: jobject,
        package_name: &CStr,
    ) -> crate::Result<Option<jobject>> {
        let mut module: jobject = core::ptr::null_mut();
        unsafe {
            jvmti_call_check!(
                self,
                v9,
                GetNamedModule,
                class_loader,
                package_name.as_ptr(),
                &mut module
            )
        };
        Ok(if module.is_null() { None } else { Some(module) })
    }

    /// Adds a reads edge from `module` to `to_module`.
    pub fn add_module_reads(
        &self,
        module: jobject,
        to_module: jobject,
    ) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v9, AddModuleReads, module, to_module) };
        Ok(())
    }

    /// Adds a module export.
    pub fn add_module_exports(
        &self,
        module: jobject,
        pkg_name: &CStr,
        to_module: jobject,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v9,
                AddModuleExports,
                module,
                pkg_name.as_ptr(),
                to_module
            )
        };
        Ok(())
    }

    /// Adds a module opens.
    pub fn add_module_opens(
        &self,
        module: jobject,
        pkg_name: &CStr,
        to_module: jobject,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v9,
                AddModuleOpens,
                module,
                pkg_name.as_ptr(),
                to_module
            )
        };
        Ok(())
    }

    /// Adds a service to a module's uses.
    pub fn add_module_uses(&self, module: jobject, service: jclass) -> crate::Result<()> {
        unsafe { jvmti_call_check!(self, v9, AddModuleUses, module, service) };
        Ok(())
    }

    /// Adds a service implementation to a module.
    pub fn add_module_provides(
        &self,
        module: jobject,
        service: jclass,
        impl_class: jclass,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(self, v9, AddModuleProvides, module, service, impl_class)
        };
        Ok(())
    }

    /// Returns whether a module can be modified.
    pub fn is_modifiable_module(&self, module: jobject) -> crate::Result<bool> {
        let mut result: jboolean = false;
        unsafe { jvmti_call_check!(self, v9, IsModifiableModule, module, &mut result) };
        Ok(result)
    }
}
