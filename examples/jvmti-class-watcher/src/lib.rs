//! JVMTI agent that monitors class loading.
//!
//! Logs every class load and class prepare event, showing the class name,
//! status flags, source file, method count, and class loader info.

use std::ffi::CStr;
use jvmti2::{agent_onload, Env, Event, EventHandler, EventMode, JClass, JThread};

struct ClassWatcherHandler;

impl EventHandler for ClassWatcherHandler {
    fn class_load(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: &mut jni::EnvUnowned<'_>,
        _thread: &JThread<'_>,
        klass: &JClass<'_>,
    ) {
        if let Ok((sig, _generic)) = jvmti_env.get_class_signature(klass) {
            let name = format_class_name(&sig.to_string_lossy());
            tracing::info!("[LOAD] {name}");
        }
    }

    fn class_prepare(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: &mut jni::EnvUnowned<'_>,
        _thread: &JThread<'_>,
        klass: &JClass<'_>,
    ) {
        let Ok((sig, _generic)) = jvmti_env.get_class_signature(klass) else { return };
        let name = format_class_name(&sig.to_string_lossy());
        let status = jvmti_env.get_class_status(klass).map(|s| format!("{s:?}")).unwrap_or_default();
        let methods = jvmti_env.get_class_methods(klass).map(|m| m.len()).unwrap_or(0);
        let source = jvmti_env.get_source_file_name(klass)
            .map(|s| s.to_string_lossy())
            .unwrap_or_else(|_| "<unknown>".into());
        let loader = match jvmti_env.get_class_loader(klass) {
            Ok(Some(loader)) => format!("{loader:?}"),
            Ok(None) => "bootstrap".into(),
            Err(_) => "<error>".into(),
        };
        tracing::info!("[PREPARED] {name} | status={status} methods={methods} source={source} loader={loader}");
    }
}

fn format_class_name(sig: &str) -> String {
    let s = sig.strip_prefix('L').unwrap_or(sig);
    let s = s.strip_suffix(';').unwrap_or(s);
    s.replace('/', ".")
}

fn on_load(env: &mut Env<'_>, _options: Option<&CStr>) -> jvmti2::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();
    tracing::info!("jvmti-class-watcher agent loaded");

    let handler = env.install_event_handler(ClassWatcherHandler)?;
    env.set_event_notification_mode(EventMode::Enable, Event::ClassLoad, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::ClassPrepare, None)?;
    std::mem::forget(handler);
    tracing::info!("class load/prepare monitoring enabled");
    Ok(())
}

agent_onload!(on_load);
