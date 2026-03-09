//! Exercises security-interesting Java APIs to demonstrate the recon agent.

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
        // --- CRYPTO: MessageDigest.getInstance("SHA-256") ---
        println!("=== CRYPTO ===");
        let md_cls = env.find_class(jni_str!("java/security/MessageDigest"))?;
        let algo = env.new_string("SHA-256")?;
        let md = env.call_static_method(
            &md_cls,
            jni_str!("getInstance"),
            jni_sig!("(Ljava/lang/String;)Ljava/security/MessageDigest;"),
            &[JValue::Object(&algo)],
        )?.l()?;
        let data = env.new_string("hello")?;
        let bytes = env.call_method(
            &data,
            jni_str!("getBytes"),
            jni_sig!("()[B"),
            &[],
        )?.l()?;
        env.call_method(
            &md,
            jni_str!("update"),
            jni_sig!("([B)V"),
            &[JValue::Object(&bytes)],
        )?;
        println!("  MessageDigest SHA-256 updated");

        // --- NET: new URL("https://example.com") ---
        println!("=== NET ===");
        let url_cls = env.find_class(jni_str!("java/net/URL"))?;
        let url_str = env.new_string("https://example.com")?;
        env.new_object(
            &url_cls,
            jni_sig!("(Ljava/lang/String;)V"),
            &[JValue::Object(&url_str)],
        )?;
        println!("  URL object created");

        // --- FILE: new File("/tmp/test").exists() ---
        println!("=== FILE ===");
        let file_cls = env.find_class(jni_str!("java/io/File"))?;
        let path = env.new_string("/tmp/test")?;
        let file = env.new_object(
            &file_cls,
            jni_sig!("(Ljava/lang/String;)V"),
            &[JValue::Object(&path)],
        )?;
        let exists = env.call_method(
            &file,
            jni_str!("exists"),
            jni_sig!("()Z"),
            &[],
        )?.z()?;
        println!("  File exists: {exists}");

        // --- REFLECT: Class.forName("java.util.HashMap") ---
        println!("=== REFLECT ===");
        let class_cls = env.find_class(jni_str!("java/lang/Class"))?;
        let class_name = env.new_string("java.util.HashMap")?;
        env.call_static_method(
            &class_cls,
            jni_str!("forName"),
            jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
            &[JValue::Object(&class_name)],
        )?;
        println!("  Class.forName loaded HashMap");

        // --- EXEC: Runtime.getRuntime() ---
        println!("=== EXEC ===");
        let runtime_cls = env.find_class(jni_str!("java/lang/Runtime"))?;
        env.call_static_method(
            &runtime_cls,
            jni_str!("getRuntime"),
            jni_sig!("()Ljava/lang/Runtime;"),
            &[],
        )?;
        println!("  Runtime.getRuntime() called");

        println!("=== RECON COMPLETE ===");
        Ok(())
    })
    .expect("JNI call failed");
}

fn find_agent_library() -> String {
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let dir = exe.parent().expect("Exe has no parent directory");
    let lib_name = if cfg!(target_os = "windows") {
        "jvmti_android_recon_agent.dll"
    } else if cfg!(target_os = "macos") {
        "libjvmti_android_recon_agent.dylib"
    } else {
        "libjvmti_android_recon_agent.so"
    };
    let path = dir.join(lib_name);
    assert!(path.exists(), "Agent library not found at {path:?}. Build first with `cargo build`.");
    path.to_string_lossy().into_owned()
}
