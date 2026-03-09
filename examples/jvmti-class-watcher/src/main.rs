//! Exercises various class-loading paths to demonstrate the class watcher agent.

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
        println!("--- Loading ArrayList ---");
        env.find_class(jni_str!("java/util/ArrayList"))?;

        println!("--- Loading HashMap ---");
        env.find_class(jni_str!("java/util/HashMap"))?;

        println!("--- Loading Thread ---");
        env.find_class(jni_str!("java/lang/Thread"))?;

        println!("--- Reflective load: java.net.URL ---");
        let class_cls = env.find_class(jni_str!("java/lang/Class"))?;
        let url_name = env.new_string("java.net.URL")?;
        env.call_static_method(
            &class_cls,
            jni_str!("forName"),
            jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
            &[JValue::Object(&url_name)],
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
        "jvmti_class_watcher_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_class_watcher_agent.dylib"
    } else {
        "libjvmti_class_watcher_agent.so"
    };
    let path = dir.join(lib_name);
    assert!(path.exists(), "Agent library not found at {path:?}. Build first with `cargo build`.");
    path.to_string_lossy().into_owned()
}
