//! JVMTI agent that traces all method entry/exit events.
//!
//! Build as a `cdylib` and load via `-agentpath:` JVM argument.

use std::ffi::CStr;

use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode, JMethodID, JThread};

/// Event handler that logs method entry and exit using `tracing`.
struct TracingHandler;

impl EventHandler for TracingHandler {
    fn method_entry(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: &mut jni::EnvUnowned<'_>,
        _thread: &JThread<'_>,
        method: JMethodID,
    ) {
        if let Ok(desc) = describe_method(jvmti_env, method) {
            tracing::info!(">> {desc}");
        }
    }

    fn method_exit(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: &mut jni::EnvUnowned<'_>,
        _thread: &JThread<'_>,
        method: JMethodID,
        _was_popped_by_exception: bool,
        _return_value: jni_sys::jvalue,
    ) {
        if let Ok(desc) = describe_method(jvmti_env, method) {
            tracing::info!("<< {desc}");
        }
    }
}

/// Build a human-readable `ClassName.methodName` string from a JMethodID.
fn describe_method(env: &Env<'_>, method: JMethodID) -> jvmti2::Result<String> {
    let (name, _sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(&klass)?;

    // class_sig is like "Ljava/util/ArrayList;" — strip L prefix and ; suffix,
    // and replace '/' with '.'
    let class_name = class_sig.to_string_lossy();
    let class_name = class_name
        .strip_prefix('L')
        .unwrap_or(&class_name);
    let class_name = class_name
        .strip_suffix(';')
        .unwrap_or(class_name)
        .replace('/', ".");

    Ok(format!("{class_name}.{name}"))
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

    tracing::info!("jvmti-tracing agent loaded");

    // Request capabilities for method entry/exit events.
    let caps = Capabilities::new()
        .can_generate_method_entry_events()
        .can_generate_method_exit_events();
    env.add_capabilities(&caps)?;

    // Install the tracing event handler.
    let handler = env.install_event_handler(TracingHandler)?;

    // Enable the events globally.
    env.set_event_notification_mode(EventMode::Enable, Event::MethodEntry, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::MethodExit, None)?;

    // Leak the guard so callbacks stay active for the lifetime of the JVM.
    std::mem::forget(handler);

    tracing::info!("method entry/exit tracing enabled");
    Ok(())
}

agent_onload!(on_load);
