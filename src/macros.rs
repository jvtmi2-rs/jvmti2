/// Directly calls a JVMTI function through the interface pointer table.
///
/// # Safety
///
/// When calling any function added after JVMTI 1.0 you must know that it's
/// valid for the current JVMTI version.
macro_rules! jvmti_call_unchecked {
    ( $env:expr, $version:tt, $name:tt $(, $args:expr )*) => {{
        let env: *mut jvmti2_sys::jvmtiEnv = $env.as_raw();
        let interface: *const jvmti2_sys::jvmtiInterface_1_ = *env;
        ((*interface).$version.$name)(env $(, $args)*)
    }};
}

/// Calls a JVMTI function and checks the return code for errors.
///
/// Returns `Err(JvmtiError)` if the return code is not `JVMTI_ERROR_NONE`.
macro_rules! jvmti_call_check {
    ( $env:expr, $version:tt, $name:tt $(, $args:expr )* ) => ({
        {
            let ret = jvmti_call_unchecked!($env, $version, $name $(, $args)*);
            if ret != $crate::sys::jvmtiError::JVMTI_ERROR_NONE {
                return Err($crate::errors::JvmtiError::from(ret));
            }
        }
    })
}

/// Maps a pointer to either `Ok(ptr)` or `Err(JvmtiError::NullPtr)`.
#[allow(unused_macros)]
macro_rules! null_check {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            Err($crate::errors::JvmtiError::NullPtr($ctx))
        } else {
            Ok($obj)
        }
    };
}

/// Directly calls a `JavaVM` function.
#[allow(unused_macros)]
macro_rules! java_vm_call_unchecked {
    ( $jvm:expr, $version:tt, $name:tt $(, $args:expr )*) => {{
        let jvm: *mut jni_sys::JavaVM = $jvm;
        ((*(*jvm)).$version.$name)(jvm $(, $args)*)
    }};
}
