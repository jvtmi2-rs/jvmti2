//! Event callback system.
//!
//! Two APIs are provided:
//!
//! - **[`EventCallbacksBuilder`]** — raw fn-pointer API. Zero overhead,
//!   `unsafe`, full control.
//! - **[`EventHandler`]** trait — safe, trait-based API. Implement the trait
//!   and install it via [`Env::install_event_handler`](crate::Env::install_event_handler).
//!   Uses `dyn` dispatch (one vtable lookup per event invocation).

use core::ffi::{c_char, c_void, CStr};

use crate::objects::{jclass_from_raw, jobject_from_raw, JClass, JFieldID, JMethodID, JObject, JThread};
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
/// Most callbacks receive both a `jvmti_env` ([`Env`](crate::Env)) for JVMTI
/// operations and a `jni_env` ([`jni::EnvUnowned`]) for JNI calls. Use
/// [`jni_env.with_env(|env| { ... })`](jni::EnvUnowned::with_env) to access
/// the full [`jni::Env`] API inside the closure.
///
/// # Example
///
/// ```no_run
/// use jvmti2::{event::EventHandler, Env, JMethodID, JThread};
///
/// struct MyHandler;
///
/// impl EventHandler for MyHandler {
///     fn vm_init(&self, _jvmti_env: &Env<'_>, _jni_env: &mut jni::EnvUnowned<'_>,
///                _thread: &JThread<'_>) {
///         println!("VM initialized!");
///     }
/// }
/// ```
#[allow(unused_variables, clippy::too_many_arguments)]
pub trait EventHandler: Send + Sync + 'static {
    /// VM initialization complete.
    fn vm_init(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
    ) {
    }
    /// VM about to terminate.
    fn vm_death(&self, jvmti_env: &crate::Env<'_>, jni_env: &mut jni::EnvUnowned<'_>) {}
    /// A thread has started.
    fn thread_start(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
    ) {
    }
    /// A thread has ended.
    fn thread_end(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
    ) {
    }
    /// A class file is being loaded; the handler may transform the bytecode.
    ///
    /// Return `Some(bytes)` with replacement bytecode to transform the class,
    /// or `None` to leave it unchanged. The trampoline handles JVMTI memory
    /// allocation for the returned bytes.
    fn class_file_load_hook(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        class_being_redefined: Option<&JClass<'_>>,
        loader: Option<&JObject<'_>>,
        name: Option<&str>,
        protection_domain: &JObject<'_>,
        class_data: &[u8],
    ) -> Option<Vec<u8>> {
        None
    }
    /// A class has been loaded.
    fn class_load(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        klass: &JClass<'_>,
    ) {
    }
    /// A class has been prepared.
    fn class_prepare(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        klass: &JClass<'_>,
    ) {
    }
    /// VM has started (JNI available, but not fully initialized).
    fn vm_start(&self, jvmti_env: &crate::Env<'_>, jni_env: &mut jni::EnvUnowned<'_>) {}
    /// An exception was thrown.
    fn exception(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        location: sys::jlocation,
        exception: &JObject<'_>,
        catch_method: Option<JMethodID>,
        catch_location: sys::jlocation,
    ) {
    }
    /// An exception was caught.
    fn exception_catch(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        location: sys::jlocation,
        exception: Option<&JObject<'_>>,
    ) {
    }
    /// A single step event.
    fn single_step(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        location: sys::jlocation,
    ) {
    }
    /// A frame was popped.
    fn frame_pop(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        was_popped_by_exception: bool,
    ) {
    }
    /// A breakpoint was hit.
    fn breakpoint(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        location: sys::jlocation,
    ) {
    }
    /// A field was accessed (read).
    fn field_access(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        location: sys::jlocation,
        field_klass: &JClass<'_>,
        object: Option<&JObject<'_>>,
        field: JFieldID,
    ) {
    }
    /// A field was modified (written).
    fn field_modification(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        location: sys::jlocation,
        field_klass: &JClass<'_>,
        object: Option<&JObject<'_>>,
        field: JFieldID,
        signature_type: char,
        new_value: jni_sys::jvalue,
    ) {
    }
    /// Method was entered.
    fn method_entry(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
    ) {
    }
    /// Method was exited.
    fn method_exit(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        method: JMethodID,
        was_popped_by_exception: bool,
        return_value: jni_sys::jvalue,
    ) {
    }
    /// A native method is being bound to its implementation.
    ///
    /// Return `Some(address)` to redirect the native method to a different
    /// implementation, or `None` to leave it unchanged.
    ///
    /// `jni_env` may be `None` during the primordial phase before JNI is ready.
    fn native_method_bind(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: Option<&mut jni::EnvUnowned<'_>>,
        thread: &JThread<'_>,
        method: JMethodID,
        address: *mut c_void,
    ) -> Option<*mut c_void> {
        None
    }
    /// A compiled method has been loaded into memory.
    ///
    /// This event does not receive a JNI environment.
    fn compiled_method_load(
        &self,
        jvmti_env: &crate::Env<'_>,
        method: JMethodID,
        code: &[u8],
        map: Option<&[sys::jvmtiAddrLocationMap]>,
        compile_info: *const c_void,
    ) {
    }
    /// A compiled method has been unloaded from memory.
    ///
    /// This event does not receive a JNI environment.
    fn compiled_method_unload(
        &self,
        jvmti_env: &crate::Env<'_>,
        method: JMethodID,
        code_addr: *const c_void,
    ) {
    }
    /// Dynamically generated code was loaded.
    ///
    /// This event does not receive a JNI environment.
    fn dynamic_code_generated(
        &self,
        jvmti_env: &crate::Env<'_>,
        name: &str,
        address: *const c_void,
        length: jni_sys::jint,
    ) {
    }
    /// The VM received a data dump request (e.g., Ctrl+Break).
    ///
    /// This event does not receive a JNI environment.
    fn data_dump_request(&self, jvmti_env: &crate::Env<'_>) {}
    /// A thread is about to wait on a monitor.
    fn monitor_wait(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        object: &JObject<'_>,
        timeout: jni_sys::jlong,
    ) {
    }
    /// A thread finished waiting on a monitor.
    fn monitor_waited(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        object: &JObject<'_>,
        timed_out: bool,
    ) {
    }
    /// A thread is about to enter a contended monitor.
    fn monitor_contended_enter(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        object: &JObject<'_>,
    ) {
    }
    /// A thread entered a contended monitor.
    fn monitor_contended_entered(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        object: &JObject<'_>,
    ) {
    }
    /// A resource has been exhausted (OOM, thread limit, etc.).
    fn resource_exhausted(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        flags: crate::sys::JVMTI_RESOURCE_EXHAUSTED_FLAGS,
        description: &str,
    ) {
    }
    /// Garbage collection started.
    ///
    /// This event does not receive a JNI environment.
    fn garbage_collection_start(&self, jvmti_env: &crate::Env<'_>) {}
    /// Garbage collection finished.
    ///
    /// This event does not receive a JNI environment.
    fn garbage_collection_finish(&self, jvmti_env: &crate::Env<'_>) {}
    /// A tagged object was freed.
    ///
    /// This event does not receive a JNI environment.
    fn object_free(&self, jvmti_env: &crate::Env<'_>, tag: jni_sys::jlong) {}
    /// The VM allocated an object.
    fn vm_object_alloc(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        object: &JObject<'_>,
        object_klass: &JClass<'_>,
        size: jni_sys::jlong,
    ) {
    }
    /// A sampled object allocation occurred (JVMTI 11+).
    fn sampled_object_alloc(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
        object: &JObject<'_>,
        object_class: &JClass<'_>,
        size: jni_sys::jlong,
    ) {
    }
    /// A virtual thread started (JVMTI 21+).
    fn virtual_thread_start(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
    ) {
    }
    /// A virtual thread ended (JVMTI 21+).
    fn virtual_thread_end(
        &self,
        jvmti_env: &crate::Env<'_>,
        jni_env: &mut jni::EnvUnowned<'_>,
        thread: &JThread<'_>,
    ) {
    }
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

// ── Hand-written trampolines ─────────────────────────────────────────
//
// Each trampoline converts raw C types to idiomatic Rust types before
// calling the handler method.

unsafe extern "system" fn tramp_vm_init(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    state.handler.vm_init(&env, &mut jni_env_w, &thread_w);
}

unsafe extern "system" fn tramp_vm_death(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    state.handler.vm_death(&env, &mut jni_env_w);
}

unsafe extern "system" fn tramp_thread_start(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    state.handler.thread_start(&env, &mut jni_env_w, &thread_w);
}

unsafe extern "system" fn tramp_thread_end(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    state.handler.thread_end(&env, &mut jni_env_w, &thread_w);
}

unsafe extern "system" fn tramp_class_file_load_hook(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    class_being_redefined: jni_sys::jclass,
    loader: jni_sys::jobject,
    name: *const c_char,
    protection_domain: jni_sys::jobject,
    class_data_len: jni_sys::jint,
    class_data: *const core::ffi::c_uchar,
    new_class_data_len: *mut jni_sys::jint,
    new_class_data: *mut *mut core::ffi::c_uint,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };

    let class_redef = if class_being_redefined.is_null() {
        None
    } else {
        Some(unsafe { jclass_from_raw(class_being_redefined) })
    };
    let loader_w = if loader.is_null() {
        None
    } else {
        Some(unsafe { jobject_from_raw(loader) })
    };
    let name_str = if name.is_null() {
        None
    } else {
        let cstr = unsafe { CStr::from_ptr(name) };
        Some(cstr.to_str().unwrap_or("<invalid utf8>"))
    };
    let prot_domain = unsafe { jobject_from_raw(protection_domain) };
    let data = unsafe { core::slice::from_raw_parts(class_data, class_data_len as usize) };

    let result = state.handler.class_file_load_hook(
        &env,
        &mut jni_env_w,
        class_redef.as_ref(),
        loader_w.as_ref(),
        name_str,
        &prot_domain,
        data,
    );

    if let Some(new_bytes) = result {
        // Allocate JVMTI memory for the replacement bytecode.
        let interface = unsafe { *jvmti_env };
        let mut buf: *mut core::ffi::c_uchar = core::ptr::null_mut();
        let ret = unsafe {
            ((*interface).v1.Allocate)(
                jvmti_env,
                new_bytes.len() as jni_sys::jlong,
                &mut buf,
            )
        };
        if ret == sys::jvmtiError::JVMTI_ERROR_NONE && !buf.is_null() {
            unsafe {
                core::ptr::copy_nonoverlapping(new_bytes.as_ptr(), buf, new_bytes.len());
                *new_class_data_len = new_bytes.len() as jni_sys::jint;
                // NOTE: new_class_data is typed as *mut *mut c_uint due to jvmti2-sys bug
                // but should be *mut *mut c_uchar. Cast through.
                *(new_class_data as *mut *mut core::ffi::c_uchar) = buf;
            }
        }
    }
}

unsafe extern "system" fn tramp_class_load(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    klass: jni_sys::jclass,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let klass_w = unsafe { jclass_from_raw(klass) };
    state
        .handler
        .class_load(&env, &mut jni_env_w, &thread_w, &klass_w);
}

unsafe extern "system" fn tramp_class_prepare(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    klass: jni_sys::jclass,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let klass_w = unsafe { jclass_from_raw(klass) };
    state
        .handler
        .class_prepare(&env, &mut jni_env_w, &thread_w, &klass_w);
}

unsafe extern "system" fn tramp_vm_start(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    state.handler.vm_start(&env, &mut jni_env_w);
}

unsafe extern "system" fn tramp_exception(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    exception: jni_sys::jobject,
    catch_method: jni_sys::jmethodID,
    catch_location: sys::jlocation,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    let exception_w = unsafe { jobject_from_raw(exception) };
    let catch_method_w = if catch_method.is_null() {
        None
    } else {
        Some(unsafe { JMethodID::from_raw(catch_method) })
    };
    state.handler.exception(
        &env,
        &mut jni_env_w,
        &thread_w,
        method_w,
        location,
        &exception_w,
        catch_method_w,
        catch_location,
    );
}

unsafe extern "system" fn tramp_exception_catch(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    exception: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    let exception_w = if exception.is_null() {
        None
    } else {
        Some(unsafe { jobject_from_raw(exception) })
    };
    state.handler.exception_catch(
        &env,
        &mut jni_env_w,
        &thread_w,
        method_w,
        location,
        exception_w.as_ref(),
    );
}

unsafe extern "system" fn tramp_single_step(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    state
        .handler
        .single_step(&env, &mut jni_env_w, &thread_w, method_w, location);
}

unsafe extern "system" fn tramp_frame_pop(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    was_popped_by_exception: jni_sys::jboolean,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    state.handler.frame_pop(
        &env,
        &mut jni_env_w,
        &thread_w,
        method_w,
        was_popped_by_exception,
    );
}

unsafe extern "system" fn tramp_breakpoint(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    state
        .handler
        .breakpoint(&env, &mut jni_env_w, &thread_w, method_w, location);
}

unsafe extern "system" fn tramp_field_access(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    field_klass: jni_sys::jclass,
    object: jni_sys::jobject,
    field: jni_sys::jfieldID,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    let field_klass_w = unsafe { jclass_from_raw(field_klass) };
    let object_w = if object.is_null() {
        None
    } else {
        Some(unsafe { jobject_from_raw(object) })
    };
    let field_w = unsafe { JFieldID::from_raw(field) };
    state.handler.field_access(
        &env,
        &mut jni_env_w,
        &thread_w,
        method_w,
        location,
        &field_klass_w,
        object_w.as_ref(),
        field_w,
    );
}

unsafe extern "system" fn tramp_field_modification(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    location: sys::jlocation,
    field_klass: jni_sys::jclass,
    object: jni_sys::jobject,
    field: jni_sys::jfieldID,
    signature_type: c_char,
    new_value: jni_sys::jvalue,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    let field_klass_w = unsafe { jclass_from_raw(field_klass) };
    let object_w = if object.is_null() {
        None
    } else {
        Some(unsafe { jobject_from_raw(object) })
    };
    let field_w = unsafe { JFieldID::from_raw(field) };
    let sig_char = (signature_type as u8) as char;
    state.handler.field_modification(
        &env,
        &mut jni_env_w,
        &thread_w,
        method_w,
        location,
        &field_klass_w,
        object_w.as_ref(),
        field_w,
        sig_char,
        new_value,
    );
}

unsafe extern "system" fn tramp_method_entry(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    state
        .handler
        .method_entry(&env, &mut jni_env_w, &thread_w, method_w);
}

unsafe extern "system" fn tramp_method_exit(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    was_popped_by_exception: jni_sys::jboolean,
    return_value: jni_sys::jvalue,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    state.handler.method_exit(
        &env,
        &mut jni_env_w,
        &thread_w,
        method_w,
        was_popped_by_exception,
        return_value,
    );
}

unsafe extern "system" fn tramp_native_method_bind(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    method: jni_sys::jmethodID,
    address: *mut c_void,
    new_address_ptr: *mut *mut c_void,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_opt = if jni_env.is_null() {
        None
    } else {
        Some(unsafe { jni::EnvUnowned::from_raw(jni_env) })
    };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    let result = state.handler.native_method_bind(
        &env,
        jni_env_opt.as_mut(),
        &thread_w,
        method_w,
        address,
    );
    if let Some(new_addr) = result {
        unsafe { *new_address_ptr = new_addr };
    }
}

unsafe extern "system" fn tramp_compiled_method_load(
    jvmti_env: *mut sys::jvmtiEnv,
    // NOTE: Due to a jvmti2-sys bug, this second C parameter is typed as
    // `*mut JNIEnv` but is actually a `jmethodID`. We cast it here.
    method: jni_sys::jmethodID,
    code_size: jni_sys::jint,
    code_addr: *const c_void,
    map_length: jni_sys::jint,
    map: *const sys::jvmtiAddrLocationMap,
    compile_info: *const c_void,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    let code = unsafe { core::slice::from_raw_parts(code_addr as *const u8, code_size as usize) };
    let map_slice = if map.is_null() || map_length <= 0 {
        None
    } else {
        Some(unsafe { core::slice::from_raw_parts(map, map_length as usize) })
    };
    state
        .handler
        .compiled_method_load(&env, method_w, code, map_slice, compile_info);
}

unsafe extern "system" fn tramp_compiled_method_unload(
    jvmti_env: *mut sys::jvmtiEnv,
    method: jni_sys::jmethodID,
    code_addr: *const c_void,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let method_w = unsafe { JMethodID::from_raw(method) };
    state
        .handler
        .compiled_method_unload(&env, method_w, code_addr);
}

unsafe extern "system" fn tramp_dynamic_code_generated(
    jvmti_env: *mut sys::jvmtiEnv,
    name: *const c_char,
    address: *const c_void,
    length: jni_sys::jint,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let name_str = if name.is_null() {
        "<null>"
    } else {
        unsafe { CStr::from_ptr(name) }
            .to_str()
            .unwrap_or("<invalid utf8>")
    };
    state
        .handler
        .dynamic_code_generated(&env, name_str, address, length);
}

unsafe extern "system" fn tramp_data_dump_request(jvmti_env: *mut sys::jvmtiEnv) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    state.handler.data_dump_request(&env);
}

unsafe extern "system" fn tramp_monitor_wait(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    timeout: jni_sys::jlong,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let object_w = unsafe { jobject_from_raw(object) };
    state
        .handler
        .monitor_wait(&env, &mut jni_env_w, &thread_w, &object_w, timeout);
}

unsafe extern "system" fn tramp_monitor_waited(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    timed_out: jni_sys::jboolean,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let object_w = unsafe { jobject_from_raw(object) };
    state
        .handler
        .monitor_waited(&env, &mut jni_env_w, &thread_w, &object_w, timed_out);
}

unsafe extern "system" fn tramp_monitor_contended_enter(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let object_w = unsafe { jobject_from_raw(object) };
    state
        .handler
        .monitor_contended_enter(&env, &mut jni_env_w, &thread_w, &object_w);
}

unsafe extern "system" fn tramp_monitor_contended_entered(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let object_w = unsafe { jobject_from_raw(object) };
    state
        .handler
        .monitor_contended_entered(&env, &mut jni_env_w, &thread_w, &object_w);
}

unsafe extern "system" fn tramp_resource_exhausted(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    flags: sys::JVMTI_RESOURCE_EXHAUSTED_FLAGS,
    _reserved: *const c_void,
    description: *const c_char,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let desc = if description.is_null() {
        "<null>"
    } else {
        unsafe { CStr::from_ptr(description) }
            .to_str()
            .unwrap_or("<invalid utf8>")
    };
    state
        .handler
        .resource_exhausted(&env, &mut jni_env_w, flags, desc);
}

unsafe extern "system" fn tramp_gc_start(jvmti_env: *mut sys::jvmtiEnv) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    state.handler.garbage_collection_start(&env);
}

unsafe extern "system" fn tramp_gc_finish(jvmti_env: *mut sys::jvmtiEnv) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    state.handler.garbage_collection_finish(&env);
}

unsafe extern "system" fn tramp_object_free(
    jvmti_env: *mut sys::jvmtiEnv,
    tag: jni_sys::jlong,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    state.handler.object_free(&env, tag);
}

unsafe extern "system" fn tramp_vm_object_alloc(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    object_klass: jni_sys::jclass,
    size: jni_sys::jlong,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let object_w = unsafe { jobject_from_raw(object) };
    let klass_w = unsafe { jclass_from_raw(object_klass) };
    state.handler.vm_object_alloc(
        &env,
        &mut jni_env_w,
        &thread_w,
        &object_w,
        &klass_w,
        size,
    );
}

unsafe extern "system" fn tramp_sampled_object_alloc(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
    object: jni_sys::jobject,
    object_class: jni_sys::jclass,
    size: jni_sys::jlong,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    let object_w = unsafe { jobject_from_raw(object) };
    let klass_w = unsafe { jclass_from_raw(object_class) };
    state.handler.sampled_object_alloc(
        &env,
        &mut jni_env_w,
        &thread_w,
        &object_w,
        &klass_w,
        size,
    );
}

unsafe extern "system" fn tramp_virtual_thread_start(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    state
        .handler
        .virtual_thread_start(&env, &mut jni_env_w, &thread_w);
}

unsafe extern "system" fn tramp_virtual_thread_end(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut jni_sys::JNIEnv,
    thread: jni_sys::jobject,
) {
    let state = unsafe { &*get_callback_state(jvmti_env) };
    let env = unsafe { trampoline_env(jvmti_env, state) };
    let mut jni_env_w = unsafe { jni::EnvUnowned::from_raw(jni_env) };
    let thread_w = unsafe { JThread::from_raw(thread) };
    state
        .handler
        .virtual_thread_end(&env, &mut jni_env_w, &thread_w);
}

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
