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
pub use jni::ids::{JFieldID, JMethodID};
pub use jni::JValue;

/// Creates a [`JObject`] from a raw `jobject` without needing `&jni::Env`.
///
/// In jni 0.22, [`JObject::from_raw`] requires `&Env` for lifetime tracking,
/// but `JObject` is `#[repr(transparent)]` over `jobject` + `PhantomData` so
/// the env is never actually read.  JVMTI callbacks receive raw pointers
/// without a `jni::Env`, making this helper necessary.
///
/// # Safety
///
/// Same requirements as [`JObject::from_raw`]: `raw` must be a valid local
/// reference (or null) that will not outlive `'a`.
pub unsafe fn jobject_from_raw<'a>(raw: jni_sys::jobject) -> jni::objects::JObject<'a> {
    // Safety: JObject is #[repr(transparent)] over jobject + PhantomData<&'a ()>.
    std::mem::transmute(raw)
}

/// Creates a [`JClass`] from a raw `jclass` without needing `&jni::Env`.
///
/// See [`jobject_from_raw`] for rationale.
///
/// # Safety
///
/// Same requirements as [`JClass::from_raw`].
pub unsafe fn jclass_from_raw<'a>(raw: jni_sys::jclass) -> jni::objects::JClass<'a> {
    // Safety: JClass is #[repr(transparent)] over JObject which is
    // #[repr(transparent)] over jclass + PhantomData.
    std::mem::transmute(raw)
}
