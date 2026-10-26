use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=swift-bridge");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=SDKROOT");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rustc-link-lib=compression");
    println!("cargo:rustc-link-lib=AppleArchive");

    let swift_dir = "swift-bridge";
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let swift_build_dir = format!("{out_dir}/swift-build");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => {
            panic!("compression-rs: unsupported target arch '{other}'. Expected x86_64 or aarch64.")
        }
    };

    let mut swift_build = Command::new("swift");
    swift_build
        .args([
            "build",
            "-c",
            "release",
            "--triple",
            swift_triple,
            "--package-path",
            swift_dir,
            "--scratch-path",
            &swift_build_dir,
        ])
        .args(["--jobs", "4"]);
    let output = swift_build.output().expect("failed to build Swift bridge");

    assert!(
        output.status.success(),
        "Swift build failed ({}):\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let output = swift_build
        .arg("--show-bin-path")
        .output()
        .expect("failed to query the Swift product path");
    assert!(
        output.status.success(),
        "Swift product path query failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let swift_bin_dir = String::from_utf8(output.stdout).expect("Swift product path must be UTF-8");
    let swift_bin_dir = swift_bin_dir.trim();
    assert!(!swift_bin_dir.is_empty(), "Swift product path is empty");

    println!("cargo:rustc-link-search=native={swift_bin_dir}");
    println!("cargo:rustc-link-lib=static=CompressionBridge");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    match Command::new("xcode-select").arg("-p").output() {
        Ok(output) if output.status.success() => {
            let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let swift_lib_path =
                format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
            if Path::new(&swift_lib_path).exists() {
                println!("cargo:rustc-link-search=native={swift_lib_path}");
                println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_lib_path}");
            }
        }
        Ok(output) => {
            println!(
                "cargo:warning=`xcode-select -p` exited non-zero (status={:?}); Swift runtime rpath not added",
                output.status.code()
            );
        }
        Err(error) => {
            println!(
                "cargo:warning=`xcode-select` could not be invoked ({error}); Swift runtime rpath not added"
            );
        }
    }
}
