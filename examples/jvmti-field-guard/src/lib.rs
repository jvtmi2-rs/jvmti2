//! JVMTI agent that guards field access on sensitive static fields.
//!
//! Sets access watches on `System.out` and `System.err`, logging every
//! read with the calling method and thread.

use std::ffi::CStr;

use jvmti2::{
    agent_onload, Capabilities, Env, Event, EventHandler, EventMode,
    JClass, JFieldID, JMethodID, JObject, JThread,
};

struct FieldGuardHandler;

impl EventHandler for FieldGuardHandler {
    fn vm_init(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: &mut jni::EnvUnowned<'_>,
        _thread: &JThread<'_>,
    ) {
        if let Err(e) = setup_watches(jvmti_env) {
            tracing::error!("Failed to set field watches: {e}");
        }
    }

    fn field_access(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: &mut jni::EnvUnowned<'_>,
        _thread: &JThread<'_>,
        method: JMethodID,
        _location: jvmti2::sys::jlocation,
        field_klass: &JClass<'_>,
        _object: Option<&JObject<'_>>,
        field: JFieldID,
    ) {
        let field_desc = describe_field(jvmti_env, field_klass, field);
        let caller = if method.into_raw().is_null() {
            "<unknown>".into()
        } else {
            describe_method(jvmti_env, method).unwrap_or_else(|_| "<unknown>".into())
        };
        tracing::warn!("[ACCESS] {field_desc} read by {caller}");
    }

    fn field_modification(
        &self,
        jvmti_env: &Env<'_>,
        _jni_env: &mut jni::EnvUnowned<'_>,
        _thread: &JThread<'_>,
        method: JMethodID,
        _location: jvmti2::sys::jlocation,
        field_klass: &JClass<'_>,
        _object: Option<&JObject<'_>>,
        field: JFieldID,
        _signature_type: char,
        _new_value: jni_sys::jvalue,
    ) {
        let field_desc = describe_field(jvmti_env, field_klass, field);
        let caller = if method.into_raw().is_null() {
            "<unknown>".into()
        } else {
            describe_method(jvmti_env, method).unwrap_or_else(|_| "<unknown>".into())
        };
        tracing::error!("[MODIFY] {field_desc} written by {caller}");
    }
}

fn setup_watches(env: &Env<'_>) -> jvmti2::Result<()> {
    let classes = env.get_loaded_classes()?;
    for &klass_raw in classes.as_slice() {
        let klass = unsafe { jvmti2::jclass_from_raw(klass_raw) };
        let (sig, _) = env.get_class_signature(&klass)?;
        if sig.to_string_lossy() == "Ljava/lang/System;" {
            let fields = env.get_class_fields(&klass)?;
            for &fid_raw in fields.as_slice() {
                let fid = unsafe { JFieldID::from_raw(fid_raw) };
                let (name, _, _) = env.get_field_name(&klass, fid)?;
                let name_str = name.to_string_lossy();
                if name_str == "out" || name_str == "err" {
                    tracing::info!("Setting access watch on System.{name_str}");
                    env.set_field_access_watch(&klass, fid)?;
                }
            }
            break;
        }
    }
    Ok(())
}

fn describe_field(env: &Env<'_>, klass: &JClass<'_>, field: JFieldID) -> String {
    let class_name = env
        .get_class_signature(klass)
        .map(|(sig, _)| format_class_name(&sig.to_string_lossy()))
        .unwrap_or_else(|_| "<unknown>".into());
    let field_name = env
        .get_field_name(klass, field)
        .map(|(name, _, _)| name.to_string_lossy())
        .unwrap_or_else(|_| "<unknown>".into());
    format!("{class_name}.{field_name}")
}

fn describe_method(env: &Env<'_>, method: JMethodID) -> jvmti2::Result<String> {
    let (name, _sig, _generic) = env.get_method_name(method)?;
    let klass = env.get_method_declaring_class(method)?;
    let (class_sig, _generic) = env.get_class_signature(&klass)?;
    let class_name = format_class_name(&class_sig.to_string_lossy());
    Ok(format!("{class_name}.{name}"))
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

    tracing::info!("jvmti-field-guard agent loaded");

    let caps = Capabilities::new()
        .can_generate_field_access_events()
        .can_generate_field_modification_events();
    env.add_capabilities(&caps)?;

    let handler = env.install_event_handler(FieldGuardHandler)?;

    env.set_event_notification_mode(EventMode::Enable, Event::VmInit, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::FieldAccess, None)?;
    env.set_event_notification_mode(EventMode::Enable, Event::FieldModification, None)?;

    std::mem::forget(handler);
    tracing::info!("field guard enabled (watches set on VMInit)");
    Ok(())
}

agent_onload!(on_load);
