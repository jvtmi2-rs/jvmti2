//! Event callback system.
//!
//! Two APIs are provided:
//!
//! - **[`EventCallbacksBuilder`]** — raw fn-pointer API. Zero overhead,
//!   `unsafe`, full control.
//! - **[`EventHandler`]** trait — safe, trait-based API. Implement the trait
//!   and install it via [`Env::install_event_handler`](crate::Env::install_event_handler).
//!   Uses `dyn` dispatch (one vtable lookup per event invocation).

use core::ffi::{c_char, c_void};

use crate::sys;

/// Builder for raw event callbacks.
///
/// Each setter is `unsafe` because the caller must guarantee the function
/// pointer remains valid for the lifetime of the JVMTI environment.
///
/// # Example
///
/// ```no_run
/// # use jvmti2::event::EventCallbacksBuilder;
/// unsafe {
///     let callbacks = EventCallbacksBuilder::new()
///         .vm_init(my_vm_init_fn)
///         .class_load(my_class_load_fn);
///     callbacks.apply(&env).unwrap();
/// }
/// ```
pub struct EventCallbacksBuilder {
    raw: sys::jvmtiEventCallbacks,
}

impl core::fmt::Debug for EventCallbacksBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventCallbacksBuilder").finish_non_exhaustive()
    }
}

impl Default for EventCallbacksBuilder {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! event_setter {
    (
        $(#[$meta:meta])*
        $method:ident, $version:ident, $field:ident, $ty:ty
    ) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The function pointer must remain valid for the lifetime of the
        /// JVMTI environment.
        #[must_use]
        pub unsafe fn $method(mut self, cb: $ty) -> Self {
            self.raw.$version.$field = Some(cb);
            self
        }
    };
}

impl EventCallbacksBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        // SAFETY: jvmtiEventCallbacks is all Option<fn> fields — zeroed = all None.
        Self {
            raw: unsafe { core::mem::zeroed() },
        }
    }

    /// Returns the raw callbacks struct.
    pub fn as_raw(&self) -> &sys::jvmtiEventCallbacks {
        &self.raw
    }

    /// Consumes the builder and returns the raw callbacks struct.
    pub fn into_raw(self) -> sys::jvmtiEventCallbacks {
        self.raw
    }

    // ── JVMTI 1.0 callbacks ─────────────────────────────────────────

    event_setter! { /// Set the VM init callback.
        vm_init, v1, VMInit, sys::jvmtiEventVMInit }
    event_setter! { /// Set the VM death callback.
        vm_death, v1, VMDeath, sys::jvmtiEventVMDeath }
    event_setter! { /// Set the thread start callback.
        thread_start, v1, ThreadStart, sys::jvmtiEventThreadStart }
    event_setter! { /// Set the thread end callback.
        thread_end, v1, ThreadEnd, sys::jvmtiEventThreadEnd }
    event_setter! { /// Set the class file load hook callback.
        class_file_load_hook, v1, ClassFileLoadHook, sys::jvmtiEventClassFileLoadHook }
    event_setter! { /// Set the class load callback.
        class_load, v1, ClassLoad, sys::jvmtiEventClassLoad }
    event_setter! { /// Set the class prepare callback.
        class_prepare, v1, ClassPrepare, sys::jvmtiEventClassPrepare }
    event_setter! { /// Set the VM start callback.
        vm_start, v1, VMStart, sys::jvmtiEventVMStart }
    event_setter! { /// Set the exception callback.
        exception, v1, Exception, sys::jvmtiEventException }
    event_setter! { /// Set the exception catch callback.
        exception_catch, v1, ExceptionCatch, sys::jvmtiEventExceptionCatch }
    event_setter! { /// Set the single step callback.
        single_step, v1, SingleStep, sys::jvmtiEventSingleStep }
    event_setter! { /// Set the frame pop callback.
        frame_pop, v1, FramePop, sys::jvmtiEventFramePop }
    event_setter! { /// Set the breakpoint callback.
        breakpoint, v1, Breakpoint, sys::jvmtiEventBreakpoint }
    event_setter! { /// Set the field access callback.
        field_access, v1, FieldAccess, sys::jvmtiEventFieldAccess }
    event_setter! { /// Set the field modification callback.
        field_modification, v1, FieldModification, sys::jvmtiEventFieldModification }
    event_setter! { /// Set the method entry callback.
        method_entry, v1, MethodEntry, sys::jvmtiEventMethodEntry }
    event_setter! { /// Set the method exit callback.
        method_exit, v1, MethodExit, sys::jvmtiEventMethodExit }
    event_setter! { /// Set the native method bind callback.
        native_method_bind, v1, NativeMethodBind, sys::jvmtiEventNativeMethodBind }
    event_setter! { /// Set the compiled method load callback.
        compiled_method_load, v1, CompiledMethodLoad, sys::jvmtiEventCompiledMethodLoad }
    event_setter! { /// Set the compiled method unload callback.
        compiled_method_unload, v1, CompiledMethodUnload, sys::jvmtiEventCompiledMethodUnload }
    event_setter! { /// Set the dynamic code generated callback.
        dynamic_code_generated, v1, DynamicCodeGenerated, sys::jvmtiEventDynamicCodeGenerated }
    event_setter! { /// Set the data dump request callback.
        data_dump_request, v1, DataDumpRequest, sys::jvmtiEventDataDumpRequest }
    event_setter! { /// Set the monitor wait callback.
        monitor_wait, v1, MonitorWait, sys::jvmtiEventMonitorWait }
    event_setter! { /// Set the monitor waited callback.
        monitor_waited, v1, MonitorWaited, sys::jvmtiEventMonitorWaited }
    event_setter! { /// Set the monitor contended enter callback.
        monitor_contended_enter, v1, MonitorContendedEnter, sys::jvmtiEventMonitorContendedEnter }
    event_setter! { /// Set the monitor contended entered callback.
        monitor_contended_entered, v1, MonitorContendedEntered, sys::jvmtiEventMonitorContendedEntered }
    event_setter! { /// Set the garbage collection start callback.
        garbage_collection_start, v1, GarbageCollectionStart, sys::jvmtiEventGarbageCollectionStart }
    event_setter! { /// Set the garbage collection finish callback.
        garbage_collection_finish, v1, GarbageCollectionFinish, sys::jvmtiEventGarbageCollectionFinish }
    event_setter! { /// Set the object free callback.
        object_free, v1, ObjectFree, sys::jvmtiEventObjectFree }
    event_setter! { /// Set the VM object alloc callback.
        vm_object_alloc, v1, VMObjectAlloc, sys::jvmtiEventVMObjectAlloc }

    // ── JVMTI 1.1 callbacks ─────────────────────────────────────────

    event_setter! { /// Set the resource exhausted callback (JVMTI 1.1+).
        resource_exhausted, v1_1, ResourceExhausted, sys::jvmtiEventResourceExhausted }

    // ── JVMTI 11 callbacks ──────────────────────────────────────────

    event_setter! { /// Set the sampled object alloc callback (JVMTI 11+).
        sampled_object_alloc, v11, SampledObjectAlloc, sys::jvmtiEventSampledObjectAlloc }

    // ── JVMTI 21 callbacks ──────────────────────────────────────────

    event_setter! { /// Set the virtual thread start callback (JVMTI 21+).
        virtual_thread_start, v21, VirtualThreadStart, sys::jvmtiEventVirtualThreadStart }
    event_setter! { /// Set the virtual thread end callback (JVMTI 21+).
        virtual_thread_end, v21, VirtualThreadEnd, sys::jvmtiEventVirtualThreadEnd }
}

// ── Trait-based safe event handler API ────────────────────────────────

/// Trait for handling JVMTI events.
///
/// All methods have default no-op implementations. Override only the events
/// you need. Handlers may be called concurrently from multiple JVM threads.
///
/// Install via [`Env::install_event_handler`](crate::Env::install_event_handler).
/// You must still enable each desired event with
/// [`Env::set_event_notification_mode`](crate::Env::set_event_notification_mode).
///
/// # Example
///
/// ```no_run
/// use jvmti2::event::EventHandler;
/// use jvmti2::Env;
///
/// struct MyHandler;
///
/// impl EventHandler for MyHandler {
///     fn vm_init(&self, _env: &Env<'_>, _thread: jni_sys::jobject) {
///         println!("VM initialized!");
///     }
/// }
/// ```
#[allow(unused_variables, clippy::too_many_arguments)]
pub trait EventHandler: Send + Sync + 'static {
    /// VM initialization complete.
    fn vm_init(&self, env: &crate::Env<'_>, thread: jni_sys::jobject) {}
    /// VM about to terminate.
    fn vm_death(&self, env: &crate::Env<'_>) {}
    /// A thread has started.
    fn thread_start(&self, env: &crate::Env<'_>, thread: jni_sys::jobject) {}
    /// A thread has ended.
    fn thread_end(&self, env: &crate::Env<'_>, thread: jni_sys::jobject) {}
    /// A class file is being loaded; the handler may transform the bytecode.
    ///
    /// # Note
    ///
    /// `new_class_data` is typed as `*mut *mut c_uint` due to a jvmti2-sys bug
    /// (should be `*mut *mut c_uchar`). Cast appropriately if you need to set it.
    fn class_file_load_hook(
        &self,
        env: &crate::Env<'_>,
        class_being_redefined: jni_sys::jclass,
        loader: jni_sys::jobject,
        name: *const c_char,
        protection_domain: jni_sys::jobject,
        class_data_len: jni_sys::jint,
        class_data: *const core::ffi::c_uchar,
        new_class_data_len: *mut jni_sys::jint,
        new_class_data: *mut *mut core::ffi::c_uint,
    ) {
    }
    /// A class has been loaded.
    fn class_load(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        klass: jni_sys::jclass,
    ) {
    }
    /// A class has been prepared.
    fn class_prepare(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        klass: jni_sys::jclass,
    ) {
    }
    /// VM has started (JNI available, but not fully initialized).
    fn vm_start(&self, env: &crate::Env<'_>) {}
    /// An exception was thrown.
    fn exception(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        location: sys::jlocation,
        exception: jni_sys::jobject,
        catch_method: jni_sys::jmethodID,
        catch_location: sys::jlocation,
    ) {
    }
    /// An exception was caught.
    fn exception_catch(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        location: sys::jlocation,
        exception: jni_sys::jobject,
    ) {
    }
    /// A single step event.
    fn single_step(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        location: sys::jlocation,
    ) {
    }
    /// A frame was popped.
    fn frame_pop(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        was_popped_by_exception: jni_sys::jboolean,
    ) {
    }
    /// A breakpoint was hit.
    fn breakpoint(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        location: sys::jlocation,
    ) {
    }
    /// A field was accessed (read).
    fn field_access(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        location: sys::jlocation,
        field_klass: jni_sys::jclass,
        object: jni_sys::jobject,
        field: jni_sys::jfieldID,
    ) {
    }
    /// A field was modified (written).
    fn field_modification(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        location: sys::jlocation,
        field_klass: jni_sys::jclass,
        object: jni_sys::jobject,
        field: jni_sys::jfieldID,
        signature_type: c_char,
        new_value: jni_sys::jvalue,
    ) {
    }
    /// Method was entered.
    fn method_entry(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
    ) {
    }
    /// Method was exited.
    fn method_exit(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        was_popped_by_exception: jni_sys::jboolean,
        return_value: jni_sys::jvalue,
    ) {
    }
    /// A native method is being bound to its implementation.
    fn native_method_bind(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        address: *mut c_void,
        new_address_ptr: *mut *mut c_void,
    ) {
    }
    /// A compiled method has been loaded into memory.
    ///
    /// # Note
    ///
    /// Due to a jvmti2-sys bug, the second C parameter is typed as `*mut JNIEnv`
    /// but is actually a `jmethodID`. The trampoline casts it correctly before
    /// calling this method, so `method` is valid.
    fn compiled_method_load(
        &self,
        env: &crate::Env<'_>,
        method: jni_sys::jmethodID,
        code_size: jni_sys::jint,
        code_addr: *const c_void,
        map_length: jni_sys::jint,
        map: *const sys::jvmtiAddrLocationMap,
        compile_info: *const c_void,
    ) {
    }
    /// A compiled method has been unloaded from memory.
    fn compiled_method_unload(
        &self,
        env: &crate::Env<'_>,
        method: jni_sys::jmethodID,
        code_addr: *const c_void,
    ) {
    }
    /// Dynamically generated code was loaded.
    fn dynamic_code_generated(
        &self,
        env: &crate::Env<'_>,
        name: *const c_char,
        address: *const c_void,
        length: jni_sys::jint,
    ) {
    }
    /// The VM received a data dump request (e.g., Ctrl+Break).
    fn data_dump_request(&self, env: &crate::Env<'_>) {}
    /// A thread is about to wait on a monitor.
    fn monitor_wait(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        object: jni_sys::jobject,
        timeout: jni_sys::jlong,
    ) {
    }
    /// A thread finished waiting on a monitor.
    fn monitor_waited(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        object: jni_sys::jobject,
        timed_out: jni_sys::jboolean,
    ) {
    }
    /// A thread is about to enter a contended monitor.
    fn monitor_contended_enter(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        object: jni_sys::jobject,
    ) {
    }
    /// A thread entered a contended monitor.
    fn monitor_contended_entered(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        object: jni_sys::jobject,
    ) {
    }
    /// A resource has been exhausted (OOM, thread limit, etc.).
    fn resource_exhausted(
        &self,
        env: &crate::Env<'_>,
        flags: sys::JVMTI_RESOURCE_EXHAUSTED_FLAGS,
        reserved: *const c_void,
        description: *const c_char,
    ) {
    }
    /// Garbage collection started.
    fn garbage_collection_start(&self, env: &crate::Env<'_>) {}
    /// Garbage collection finished.
    fn garbage_collection_finish(&self, env: &crate::Env<'_>) {}
    /// A tagged object was freed.
    fn object_free(&self, env: &crate::Env<'_>, tag: jni_sys::jlong) {}
    /// The VM allocated an object.
    fn vm_object_alloc(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        object: jni_sys::jobject,
        object_klass: jni_sys::jclass,
        size: jni_sys::jlong,
    ) {
    }
    /// A sampled object allocation occurred (JVMTI 11+).
    fn sampled_object_alloc(
        &self,
        env: &crate::Env<'_>,
        thread: jni_sys::jobject,
        object: jni_sys::jobject,
        object_class: jni_sys::jclass,
        size: jni_sys::jlong,
    ) {
    }
    /// A virtual thread started (JVMTI 21+).
    fn virtual_thread_start(&self, env: &crate::Env<'_>, thread: jni_sys::jobject) {}
    /// A virtual thread ended (JVMTI 21+).
    fn virtual_thread_end(&self, env: &crate::Env<'_>, thread: jni_sys::jobject) {}
}

// ── Callback state and trampolines ───────────────────────────────────

/// State stored in JVMTI environment-local storage by [`install_handler`].
struct CallbackState {
    handler: Box<dyn EventHandler>,
    vm: *mut jni_sys::JavaVM,
}

// SAFETY: CallbackState is stored per-environment. The handler is Send+Sync
// and the vm pointer is safe to share across threads.
unsafe impl Send for CallbackState {}
unsafe impl Sync for CallbackState {}

/// Retrieves the [`CallbackState`] from environment-local storage.
///
/// # Safety
///
/// Must only be called from a trampoline when a handler is installed.
unsafe fn get_callback_state(jvmti_env: *mut sys::jvmtiEnv) -> *const CallbackState {
    let interface = *jvmti_env;
    let mut data: *mut c_void = core::ptr::null_mut();
    ((*interface).v1.GetEnvironmentLocalStorage)(jvmti_env, &mut data);
    data as *const CallbackState
}

/// Creates a temporary [`Env`](crate::Env) for use inside a trampoline.
///
/// The returned `ManuallyDrop<Env>` must NOT be dropped — the underlying
/// `jvmtiEnv` is owned by the JVM, not by us.
unsafe fn trampoline_env(
    jvmti_env: *mut sys::jvmtiEnv,
    state: &CallbackState,
) -> core::mem::ManuallyDrop<crate::Env<'static>> {
    core::mem::ManuallyDrop::new(crate::Env::from_raw(jvmti_env, state.vm))
}

// Trampolines for events that receive (jvmti_env, jni_env, ...).
macro_rules! trampoline_jni {
    (fn $name:ident($($param:ident: $ty:ty),*) => $method:ident) => {
        unsafe extern "system" fn $name(
            jvmti_env: *mut sys::jvmtiEnv,
            _jni_env: *mut jni_sys::JNIEnv,
            $($param: $ty),*
        ) {
            let state = &*get_callback_state(jvmti_env);
            let env = trampoline_env(jvmti_env, state);
            state.handler.$method(&env $(, $param)*);
        }
    };
}

// Trampolines for events that receive (jvmti_env, ...) without jni_env.
macro_rules! trampoline_raw {
    (fn $name:ident($($param:ident: $ty:ty),*) => $method:ident) => {
        unsafe extern "system" fn $name(
            jvmti_env: *mut sys::jvmtiEnv,
            $($param: $ty),*
        ) {
            let state = &*get_callback_state(jvmti_env);
            let env = trampoline_env(jvmti_env, state);
            state.handler.$method(&env $(, $param)*);
        }
    };
}

// ── v1 events with jni_env ───────────────────────────────────────────

trampoline_jni! { fn tramp_vm_init(thread: jni_sys::jobject) => vm_init }
trampoline_jni! { fn tramp_vm_death() => vm_death }
trampoline_jni! { fn tramp_thread_start(thread: jni_sys::jobject) => thread_start }
trampoline_jni! { fn tramp_thread_end(thread: jni_sys::jobject) => thread_end }
trampoline_jni! { fn tramp_class_file_load_hook(
    class_being_redefined: jni_sys::jclass,
    loader: jni_sys::jobject,
    name: *const c_char,
    protection_domain: jni_sys::jobject,
    class_data_len: jni_sys::jint,
    class_data: *const core::ffi::c_uchar,
    new_class_data_len: *mut jni_sys::jint,
    new_class_data: *mut *mut core::ffi::c_uint
) => class_file_load_hook }
trampoline_jni! { fn tramp_class_load(
    thread: jni_sys::jobject, klass: jni_sys::jclass
) => class_load }
trampoline_jni! { fn tramp_class_prepare(
    thread: jni_sys::jobject, klass: jni_sys::jclass
) => class_prepare }
trampoline_jni! { fn tramp_vm_start() => vm_start }
trampoline_jni! { fn tramp_exception(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    exception: jni_sys::jobject,
    catch_method: jni_sys::jmethodID,
    catch_location: sys::jlocation
) => exception }
trampoline_jni! { fn tramp_exception_catch(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    exception: jni_sys::jobject
) => exception_catch }
trampoline_jni! { fn tramp_single_step(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation
) => single_step }
trampoline_jni! { fn tramp_frame_pop(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    was_popped_by_exception: jni_sys::jboolean
) => frame_pop }
trampoline_jni! { fn tramp_breakpoint(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation
) => breakpoint }
trampoline_jni! { fn tramp_field_access(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    field_klass: jni_sys::jclass,
    object: jni_sys::jobject,
    field: jni_sys::jfieldID
) => field_access }
trampoline_jni! { fn tramp_field_modification(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    field_klass: jni_sys::jclass,
    object: jni_sys::jobject,
    field: jni_sys::jfieldID,
    signature_type: c_char,
    new_value: jni_sys::jvalue
) => field_modification }
trampoline_jni! { fn tramp_method_entry(
    thread: jni_sys::jobject, method: jni_sys::jmethodID
) => method_entry }
trampoline_jni! { fn tramp_method_exit(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    was_popped_by_exception: jni_sys::jboolean,
    return_value: jni_sys::jvalue
) => method_exit }
trampoline_jni! { fn tramp_native_method_bind(
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    address: *mut c_void,
    new_address_ptr: *mut *mut c_void
) => native_method_bind }
trampoline_jni! { fn tramp_monitor_wait(
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    timeout: jni_sys::jlong
) => monitor_wait }
trampoline_jni! { fn tramp_monitor_waited(
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    timed_out: jni_sys::jboolean
) => monitor_waited }
trampoline_jni! { fn tramp_monitor_contended_enter(
    thread: jni_sys::jobject, object: jni_sys::jobject
) => monitor_contended_enter }
trampoline_jni! { fn tramp_monitor_contended_entered(
    thread: jni_sys::jobject, object: jni_sys::jobject
) => monitor_contended_entered }
trampoline_jni! { fn tramp_vm_object_alloc(
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    object_klass: jni_sys::jclass,
    size: jni_sys::jlong
) => vm_object_alloc }

// ── v1 events WITHOUT jni_env ────────────────────────────────────────

trampoline_raw! { fn tramp_data_dump_request() => data_dump_request }
trampoline_raw! { fn tramp_gc_start() => garbage_collection_start }
trampoline_raw! { fn tramp_gc_finish() => garbage_collection_finish }
trampoline_raw! { fn tramp_object_free(tag: jni_sys::jlong) => object_free }
trampoline_raw! { fn tramp_compiled_method_unload(
    method: jni_sys::jmethodID, code_addr: *const c_void
) => compiled_method_unload }
trampoline_raw! { fn tramp_dynamic_code_generated(
    name: *const c_char, address: *const c_void, length: jni_sys::jint
) => dynamic_code_generated }

unsafe extern "system" fn tramp_compiled_method_load(
    jvmti_env: *mut sys::jvmtiEnv,
    method: jni_sys::jmethodID,
    code_size: jni_sys::jint,
    code_addr: *const c_void,
    map_length: jni_sys::jint,
    map: *const sys::jvmtiAddrLocationMap,
    compile_info: *const c_void,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    state
        .handler
        .compiled_method_load(&env, method, code_size, code_addr, map_length, map, compile_info);
}

// ── v1_1 events ──────────────────────────────────────────────────────

trampoline_jni! { fn tramp_resource_exhausted(
    flags: sys::JVMTI_RESOURCE_EXHAUSTED_FLAGS,
    reserved: *const c_void,
    description: *const c_char
) => resource_exhausted }

// ── v11 events ───────────────────────────────────────────────────────

trampoline_jni! { fn tramp_sampled_object_alloc(
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    object_class: jni_sys::jclass,
    size: jni_sys::jlong
) => sampled_object_alloc }

// ── v21 events ───────────────────────────────────────────────────────

trampoline_jni! { fn tramp_virtual_thread_start(thread: jni_sys::jobject) => virtual_thread_start }
trampoline_jni! { fn tramp_virtual_thread_end(thread: jni_sys::jobject) => virtual_thread_end }

// ── Installation ─────────────────────────────────────────────────────

/// RAII guard for an installed [`EventHandler`].
///
/// Dropping this clears the event callbacks and frees the handler from
/// environment-local storage.
pub struct InstalledHandler {
    env_raw: *mut sys::jvmtiEnv,
}

impl core::fmt::Debug for InstalledHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InstalledHandler").finish_non_exhaustive()
    }
}

impl Drop for InstalledHandler {
    fn drop(&mut self) {
        unsafe {
            let interface = *self.env_raw;

            // Clear event callbacks.
            let empty: sys::jvmtiEventCallbacks = core::mem::zeroed();
            ((*interface).v1.SetEventCallbacks)(
                self.env_raw,
                &empty as *const sys::jvmtiEventCallbacks,
                core::mem::size_of::<sys::jvmtiEventCallbacks>() as jni_sys::jint,
            );

            // Retrieve and drop the handler state.
            let mut data: *mut c_void = core::ptr::null_mut();
            ((*interface).v1.GetEnvironmentLocalStorage)(self.env_raw, &mut data);
            if !data.is_null() {
                let _ = Box::from_raw(data as *mut CallbackState);
                ((*interface).v1.SetEnvironmentLocalStorage)(self.env_raw, core::ptr::null());
            }
        }
    }
}

/// Installs an [`EventHandler`] on the JVMTI environment.
///
/// Stores the handler in environment-local storage and sets up C callback
/// trampolines for all events. You must still enable each desired event via
/// [`Env::set_event_notification_mode`](crate::Env::set_event_notification_mode).
///
/// Returns an [`InstalledHandler`] guard. Dropping it clears the callbacks
/// and frees the handler.
///
/// # Panics
///
/// Panics if environment-local storage is already in use (only one handler
/// per JVMTI environment).
pub(crate) fn install_handler<H: EventHandler>(
    env: &crate::Env<'_>,
    handler: H,
) -> crate::Result<InstalledHandler> {
    let state = Box::new(CallbackState {
        handler: Box::new(handler),
        vm: env.as_java_vm_raw(),
    });
    let state_ptr = Box::into_raw(state) as *const c_void;

    // Store in environment-local storage.
    unsafe {
        let raw = env.as_raw();
        let interface = *raw;
        let ret = ((*interface).v1.SetEnvironmentLocalStorage)(raw, state_ptr);
        if ret != sys::jvmtiError::JVMTI_ERROR_NONE {
            // Clean up on failure.
            let _ = Box::from_raw(state_ptr as *mut CallbackState);
            return Err(crate::JvmtiError::from(ret));
        }
    }

    // Build callbacks struct with all trampolines.
    let mut cb: sys::jvmtiEventCallbacks = unsafe { core::mem::zeroed() };

    // v1 callbacks
    cb.v1.VMInit = Some(tramp_vm_init);
    cb.v1.VMDeath = Some(tramp_vm_death);
    cb.v1.ThreadStart = Some(tramp_thread_start);
    cb.v1.ThreadEnd = Some(tramp_thread_end);
    cb.v1.ClassFileLoadHook = Some(tramp_class_file_load_hook);
    cb.v1.ClassLoad = Some(tramp_class_load);
    cb.v1.ClassPrepare = Some(tramp_class_prepare);
    cb.v1.VMStart = Some(tramp_vm_start);
    cb.v1.Exception = Some(tramp_exception);
    cb.v1.ExceptionCatch = Some(tramp_exception_catch);
    cb.v1.SingleStep = Some(tramp_single_step);
    cb.v1.FramePop = Some(tramp_frame_pop);
    cb.v1.Breakpoint = Some(tramp_breakpoint);
    cb.v1.FieldAccess = Some(tramp_field_access);
    cb.v1.FieldModification = Some(tramp_field_modification);
    cb.v1.MethodEntry = Some(tramp_method_entry);
    cb.v1.MethodExit = Some(tramp_method_exit);
    cb.v1.NativeMethodBind = Some(tramp_native_method_bind);
    cb.v1.CompiledMethodLoad = Some(tramp_compiled_method_load);
    cb.v1.CompiledMethodUnload = Some(tramp_compiled_method_unload);
    cb.v1.DynamicCodeGenerated = Some(tramp_dynamic_code_generated);
    cb.v1.DataDumpRequest = Some(tramp_data_dump_request);
    cb.v1.MonitorWait = Some(tramp_monitor_wait);
    cb.v1.MonitorWaited = Some(tramp_monitor_waited);
    cb.v1.MonitorContendedEnter = Some(tramp_monitor_contended_enter);
    cb.v1.MonitorContendedEntered = Some(tramp_monitor_contended_entered);
    cb.v1.GarbageCollectionStart = Some(tramp_gc_start);
    cb.v1.GarbageCollectionFinish = Some(tramp_gc_finish);
    cb.v1.ObjectFree = Some(tramp_object_free);
    cb.v1.VMObjectAlloc = Some(tramp_vm_object_alloc);

    // v1_1 callbacks
    cb.v1_1.ResourceExhausted = Some(tramp_resource_exhausted);

    // v11 callbacks
    cb.v11.SampledObjectAlloc = Some(tramp_sampled_object_alloc);

    // v21 callbacks
    cb.v21.VirtualThreadStart = Some(tramp_virtual_thread_start);
    cb.v21.VirtualThreadEnd = Some(tramp_virtual_thread_end);

    // Set the callbacks on the JVMTI environment.
    env.set_event_callbacks_raw(&cb)?;

    Ok(InstalledHandler {
        env_raw: env.as_raw(),
    })
}
