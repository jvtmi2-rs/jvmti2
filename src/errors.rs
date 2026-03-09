use crate::sys;
use thiserror::Error;

/// The primary result type for all JVMTI operations.
pub type Result<T> = core::result::Result<T, JvmtiError>;

/// The primary error type for all JVMTI operations.
///
/// Each variant in the "JVMTI spec errors" section maps directly to a JVMTI
/// error code from the specification. The "Library-level errors" section
/// contains errors that originate from this crate rather than from the JVM.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JvmtiError {
    // ── JVMTI spec errors ────────────────────────────────────────────

    /// The passed thread is not a valid thread.
    #[error("invalid thread")]
    InvalidThread,
    /// The passed thread group is not a valid thread group.
    #[error("invalid thread group")]
    InvalidThreadGroup,
    /// Invalid priority.
    #[error("invalid priority")]
    InvalidPriority,
    /// Thread was not suspended.
    #[error("thread not suspended")]
    ThreadNotSuspended,
    /// Thread already suspended.
    #[error("thread suspended")]
    ThreadSuspended,
    /// This operation requires the thread to be alive.
    #[error("thread not alive")]
    ThreadNotAlive,
    /// Invalid object.
    #[error("invalid object")]
    InvalidObject,
    /// Invalid class.
    #[error("invalid class")]
    InvalidClass,
    /// Class has been loaded but not yet prepared.
    #[error("class not prepared")]
    ClassNotPrepared,
    /// Invalid method.
    #[error("invalid method id")]
    InvalidMethodId,
    /// Invalid location.
    #[error("invalid location")]
    InvalidLocation,
    /// Invalid field.
    #[error("invalid field id")]
    InvalidFieldId,
    /// Invalid module.
    #[error("invalid module")]
    InvalidModule,
    /// There are no more frames on the call stack.
    #[error("no more frames")]
    NoMoreFrames,
    /// Information about the frame is not available.
    #[error("opaque frame")]
    OpaqueFrame,
    /// The variable type does not match the function used.
    #[error("type mismatch")]
    TypeMismatch,
    /// Invalid slot.
    #[error("invalid slot")]
    InvalidSlot,
    /// Item already set.
    #[error("duplicate")]
    Duplicate,
    /// Element not found.
    #[error("not found")]
    NotFound,
    /// Invalid raw monitor.
    #[error("invalid monitor")]
    InvalidMonitor,
    /// Not monitor owner.
    #[error("not monitor owner")]
    NotMonitorOwner,
    /// The call has been interrupted before completion.
    #[error("interrupt")]
    Interrupt,
    /// The class file format is malformed.
    #[error("invalid class format")]
    InvalidClassFormat,
    /// The requested change would lead to a circular class definition.
    #[error("circular class definition")]
    CircularClassDefinition,
    /// The class bytes fail verification.
    #[error("fails verification")]
    FailsVerification,
    /// A new method has been added during class redefinition.
    #[error("unsupported redefinition: method added")]
    UnsupportedRedefinitionMethodAdded,
    /// A schema change has occurred during class redefinition.
    #[error("unsupported redefinition: schema changed")]
    UnsupportedRedefinitionSchemaChanged,
    /// The thread state has been modified and is now inconsistent.
    #[error("invalid typestate")]
    InvalidTypestate,
    /// A direct superclass has changed during redefinition.
    #[error("unsupported redefinition: hierarchy changed")]
    UnsupportedRedefinitionHierarchyChanged,
    /// A method has been deleted during redefinition.
    #[error("unsupported redefinition: method deleted")]
    UnsupportedRedefinitionMethodDeleted,
    /// The class file version is not supported.
    #[error("unsupported version")]
    UnsupportedVersion,
    /// The class name defined in the new class file is different.
    #[error("names don't match")]
    NamesDontMatch,
    /// Class modifiers have changed during redefinition.
    #[error("unsupported redefinition: class modifiers changed")]
    UnsupportedRedefinitionClassModifiersChanged,
    /// Method modifiers have changed during redefinition.
    #[error("unsupported redefinition: method modifiers changed")]
    UnsupportedRedefinitionMethodModifiersChanged,
    /// A class attribute has changed during redefinition.
    #[error("unsupported redefinition: class attribute changed")]
    UnsupportedRedefinitionClassAttributeChanged,
    /// The requested operation is not supported.
    #[error("unsupported operation")]
    UnsupportedOperation,
    /// The class cannot be modified.
    #[error("unmodifiable class")]
    UnmodifiableClass,
    /// The module cannot be modified.
    #[error("unmodifiable module")]
    UnmodifiableModule,
    /// The functionality is not available in this virtual machine.
    #[error("not available")]
    NotAvailable,
    /// The environment does not possess the capability.
    #[error("must possess capability")]
    MustPossessCapability,
    /// Null pointer.
    #[error("null pointer")]
    NullPointer,
    /// The requested information is not available.
    #[error("absent information")]
    AbsentInformation,
    /// The specified event type is not valid.
    #[error("invalid event type")]
    InvalidEventType,
    /// Illegal argument.
    #[error("illegal argument")]
    IllegalArgument,
    /// The requested information is not available for native methods.
    #[error("native method")]
    NativeMethod,
    /// The class loader does not support this operation.
    #[error("class loader unsupported")]
    ClassLoaderUnsupported,
    /// The virtual machine ran out of memory.
    #[error("out of memory")]
    OutOfMemory,
    /// Access denied.
    #[error("access denied")]
    AccessDenied,
    /// The functionality is not available during the current phase.
    #[error("wrong phase")]
    WrongPhase,
    /// An unexpected internal error has occurred.
    #[error("internal")]
    Internal,
    /// The thread being used to call this function is not attached.
    #[error("unattached thread")]
    UnattachedThread,
    /// The environment is no longer valid.
    #[error("invalid environment")]
    InvalidEnvironment,

    /// An unrecognised JVMTI error code.
    #[error("unknown JVMTI error code: {0}")]
    Other(i32),

    // ── Library-level errors ─────────────────────────────────────────

    /// A null pointer was returned from a JVMTI function.
    #[error("null pointer returned from {0}")]
    NullPtr(&'static str),
    /// An unknown enum value was encountered.
    #[error("unknown enum value for {0}: {1}")]
    UnknownEnumValue(&'static str, i32),
    /// The JVMTI version is too old for the requested function.
    #[error("JVMTI version too old for requested function")]
    VersionTooOld,
    /// A JNI error occurred.
    #[error("JNI error: {0}")]
    Jni(#[from] jni::errors::Error),
}

impl From<sys::jvmtiError> for JvmtiError {
    fn from(value: sys::jvmtiError) -> Self {
        use sys::jvmtiError::*;
        match value {
            JVMTI_ERROR_NONE => unreachable!("JVMTI_ERROR_NONE should not be converted to error"),
            JVMTI_ERROR_INVALID_THREAD => Self::InvalidThread,
            JVMTI_ERROR_INVALID_THREAD_GROUP => Self::InvalidThreadGroup,
            JVMTI_ERROR_INVALID_PRIORITY => Self::InvalidPriority,
            JVMTI_ERROR_THREAD_NOT_SUSPENDED => Self::ThreadNotSuspended,
            JVMTI_ERROR_THREAD_SUSPENDED => Self::ThreadSuspended,
            JVMTI_ERROR_THREAD_NOT_ALIVE => Self::ThreadNotAlive,
            JVMTI_ERROR_INVALID_OBJECT => Self::InvalidObject,
            JVMTI_ERROR_INVALID_CLASS => Self::InvalidClass,
            JVMTI_ERROR_CLASS_NOT_PREPARED => Self::ClassNotPrepared,
            JVMTI_ERROR_INVALID_METHODID => Self::InvalidMethodId,
            JVMTI_ERROR_INVALID_LOCATION => Self::InvalidLocation,
            JVMTI_ERROR_INVALID_FIELDID => Self::InvalidFieldId,
            JVMTI_ERROR_INVALID_MODULE => Self::InvalidModule,
            JVMTI_ERROR_NO_MORE_FRAMES => Self::NoMoreFrames,
            JVMTI_ERROR_OPAQUE_FRAME => Self::OpaqueFrame,
            JVMTI_ERROR_TYPE_MISMATCH => Self::TypeMismatch,
            JVMTI_ERROR_INVALID_SLOT => Self::InvalidSlot,
            JVMTI_ERROR_DUPLICATE => Self::Duplicate,
            JVMTI_ERROR_NOT_FOUND => Self::NotFound,
            JVMTI_ERROR_INVALID_MONITOR => Self::InvalidMonitor,
            JVMTI_ERROR_NOT_MONITOR_OWNER => Self::NotMonitorOwner,
            JVMTI_ERROR_INTERRUPT => Self::Interrupt,
            JVMTI_ERROR_INVALID_CLASS_FORMAT => Self::InvalidClassFormat,
            JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION => Self::CircularClassDefinition,
            JVMTI_ERROR_FAILS_VERIFICATION => Self::FailsVerification,
            JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED => Self::UnsupportedRedefinitionMethodAdded,
            JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED => Self::UnsupportedRedefinitionSchemaChanged,
            JVMTI_ERROR_INVALID_TYPESTATE => Self::InvalidTypestate,
            JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED => Self::UnsupportedRedefinitionHierarchyChanged,
            JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED => Self::UnsupportedRedefinitionMethodDeleted,
            JVMTI_ERROR_UNSUPPORTED_VERSION => Self::UnsupportedVersion,
            JVMTI_ERROR_NAMES_DONT_MATCH => Self::NamesDontMatch,
            JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED => Self::UnsupportedRedefinitionClassModifiersChanged,
            JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED => Self::UnsupportedRedefinitionMethodModifiersChanged,
            JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED => Self::UnsupportedRedefinitionClassAttributeChanged,
            JVMTI_ERROR_UNSUPPORTED_OPERATION => Self::UnsupportedOperation,
            JVMTI_ERROR_UNMODIFIABLE_CLASS => Self::UnmodifiableClass,
            JVMTI_ERROR_UNMODIFIABLE_MODULE => Self::UnmodifiableModule,
            JVMTI_ERROR_NOT_AVAILABLE => Self::NotAvailable,
            JVMTI_ERROR_MUST_POSSESS_CAPABILITY => Self::MustPossessCapability,
            JVMTI_ERROR_NULL_POINTER => Self::NullPointer,
            JVMTI_ERROR_ABSENT_INFORMATION => Self::AbsentInformation,
            JVMTI_ERROR_INVALID_EVENT_TYPE => Self::InvalidEventType,
            JVMTI_ERROR_ILLEGAL_ARGUMENT => Self::IllegalArgument,
            JVMTI_ERROR_NATIVE_METHOD => Self::NativeMethod,
            JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED => Self::ClassLoaderUnsupported,
            JVMTI_ERROR_OUT_OF_MEMORY => Self::OutOfMemory,
            JVMTI_ERROR_ACCESS_DENIED => Self::AccessDenied,
            JVMTI_ERROR_WRONG_PHASE => Self::WrongPhase,
            JVMTI_ERROR_INTERNAL => Self::Internal,
            JVMTI_ERROR_UNATTACHED_THREAD => Self::UnattachedThread,
            JVMTI_ERROR_INVALID_ENVIRONMENT => Self::InvalidEnvironment,
        }
    }
}
