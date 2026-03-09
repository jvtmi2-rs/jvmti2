//! Idiomatic Rust enums wrapping JVMTI C enumerations.

use crate::{errors::JvmtiError, sys};

/// Generates an idiomatic Rust enum from a JVMTI C enum.
macro_rules! define_jvmti_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident : $sys_ty:ty {
            $(
                $(#[$vmeta:meta])*
                $variant:ident = $sys_variant:ident,
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $(#[$vmeta])*
                $variant,
            )+
        }

        impl From<$name> for $sys_ty {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => <$sys_ty>::$sys_variant, )+
                }
            }
        }

        impl TryFrom<$sys_ty> for $name {
            type Error = JvmtiError;

            fn try_from(value: $sys_ty) -> Result<Self, Self::Error> {
                match value {
                    $( <$sys_ty>::$sys_variant => Ok(Self::$variant), )+
                    #[allow(unreachable_patterns)]
                    _ => Err(JvmtiError::UnknownEnumValue(
                        stringify!($name),
                        value as i32,
                    )),
                }
            }
        }
    };
}

define_jvmti_enum! {
    /// Whether to enable or disable event notification.
    pub enum EventMode : sys::jvmtiEventMode {
        /// Enable event notification.
        Enable = JVMTI_ENABLE,
        /// Disable event notification.
        Disable = JVMTI_DISABLE,
    }
}

define_jvmti_enum! {
    /// JVMTI event types.
    pub enum Event : sys::jvmtiEvent {
        /// VM initialization event.
        VmInit = JVMTI_EVENT_VM_INIT,
        /// VM death event.
        VmDeath = JVMTI_EVENT_VM_DEATH,
        /// Thread start event.
        ThreadStart = JVMTI_EVENT_THREAD_START,
        /// Thread end event.
        ThreadEnd = JVMTI_EVENT_THREAD_END,
        /// Class file load hook event.
        ClassFileLoadHook = JVMTI_EVENT_CLASS_FILE_LOAD_HOOK,
        /// Class load event.
        ClassLoad = JVMTI_EVENT_CLASS_LOAD,
        /// Class prepare event.
        ClassPrepare = JVMTI_EVENT_CLASS_PREPARE,
        /// VM start event.
        VmStart = JVMTI_EVENT_VM_START,
        /// Exception event.
        Exception = JVMTI_EVENT_EXCEPTION,
        /// Exception catch event.
        ExceptionCatch = JVMTI_EVENT_EXCEPTION_CATCH,
        /// Single step event.
        SingleStep = JVMTI_EVENT_SINGLE_STEP,
        /// Frame pop event.
        FramePop = JVMTI_EVENT_FRAME_POP,
        /// Breakpoint event.
        Breakpoint = JVMTI_EVENT_BREAKPOINT,
        /// Field access event.
        FieldAccess = JVMTI_EVENT_FIELD_ACCESS,
        /// Field modification event.
        FieldModification = JVMTI_EVENT_FIELD_MODIFICATION,
        /// Method entry event.
        MethodEntry = JVMTI_EVENT_METHOD_ENTRY,
        /// Method exit event.
        MethodExit = JVMTI_EVENT_METHOD_EXIT,
        /// Native method bind event.
        NativeMethodBind = JVMTI_EVENT_NATIVE_METHOD_BIND,
        /// Compiled method load event.
        CompiledMethodLoad = JVMTI_EVENT_COMPILED_METHOD_LOAD,
        /// Compiled method unload event.
        CompiledMethodUnload = JVMTI_EVENT_COMPILED_METHOD_UNLOAD,
        /// Dynamic code generated event.
        DynamicCodeGenerated = JVMTI_EVENT_DYNAMIC_CODE_GENERATED,
        /// Data dump request event.
        DataDumpRequest = JVMTI_EVENT_DATA_DUMP_REQUEST,
        /// Monitor wait event.
        MonitorWait = JVMTI_EVENT_MONITOR_WAIT,
        /// Monitor waited event.
        MonitorWaited = JVMTI_EVENT_MONITOR_WAITED,
        /// Monitor contended enter event.
        MonitorContendedEnter = JVMTI_EVENT_MONITOR_CONTENDED_ENTER,
        /// Monitor contended entered event.
        MonitorContendedEntered = JVMTI_EVENT_MONITOR_CONTENDED_ENTERED,
        /// Resource exhausted event.
        ResourceExhausted = JVMTI_EVENT_RESOURCE_EXHAUSTED,
        /// Garbage collection start event.
        GarbageCollectionStart = JVMTI_EVENT_GARBAGE_COLLECTION_START,
        /// Garbage collection finish event.
        GarbageCollectionFinish = JVMTI_EVENT_GARBAGE_COLLECTION_FINISH,
        /// Object free event.
        ObjectFree = JVMTI_EVENT_OBJECT_FREE,
        /// VM object alloc event.
        VmObjectAlloc = JVMTI_EVENT_VM_OBJECT_ALLOC,
        /// Sampled object alloc event.
        SampledObjectAlloc = JVMTI_EVENT_SAMPLED_OBJECT_ALLOC,
        /// Virtual thread start event.
        VirtualThreadStart = JVMTI_EVENT_VIRTUAL_THREAD_START,
        /// Virtual thread end event.
        VirtualThreadEnd = JVMTI_EVENT_VIRTUAL_THREAD_END,
    }
}

define_jvmti_enum! {
    /// Execution phase of the VM.
    pub enum Phase : sys::jvmtiPhase {
        /// The `Agent_OnLoad` or `Agent_OnAttach` function is executing.
        OnLoad = JVMTI_PHASE_ONLOAD,
        /// The VM is starting but has not yet loaded the primordial classes.
        Primordial = JVMTI_PHASE_PRIMORDIAL,
        /// The VM has started but initialization is not yet complete.
        Start = JVMTI_PHASE_START,
        /// The VM is initialized and fully operational.
        Live = JVMTI_PHASE_LIVE,
        /// The VM has died.
        Dead = JVMTI_PHASE_DEAD,
    }
}

define_jvmti_enum! {
    /// Heap reference kind.
    pub enum HeapReferenceKind : sys::jvmtiHeapReferenceKind {
        /// Reference from a class.
        Class = JVMTI_HEAP_REFERENCE_CLASS,
        /// Reference from a field.
        Field = JVMTI_HEAP_REFERENCE_FIELD,
        /// Reference from an array element.
        ArrayElement = JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT,
        /// Reference from a class loader.
        ClassLoader = JVMTI_HEAP_REFERENCE_CLASS_LOADER,
        /// Reference from signers.
        Signers = JVMTI_HEAP_REFERENCE_SIGNERS,
        /// Reference from a protection domain.
        ProtectionDomain = JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN,
        /// Reference from an interface.
        Interface = JVMTI_HEAP_REFERENCE_INTERFACE,
        /// Reference from a static field.
        StaticField = JVMTI_HEAP_REFERENCE_STATIC_FIELD,
        /// Reference from the constant pool.
        ConstantPool = JVMTI_HEAP_REFERENCE_CONSTANT_POOL,
        /// Reference from a superclass.
        Superclass = JVMTI_HEAP_REFERENCE_SUPERCLASS,
        /// Reference from a JNI global.
        JniGlobal = JVMTI_HEAP_REFERENCE_JNI_GLOBAL,
        /// Reference from a system class.
        SystemClass = JVMTI_HEAP_REFERENCE_SYSTEM_CLASS,
        /// Reference from a monitor.
        Monitor = JVMTI_HEAP_REFERENCE_MONITOR,
        /// Reference from a stack local.
        StackLocal = JVMTI_HEAP_REFERENCE_STACK_LOCAL,
        /// Reference from a JNI local.
        JniLocal = JVMTI_HEAP_REFERENCE_JNI_LOCAL,
        /// Reference from a thread.
        Thread = JVMTI_HEAP_REFERENCE_THREAD,
        /// Other reference.
        Other = JVMTI_HEAP_REFERENCE_OTHER,
    }
}

define_jvmti_enum! {
    /// Primitive type.
    pub enum PrimitiveType : sys::jvmtiPrimitiveType {
        /// `boolean`
        Boolean = JVMTI_PRIMITIVE_TYPE_BOOLEAN,
        /// `byte`
        Byte = JVMTI_PRIMITIVE_TYPE_BYTE,
        /// `char`
        Char = JVMTI_PRIMITIVE_TYPE_CHAR,
        /// `short`
        Short = JVMTI_PRIMITIVE_TYPE_SHORT,
        /// `int`
        Int = JVMTI_PRIMITIVE_TYPE_INT,
        /// `long`
        Long = JVMTI_PRIMITIVE_TYPE_LONG,
        /// `float`
        Float = JVMTI_PRIMITIVE_TYPE_FLOAT,
        /// `double`
        Double = JVMTI_PRIMITIVE_TYPE_DOUBLE,
    }
}

define_jvmti_enum! {
    /// Heap object filter.
    pub enum HeapObjectFilter : sys::jvmtiHeapObjectFilter {
        /// Only tagged objects.
        Tagged = JVMTI_HEAP_OBJECT_TAGGED,
        /// Only untagged objects.
        Untagged = JVMTI_HEAP_OBJECT_UNTAGGED,
        /// All objects.
        Either = JVMTI_HEAP_OBJECT_EITHER,
    }
}

define_jvmti_enum! {
    /// Heap root kind.
    pub enum HeapRootKind : sys::jvmtiHeapRootKind {
        /// JNI global reference.
        JniGlobal = JVMTI_HEAP_ROOT_JNI_GLOBAL,
        /// System class.
        SystemClass = JVMTI_HEAP_ROOT_SYSTEM_CLASS,
        /// Monitor.
        Monitor = JVMTI_HEAP_ROOT_MONITOR,
        /// Stack local.
        StackLocal = JVMTI_HEAP_ROOT_STACK_LOCAL,
        /// JNI local reference.
        JniLocal = JVMTI_HEAP_ROOT_JNI_LOCAL,
        /// Thread.
        Thread = JVMTI_HEAP_ROOT_THREAD,
        /// Other root.
        Other = JVMTI_HEAP_ROOT_OTHER,
    }
}

define_jvmti_enum! {
    /// Object reference kind (legacy iteration API).
    pub enum ObjectReferenceKind : sys::jvmtiObjectReferenceKind {
        /// Reference from a class.
        Class = JVMTI_REFERENCE_CLASS,
        /// Reference from a field.
        Field = JVMTI_REFERENCE_FIELD,
        /// Reference from an array element.
        ArrayElement = JVMTI_REFERENCE_ARRAY_ELEMENT,
        /// Reference from a class loader.
        ClassLoader = JVMTI_REFERENCE_CLASS_LOADER,
        /// Reference from signers.
        Signers = JVMTI_REFERENCE_SIGNERS,
        /// Reference from a protection domain.
        ProtectionDomain = JVMTI_REFERENCE_PROTECTION_DOMAIN,
        /// Reference from an interface.
        Interface = JVMTI_REFERENCE_INTERFACE,
        /// Reference from a static field.
        StaticField = JVMTI_REFERENCE_STATIC_FIELD,
        /// Reference from the constant pool.
        ConstantPool = JVMTI_REFERENCE_CONSTANT_POOL,
    }
}

define_jvmti_enum! {
    /// Iteration control.
    pub enum IterationControl : sys::jvmtiIterationControl {
        /// Continue iteration.
        Continue = JVMTI_ITERATION_CONTINUE,
        /// Continue without visiting referees.
        Ignore = JVMTI_ITERATION_IGNORE,
        /// Abort iteration.
        Abort = JVMTI_ITERATION_ABORT,
    }
}

define_jvmti_enum! {
    /// Extension function/event parameter type.
    pub enum ParamType : sys::jvmtiParamTypes {
        /// `jbyte`
        JByte = JVMTI_TYPE_JBYTE,
        /// `jchar`
        JChar = JVMTI_TYPE_JCHAR,
        /// `jshort`
        JShort = JVMTI_TYPE_JSHORT,
        /// `jint`
        JInt = JVMTI_TYPE_JINT,
        /// `jlong`
        JLong = JVMTI_TYPE_JLONG,
        /// `jfloat`
        JFloat = JVMTI_TYPE_JFLOAT,
        /// `jdouble`
        JDouble = JVMTI_TYPE_JDOUBLE,
        /// `jboolean`
        JBoolean = JVMTI_TYPE_JBOOLEAN,
        /// `jobject`
        JObject = JVMTI_TYPE_JOBJECT,
        /// `jthread`
        JThread = JVMTI_TYPE_JTHREAD,
        /// `jclass`
        JClass = JVMTI_TYPE_JCLASS,
        /// `jvalue`
        JValue = JVMTI_TYPE_JVALUE,
        /// `jfieldID`
        JFieldId = JVMTI_TYPE_JFIELDID,
        /// `jmethodID`
        JMethodId = JVMTI_TYPE_JMETHODID,
        /// `char*`
        CChar = JVMTI_TYPE_CCHAR,
        /// `void*`
        CVoid = JVMTI_TYPE_CVOID,
        /// `JNIEnv*`
        JniEnv = JVMTI_TYPE_JNIENV,
    }
}

define_jvmti_enum! {
    /// Extension function/event parameter kind.
    pub enum ParamKind : sys::jvmtiParamKind {
        /// Input parameter.
        In = JVMTI_KIND_IN,
        /// Input pointer parameter.
        InPtr = JVMTI_KIND_IN_PTR,
        /// Input buffer parameter.
        InBuf = JVMTI_KIND_IN_BUF,
        /// Allocated buffer parameter.
        AllocBuf = JVMTI_KIND_ALLOC_BUF,
        /// Allocated-allocated buffer parameter.
        AllocAllocBuf = JVMTI_KIND_ALLOC_ALLOC_BUF,
        /// Output parameter.
        Out = JVMTI_KIND_OUT,
        /// Output buffer parameter.
        OutBuf = JVMTI_KIND_OUT_BUF,
    }
}

define_jvmti_enum! {
    /// Timer kind.
    pub enum TimerKind : sys::jvmtiTimerKind {
        /// User CPU time.
        UserCpu = JVMTI_TIMER_USER_CPU,
        /// Total CPU time.
        TotalCpu = JVMTI_TIMER_TOTAL_CPU,
        /// Elapsed time.
        Elapsed = JVMTI_TIMER_ELAPSED,
    }
}

define_jvmti_enum! {
    /// Verbose flag.
    pub enum VerboseFlag : sys::jvmtiVerboseFlag {
        /// Other verbose output.
        Other = JVMTI_VERBOSE_OTHER,
        /// GC verbose output.
        Gc = JVMTI_VERBOSE_GC,
        /// Class loading verbose output.
        Class = JVMTI_VERBOSE_CLASS,
        /// JNI verbose output.
        Jni = JVMTI_VERBOSE_JNI,
    }
}

define_jvmti_enum! {
    /// JLocation format.
    pub enum JLocationFormat : sys::jvmtiJlocationFormat {
        /// JVM bytecode index.
        JvmBci = JVMTI_JLOCATION_JVMBCI,
        /// Machine PC.
        MachinePc = JVMTI_JLOCATION_MACHINEPC,
        /// Other format.
        Other = JVMTI_JLOCATION_OTHER,
    }
}
