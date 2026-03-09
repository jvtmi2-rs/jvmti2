//! JVMTI agent for attack surface reconnaissance.
//!
//! Monitors method entry events and native method binds, filtering for
//! security-interesting API calls: crypto, networking, file I/O, reflection,
//! process execution, and dynamic class loading.
//!
//! Designed for both standard JVM and Android ART (JVMTI supported since
//! Android 8 / API 26).

use std::ffi::{c_void, CStr};

use jvmti2::{agent_onload, Capabilities, Env, Event, EventHandler, EventMode};

/// Security-interesting class prefixes and their category tags.
const WATCH_PREFIXES: &[(&str, &str)] = &[
    ("javax.crypto.", "CRYPTO"),
    ("java.security.", "SECURITY"),
    ("java.net.", "NET"),
    ("javax.net.", "NET"),
    ("java.io.File", "FILE"),
    ("java.nio.file.", "FILE"),
    ("java.lang.reflect.", "REFLECT"),
    ("java.lang.Class.forName", "DYNLOAD"),
    ("java.lang.Class.getMethod", "REFLECT"),
    ("java.lang.Runtime.exec", "EXEC"),
    ("java.lang.Runtime.getRuntime", "EXEC"),
    ("java.lang.ProcessBuilder", "EXEC"),
    // Android-specific (no-op on standard JVM, ready for ART):
    ("dalvik.system.DexClassLoader", "DYNLOAD"),
    ("dalvik.system.PathClassLoader", "DYNLOAD"),
    ("android.content.Intent", "IPC"),
    ("android.os.Binder", "IPC"),
    ("android.app.Activity.startActivity", "IPC"),
];

struct ReconHandler;

impl EventHandler for ReconHandler {
    fn method_entry(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
    ) {
        let Ok(full_name) = describe_method(env, method) else { return };

        for &(prefix, tag) in WATCH_PREFIXES {
            if full_name.starts_with(prefix) {
                tracing::warn!("[{tag}] >> {full_name}");
                return;
            }
        }
    }

    fn native_method_bind(
        &self,
        env: &Env<'_>,
        _thread: jni_sys::jobject,
        method: jni_sys::jmethodID,
        address: *mut c_void,
        _new_address_ptr: *mut *mut c_void,
    ) {
        if let Ok(desc) = describe_method(env, method) {
            tracing::info!("[NATIVE] {desc} -> {address:?}");
        }
    }
}

fn describe_method(env: &Env<'_>, method: jni_sys::jmethodID) -> jvmti2::Result<String> {
    let (name, _sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(klass)?;
    let class_name = class_sig.to_string_lossy();
    let class_name = class_name.strip_prefix('L').unwrap_or(&class_name);
    let class_name = class_name.strip_suffix(';').unwrap_or(class_name).replace('/', ".");
    Ok(format!("{class_name}.{name}"))
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    tracing::info!("jvmti-android-recon agent loaded");
    tracing::info!("Watching {} security-interesting prefixes", WATCH_PREFIXES.len());

    let caps = Capabilities::new()
        .can_generate_method_entry_events()
        .can_generate_native_method_bind_events();
    env.add_capabilities(&caps)?;

    let handler = env.install_event_handler(ReconHandler)?;

    env.set_event_notification_mode(EventMode::Enable, Event::MethodEntry, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::NativeMethodBind, None)?;

    std::mem::forget(handler);
    tracing::info!("attack surface reconnaissance enabled");
    Ok(())
}

agent_onload!(on_load);
