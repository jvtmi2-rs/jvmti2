//! Builder-pattern wrapper for JVMTI capabilities.

use crate::sys;

/// A builder for constructing a set of JVMTI capabilities.
///
/// Use the chainable methods to declare the capabilities you need, then pass
/// the result to [`Env::add_capabilities`](crate::Env::add_capabilities).
///
/// # Example
///
/// ```no_run
/// # use jvmti2::Capabilities;
/// let caps = Capabilities::new()
///     .can_tag_objects()
///     .can_suspend()
///     .can_generate_breakpoint_events();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Capabilities {
    pub(crate) inner: sys::jvmtiCapabilities,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! cap_method {
    (
        $(#[$meta:meta])*
        $method:ident => $flag:ident
    ) => {
        $(#[$meta])*
        #[must_use]
        pub const fn $method(mut self) -> Self {
            self.inner = self.inner.union(sys::jvmtiCapabilities::$flag);
            self
        }
    };
}

macro_rules! cap_check {
    (
        $(#[$meta:meta])*
        $method:ident => $flag:ident
    ) => {
        $(#[$meta])*
        pub const fn $method(&self) -> bool {
            self.inner.contains(sys::jvmtiCapabilities::$flag)
        }
    };
}

impl Capabilities {
    /// Creates an empty set of capabilities.
    pub const fn new() -> Self {
        Self {
            inner: sys::jvmtiCapabilities::empty(),
        }
    }

    /// Returns the raw `jvmtiCapabilities` bitflags value.
    pub const fn as_raw(&self) -> &sys::jvmtiCapabilities {
        &self.inner
    }

    /// Creates from raw `jvmtiCapabilities`.
    pub const fn from_raw(raw: sys::jvmtiCapabilities) -> Self {
        Self { inner: raw }
    }

    // ── Builder methods ──────────────────────────────────────────────

    cap_method! { /// Can set and get object tags.
        can_tag_objects => CAN_TAG_OBJECTS }
    cap_method! { /// Can generate field modification events.
        can_generate_field_modification_events => CAN_GENERATE_FIELD_MODIFICATION_EVENTS }
    cap_method! { /// Can generate field access events.
        can_generate_field_access_events => CAN_GENERATE_FIELD_ACCESS_EVENTS }
    cap_method! { /// Can get bytecodes of a method.
        can_get_bytecodes => CAN_GET_BYTECODES }
    cap_method! { /// Can test if a field or method is synthetic.
        can_get_synthetic_attribute => CAN_GET_SYNTHETIC_ATTRIBUTE }
    cap_method! { /// Can get information about owned monitors.
        can_get_owned_monitor_info => CAN_GET_OWNED_MONITOR_INFO }
    cap_method! { /// Can get the monitor a thread is waiting to enter.
        can_get_current_contended_monitor => CAN_GET_CURRENT_CONTENDED_MONITOR }
    cap_method! { /// Can get the monitor information.
        can_get_monitor_info => CAN_GET_MONITOR_INFO }
    cap_method! { /// Can pop frames off the stack.
        can_pop_frame => CAN_POP_FRAME }
    cap_method! { /// Can redefine classes.
        can_redefine_classes => CAN_REDEFINE_CLASSES }
    cap_method! { /// Can signal (stop) a thread.
        can_signal_thread => CAN_SIGNAL_THREAD }
    cap_method! { /// Can get the source file name of a class.
        can_get_source_file_name => CAN_GET_SOURCE_FILE_NAME }
    cap_method! { /// Can get line number information.
        can_get_line_numbers => CAN_GET_LINE_NUMBERS }
    cap_method! { /// Can get the source debug extension.
        can_get_source_debug_extension => CAN_GET_SOURCE_DEBUG_EXTENSION }
    cap_method! { /// Can access local variables.
        can_access_local_variables => CAN_ACCESS_LOCAL_VARIABLES }
    cap_method! { /// Can maintain original method ordering.
        can_maintain_original_method_order => CAN_MAINTAIN_ORIGINAL_METHOD_ORDER }
    cap_method! { /// Can generate single step events.
        can_generate_single_step_events => CAN_GENERATE_SINGLE_STEP_EVENTS }
    cap_method! { /// Can generate exception events.
        can_generate_exception_events => CAN_GENERATE_EXCEPTION_EVENTS }
    cap_method! { /// Can generate frame pop events.
        can_generate_frame_pop_events => CAN_GENERATE_FRAME_POP_EVENTS }
    cap_method! { /// Can generate breakpoint events.
        can_generate_breakpoint_events => CAN_GENERATE_BREAKPOINT_EVENTS }
    cap_method! { /// Can suspend and resume threads.
        can_suspend => CAN_SUSPEND }
    cap_method! { /// Can redefine any class.
        can_redefine_any_class => CAN_REDEFINE_ANY_CLASS }
    cap_method! { /// Can get current thread CPU time.
        can_get_current_thread_cpu_time => CAN_GET_CURRENT_THREAD_CPU_TIME }
    cap_method! { /// Can get thread CPU time.
        can_get_thread_cpu_time => CAN_GET_THREAD_CPU_TIME }
    cap_method! { /// Can generate method entry events.
        can_generate_method_entry_events => CAN_GENERATE_METHOD_ENTRY_EVENTS }
    cap_method! { /// Can generate method exit events.
        can_generate_method_exit_events => CAN_GENERATE_METHOD_EXIT_EVENTS }
    cap_method! { /// Can generate all class hook events.
        can_generate_all_class_hook_events => CAN_GENERATE_ALL_CLASS_HOOK_EVENTS }
    cap_method! { /// Can generate compiled method load events.
        can_generate_compiled_method_load_events => CAN_GENERATE_COMPILED_METHOD_LOAD_EVENTS }
    cap_method! { /// Can generate monitor events.
        can_generate_monitor_events => CAN_GENERATE_MONITOR_EVENTS }
    cap_method! { /// Can generate VM object alloc events.
        can_generate_vm_object_alloc_events => CAN_GENERATE_VM_OBJECT_ALLOC_EVENTS }
    cap_method! { /// Can generate native method bind events.
        can_generate_native_method_bind_events => CAN_GENERATE_NATIVE_METHOD_BIND_EVENTS }
    cap_method! { /// Can generate garbage collection events.
        can_generate_garbage_collection_events => CAN_GENERATE_GARBAGE_COLLECTION_EVENTS }
    cap_method! { /// Can generate object free events.
        can_generate_object_free_events => CAN_GENERATE_OBJECT_FREE_EVENTS }
    cap_method! { /// Can force early return.
        can_force_early_return => CAN_FORCE_EARLY_RETURN }
    cap_method! { /// Can get owned monitor stack depth info.
        can_get_owned_monitor_stack_depth_info => CAN_GET_OWNED_MONITOR_STACK_DEPTH_INFO }
    cap_method! { /// Can get the constant pool.
        can_get_constant_pool => CAN_GET_CONSTANT_POOL }
    cap_method! { /// Can set the native method prefix.
        can_set_native_method_prefix => CAN_SET_NATIVE_METHOD_PREFIX }
    cap_method! { /// Can retransform classes.
        can_retransform_classes => CAN_RETRANSFORM_CLASSES }
    cap_method! { /// Can retransform any class.
        can_retransform_any_class => CAN_RETRANSFORM_ANY_CLASS }
    cap_method! { /// Can generate resource exhaustion heap events.
        can_generate_resource_exhaustion_heap_events => CAN_GENERATE_RESOURCE_EXHAUSTION_HEAP_EVENTS }
    cap_method! { /// Can generate resource exhaustion thread events.
        can_generate_resource_exhaustion_threads_events => CAN_GENERATE_RESOURCE_EXHAUSTION_THREADS_EVENTS }
    cap_method! { /// Can generate early VM start events.
        can_generate_early_vmstart => CAN_GENERATE_EARLY_VMSTART }
    cap_method! { /// Can generate early class hook events.
        can_generate_early_class_hook_events => CAN_GENERATE_EARLY_CLASS_HOOK_EVENTS }
    cap_method! { /// Can generate sampled object alloc events.
        can_generate_sampled_object_alloc_events => CAN_GENERATE_SAMPLED_OBJECT_ALLOC_EVENTS }
    cap_method! { /// Can support virtual threads.
        can_support_virtual_threads => CAN_SUPPORT_VIRTUAL_THREADS }

    // ── Query methods ────────────────────────────────────────────────

    cap_check! { /// Returns `true` if `can_tag_objects` is set.
        has_tag_objects => CAN_TAG_OBJECTS }
    cap_check! { /// Returns `true` if `can_suspend` is set.
        has_suspend => CAN_SUSPEND }
    cap_check! { /// Returns `true` if `can_generate_breakpoint_events` is set.
        has_generate_breakpoint_events => CAN_GENERATE_BREAKPOINT_EVENTS }
    cap_check! { /// Returns `true` if `can_access_local_variables` is set.
        has_access_local_variables => CAN_ACCESS_LOCAL_VARIABLES }
    cap_check! { /// Returns `true` if `can_support_virtual_threads` is set.
        has_support_virtual_threads => CAN_SUPPORT_VIRTUAL_THREADS }
}
