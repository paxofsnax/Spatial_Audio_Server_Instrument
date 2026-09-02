// Antopia fork: no bindgen at build time. The upstream build.rs generates
// bindings with bindgen 0.32, which crashes against the modern libclang on
// this machine. Instead we ship pre-generated bindings (created with
// bindgen 0.69 --with-derive-default against the MacOSX11.3.sdk) and simply
// copy them into OUT_DIR.

use std::env;
use std::path::PathBuf;

fn frameworks_path() -> String {
    let output = std::process::Command::new("xcode-select")
        .arg("-p")
        .output()
        .expect("could not run xcode-select")
        .stdout;
    let prefix = std::str::from_utf8(&output)
        .expect("invalid output from xcode-select")
        .trim_right();

    let infix = if prefix == "/Library/Developer/CommandLineTools" {
        "SDKs/MacOSX11.3.sdk".to_string()
    } else {
        "Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk".to_string()
    };

    format!("{}/{}/System/Library/Frameworks", prefix, infix)
}

fn build(frameworks_path: &str) {
    // Only link to each framework if their features are enabled.
    #[cfg(feature = "audio_toolbox")]
    println!("cargo:rustc-link-lib=framework=AudioToolbox");

    #[cfg(feature = "audio_unit")]
    println!("cargo:rustc-link-lib=framework=AudioUnit");

    #[cfg(feature = "core_audio")]
    println!("cargo:rustc-link-lib=framework=CoreAudio");

    #[cfg(feature = "open_al")]
    println!("cargo:rustc-link-lib=framework=OpenAL");

    #[cfg(all(feature = "core_midi", target_os = "macos"))]
    println!("cargo:rustc-link-lib=framework=CoreMIDI");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    std::fs::copy(
        manifest_dir.join("pregenerated-bindings.rs"),
        out_dir.join("coreaudio.rs"),
    )
    .expect("could not copy pregenerated bindings");
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    build(&frameworks_path());
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    eprintln!("coreaudio-sys requires macos or ios target");
}
