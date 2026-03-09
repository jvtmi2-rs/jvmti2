//! Exercises Java native methods to demonstrate the native spy agent.

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
        println!("--- System.currentTimeMillis() ---");
        let system = env.find_class(jni_str!("java/lang/System"))?;
        let time = env.call_static_method(
            &system,
            jni_str!("currentTimeMillis"),
            jni_sig!("()J"),
            &[],
        )?;
        println!("Time: {:?}", time.j());

        println!("--- System.arraycopy() ---");
        let src = env.new_int_array(4)?;
        let dst = env.new_int_array(4)?;
        env.call_static_method(
            &system,
            jni_str!("arraycopy"),
            jni_sig!("(Ljava/lang/Object;ILjava/lang/Object;II)V"),
            &[
                JValue::Object(&src),
                JValue::Int(0),
                JValue::Object(&dst),
                JValue::Int(0),
                JValue::Int(4),
            ],
        )?;

        println!("--- Thread.currentThread() ---");
        let thread_cls = env.find_class(jni_str!("java/lang/Thread"))?;
        env.call_static_method(
            &thread_cls,
            jni_str!("currentThread"),
            jni_sig!("()Ljava/lang/Thread;"),
            &[],
        )?;

        println!("--- Done ---");
        Ok(())
    })
    .expect("JNI call failed");
}

/// Locate the agent cdylib next to the runner binary.
///
/// Cargo places both artifacts in the same `target/{profile}/` directory,
/// so we look for the library right next to our own executable.
fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");

    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_native_spy_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_native_spy_agent.dylib"
    } else {
        "libjvmti_native_spy_agent.so"
    };

    let path = dir.join(lib_name);
    assert!(
        path.exists(),
        "Agent library not found at {path:?}. Build first with `cargo build`."
    );

    path.to_string_lossy().into_owned()
}
