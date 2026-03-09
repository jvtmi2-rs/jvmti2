//! Accesses System.out and System.err to trigger field guard events.

use jni::{jni_sig, jni_str, objects::JValue, InitArgsBuilder, JavaVM};

fn main() {
    let agent_path = find_agent_library();
    println!("Loading agent from: {agent_path}");

    let agent_opt = format!("-agentpath:{agent_path}");
    let args = InitArgsBuilder::new()
        .option(&agent_opt)
        .build()
        .expect("Failed to build JVM init args");

    let jvm = JavaVM::new(args).expect("Failed to create JVM");

    jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
        println!("--- Accessing System.out ---");
        let system = env.find_class(jni_str!("java/lang/System"))?;
        let out = env
            .get_static_field(&system, jni_str!("out"), jni_sig!("Ljava/io/PrintStream;"))?
            .l()?;

        let msg = env.new_string("Hello from field-guard runner!")?;
        env.call_method(
            out,
            jni_str!("println"),
            jni_sig!("(Ljava/lang/Object;)V"),
            &[JValue::Object(&msg)],
        )?;

        println!("--- Accessing System.err ---");
        let err = env
            .get_static_field(&system, jni_str!("err"), jni_sig!("Ljava/io/PrintStream;"))?
            .l()?;

        let msg2 = env.new_string("Hello from System.err!")?;
        env.call_method(
            err,
            jni_str!("println"),
            jni_sig!("(Ljava/lang/Object;)V"),
            &[JValue::Object(&msg2)],
        )?;

        println!("--- Done ---");
        Ok(())
    })
    .expect("JNI call failed");
}

fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");
    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_field_guard_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_field_guard_agent.dylib"
    } else {
        "libjvmti_field_guard_agent.so"
    };
    let path = dir.join(lib_name);
    assert!(
        path.exists(),
        "Agent library not found at {path:?}. Build first with `cargo build`."
    );
    path.to_string_lossy().into_owned()
}
