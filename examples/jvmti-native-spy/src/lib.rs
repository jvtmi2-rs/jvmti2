//! JVMTI agent that watches native method binding.
//!
//! Logs every native method bind event, showing which Java native method
//! is being linked to which native function address.

use std::ffi::{c_void, CStr};

use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode, JMethodID, JThread};

/// Event handler that logs native method bind events using `tracing`.
struct NativeSpyHandler;

impl EventHandler for NativeSpyHandler {
    fn native_method_bind(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: Option<&mut jni::EnvUnowned<'_>>,
        _thread: &JThread<'_>,
        method: JMethodID,
        address: *mut c_void,
    ) -> Option<*mut c_void> {
        if let Ok(desc) = describe_method(jvmti_env, method) {
            tracing::info!("[BIND] {desc} -> {address:?}");
        }
        None
    }
}

/// Build a human-readable `ClassName.methodName(sig)` string from a JMethodID.
fn describe_method(env: &Env<'_>, method: JMethodID) -> jvmti2::Result<String> {
    let (name, sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(&klass)?;

    // class_sig is like "Ljava/util/ArrayList;" — strip L prefix and ; suffix,
    // and replace '/' with '.'
    let class_name = class_sig.to_string_lossy();
    let class_name = class_name.strip_prefix('L').unwrap_or(&class_name);
    let class_name = class_name
        .strip_suffix(';')
        .unwrap_or(class_name)
        .replace('/', ".");

    Ok(format!("{class_name}.{name}{sig}"))
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    // Initialise tracing output.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    tracing::info!("jvmti-native-spy agent loaded");

    // Request capability for native method bind events.
    let caps = Capabilities::new().can_generate_native_method_bind_events();
    env.add_capabilities(&caps)?;

    // Install the event handler.
    let handler = env.install_event_handler(NativeSpyHandler)?;

    // Enable native method bind events globally.
    env.set_event_notification_mode(EventMode::Enable, Event::NativeMethodBind, None)?;

    // Leak the guard so callbacks stay active for the lifetime of the JVM.
    std::mem::forget(handler);

    tracing::info!("native method bind monitoring enabled");
    Ok(())
}

agent_onload!(on_load);
