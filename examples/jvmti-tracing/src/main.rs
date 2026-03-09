//! Creates a JVM with the tracing agent loaded and runs a simple Java program.
//!
//! The agent (built as `jvmti_tracing_agent.dll` / `.so` / `.dylib`) is
//! attached via `-agentpath:`, so JVMTI method-entry and method-exit events
//! are printed to the console as the Java code executes.

use jni::{
    jni_sig, jni_str,
    objects::{JObject, JValue},
    InitArgsBuilder, JavaVM,
};

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
        let arraylist = create_arraylist(env)?;
        println!("ArrayList created: {arraylist:?}");
        Ok(())
    })
    .expect("JNI call failed");
}

fn create_arraylist<'a>(env: &mut jni::Env<'a>) -> jni::errors::Result<JObject<'a>> {
    let clazz = env.find_class(jni_str!("java/util/ArrayList"))?;
    let arraylist = env.new_object(&clazz, jni_sig!("()V"), &[])?;

    let hello1 = env.new_string("hello element")?;
    let hello2 = env.new_string("hello second element")?;

    for hello in [hello1, hello2] {
        env.call_method(
            &arraylist,
            jni_str!("add"),
            jni_sig!("(Ljava/lang/Object;)Z"),
            &[JValue::Object(&hello)],
        )?;
    }

    // Bonus: System.out.println(arraylist)
    let system = env.find_class(jni_str!("java/lang/System"))?;
    let out = env
        .get_static_field(system, jni_str!("out"), jni_sig!("Ljava/io/PrintStream;"))?
        .l()?;
    env.call_method(
        out,
        jni_str!("println"),
        jni_sig!("(Ljava/lang/Object;)V"),
        &[JValue::Object(&arraylist)],
    )?;

    Ok(arraylist)
}

/// Locate the agent cdylib next to the runner binary.
///
/// Cargo places both artifacts in the same `target/{profile}/` directory,
/// so we look for the library right next to our own executable.
fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");

    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_tracing_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_tracing_agent.dylib"
    } else {
        "libjvmti_tracing_agent.so"
    };

    let path = dir.join(lib_name);
    assert!(
        path.exists(),
        "Agent library not found at {path:?}. Build the cdylib first with `cargo build`."
    );

    path.to_string_lossy().into_owned()
}
