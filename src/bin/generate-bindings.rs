//! UniFFI bindings generator utility
//!
//! Generates Python, Kotlin, and Swift bindings from the UDL file.

use std::process::Command;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let udl_file = format!("{}/src/magneto_serge.udl", manifest_dir);
    let lib_file = format!("{}/target/release/libmagneto_serge.dylib", manifest_dir);

    println!("🔗 Generating UniFFI bindings...");
    println!("UDL file: {}", udl_file);
    println!("Library: {}", lib_file);

    // Check if library exists
    if !std::path::Path::new(&lib_file).exists() {
        eprintln!("❌ Error: Library not found at {}", lib_file);
        eprintln!("Please run: cargo build --release --lib");
        std::process::exit(1);
    }

    let languages = vec!["python", "kotlin", "swift"];

    for lang in languages {
        println!("\n📦 Generating {} bindings...", lang);

        let output_dir = format!("{}/bindings/{}", manifest_dir, lang);
        std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");

        let status = Command::new("uniffi-bindgen")
            .arg("generate")
            .arg(&udl_file)
            .arg("--language")
            .arg(lang)
            .arg("--out-dir")
            .arg(&output_dir)
            .arg("--library")
            .arg(&lib_file)
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!(
                    "✅ {} bindings generated successfully in {}",
                    lang, output_dir
                );
            }
            Ok(exit_status) => {
                eprintln!(
                    "❌ Failed to generate {} bindings (exit code: {:?})",
                    lang,
                    exit_status.code()
                );
            }
            Err(e) => {
                eprintln!("❌ Error running uniffi-bindgen for {}: {}", lang, e);
                eprintln!("   Make sure uniffi-bindgen is installed:");
                eprintln!("   cargo install uniffi_bindgen_cli");
            }
        }
    }

    println!("\n✨ Done!");
}
