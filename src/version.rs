/// JVMTI version number.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(transparent)]
pub struct JvmtiVersion {
    ver: u32,
}

impl JvmtiVersion {
    /// JVMTI Version 1.
    pub const V1: Self = JvmtiVersion {
        ver: jvmti2_sys::JVMTI_VERSION_1 as u32,
    };
    /// JVMTI Version 1.0.
    pub const V1_0: Self = JvmtiVersion {
        ver: jvmti2_sys::JVMTI_VERSION_1_0 as u32,
    };
    /// JVMTI Version 1.1.
    pub const V1_1: Self = JvmtiVersion {
        ver: jvmti2_sys::JVMTI_VERSION_1_1 as u32,
    };
    /// JVMTI Version 1.2.
    pub const V1_2: Self = JvmtiVersion {
        ver: jvmti2_sys::JVMTI_VERSION_1_2 as u32,
    };

    /// Creates a version from a raw version constant.
    pub fn new(ver: jni_sys::jint) -> Self {
        Self::from(ver)
    }

    /// Returns the major component of the version number.
    pub fn major(&self) -> u16 {
        ((self.ver & 0x00ff0000) >> 16) as u16
    }

    /// Returns the minor component of the version number.
    pub fn minor(&self) -> u16 {
        (self.ver & 0xff) as u16
    }
}

impl From<jni_sys::jint> for JvmtiVersion {
    fn from(value: jni_sys::jint) -> Self {
        Self { ver: value as u32 }
    }
}

impl From<JvmtiVersion> for jni_sys::jint {
    fn from(val: JvmtiVersion) -> Self {
        val.ver as i32
    }
}
