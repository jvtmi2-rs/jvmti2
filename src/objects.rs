//! Typed wrappers around JNI object references for JVMTI-specific types.
//!
//! These follow the jni-rs pattern of `#[repr(transparent)]` newtypes
//! with lifetime tracking. Each type stores a raw `jobject` pointer directly
//! (rather than wrapping `jni::objects::JObject`) so it can be constructed
//! in JVMTI callbacks without needing a `jni::Env`.

/// Generates a typed JVMTI wrapper around a raw `jobject`.
macro_rules! bind_jvmti_type {
    (
        $(#[$meta:meta])*
        pub struct $name:ident<'local>;
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        pub struct $name<'local> {
            raw: jni_sys::jobject,
            _lifetime: ::core::marker::PhantomData<&'local ()>,
        }

        impl ::core::fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("raw", &self.raw)
                    .finish()
            }
        }

        impl Default for $name<'_> {
            fn default() -> Self {
                Self {
                    raw: ::core::ptr::null_mut(),
                    _lifetime: ::core::marker::PhantomData,
                }
            }
        }

        impl<'local> $name<'local> {
            /// Creates a new wrapper from a raw `jobject`.
            ///
            /// # Safety
            ///
            /// The caller must guarantee that the raw pointer is valid and
            /// that it actually refers to the correct Java type.
            pub unsafe fn from_raw(raw: jni_sys::jobject) -> Self {
                Self {
                    raw,
                    _lifetime: ::core::marker::PhantomData,
                }
            }

            /// Returns the raw `jobject` pointer.
            pub fn as_raw(&self) -> jni_sys::jobject {
                self.raw
            }

            /// Consumes this wrapper and returns the raw `jobject` pointer.
            pub fn into_raw(self) -> jni_sys::jobject {
                self.raw
            }
        }
    };
}

bind_jvmti_type! {
    /// A Java thread reference (`jthread`).
    ///
    /// Wraps a `jobject` representing a `java.lang.Thread` instance.
    pub struct JThread<'local>;
}

bind_jvmti_type! {
    /// A Java thread group reference (`jthreadGroup`).
    ///
    /// Wraps a `jobject` representing a `java.lang.ThreadGroup` instance.
    pub struct JThreadGroup<'local>;
}

bind_jvmti_type! {
    /// A Java module reference.
    ///
    /// Wraps a `jobject` representing a `java.lang.Module` instance (Java 9+).
    pub struct JModule<'local>;
}

// Re-export commonly-used jni-rs types for convenience.
pub use jni::objects::{JClass, JObject, JString};
