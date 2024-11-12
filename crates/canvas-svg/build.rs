use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

//const DEFAULT_CLANG_VERSION: &str = "14.0.7";
const DEFAULT_CLANG_VERSION: &str = "12.0.9";
fn main() {
    setup_aarch64_android_workaround();
    // setup_x86_64_android_workaround();
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/android.rs");
}


#[allow(dead_code)]
fn setup_x86_64_android_workaround() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");
    if target_arch == "x86_64" && target_os == "android" {
        println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");
        /*
        let android_ndk_home = if let Ok(android_ndk_home) = env::var("ANDROID_NDK") {
            android_ndk_home
        } else {
            env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME not set")
        };

        let build_os = match env::consts::OS {
            "linux" => "linux",
            "macos" => "darwin",
            "windows" => "windows",
            _ => panic!(
                "Unsupported OS. You must use either Linux, MacOS or Windows to build the crate."
            ),
        };

        let ndk_clang_version = if let Ok(mut android_version_txt) = File::open(format!(
            "{android_ndk_home}/toolchains/llvm/prebuilt/{build_os}-x86_64/AndroidVersion.txt"
        )) {
            let mut data = String::new();
            let _ = android_version_txt.read_to_string(&mut data);
            let line = data.lines().take(1).next();
            line.unwrap_or("").to_string()
        } else {
            DEFAULT_CLANG_VERSION.to_string()
        };

        let clang_version = env::var("NDK_CLANG_VERSION").unwrap_or(ndk_clang_version);

        let linux_x86_64_lib_dir = format!(
            "toolchains/llvm/prebuilt/{build_os}-x86_64/lib64/clang/{clang_version}/lib/linux/"
        );
        let linkpath = format!("{android_ndk_home}/{linux_x86_64_lib_dir}");
        if Path::new(&linkpath).exists() {
            println!("cargo:rustc-link-search={android_ndk_home}/{linux_x86_64_lib_dir}");
            println!("cargo:rustc-link-lib=static=clang_rt.builtins-x86_64-android");
        } else {
            panic!("Path {linkpath} not exists");
        }

        */
    }
}

fn setup_aarch64_android_workaround() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");
    if target_arch == "aarch64" && target_os == "android" {
        let android_ndk_home = if let Ok(android_ndk_home) = env::var("ANDROID_NDK") {
            android_ndk_home
        } else {
            env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME not set")
        };


        let build_os = match env::consts::OS {
            "linux" => "linux",
            "macos" => "darwin",
            "windows" => "windows",
            _ => panic!(
                "Unsupported OS. You must use either Linux, MacOS or Windows to build the crate."
            ),
        };

        let ndk_clang_version = if let Ok(mut android_version_txt) = File::open(format!(
            "{android_ndk_home}/toolchains/llvm/prebuilt/{build_os}-x86_64/AndroidVersion.txt"
        )) {
            let mut data = String::new();
            let _ = android_version_txt.read_to_string(&mut data);
            let line = data.lines().take(1).next();
            let mut line = line.unwrap_or("").to_string();
            // in ndk 27 the clang version is 18.0.1 but the directory is named 18 ... let tweak it
            if android_ndk_home.contains("27.0.12077973") {
                line = "18".to_string();
            }
            line
        } else {
            DEFAULT_CLANG_VERSION.to_string()
        };

        let clang_version = env::var("NDK_CLANG_VERSION").unwrap_or(ndk_clang_version);

        let linux_aarch64_lib_dir = format!(
            "toolchains/llvm/prebuilt/{build_os}-x86_64/lib64/clang/{clang_version}/lib/linux/"
        );

        let linkpath = format!("{android_ndk_home}/{linux_aarch64_lib_dir}");


        let linux_aarch64_lib_dir_other = format!(
            "toolchains/llvm/prebuilt/{build_os}-x86_64/lib/clang/{clang_version}/lib/linux/"
        );

        println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");

        let linkpath_other = format!("{android_ndk_home}/{linux_aarch64_lib_dir_other}");
        if Path::new(&linkpath).exists() {
            println!("cargo:rustc-link-search={android_ndk_home}/{linux_aarch64_lib_dir}");
            println!("cargo:rustc-link-lib=static=clang_rt.builtins-aarch64-android");
        }else if Path::new(&linkpath_other).exists(){
            println!("cargo:rustc-link-search={android_ndk_home}/{linux_aarch64_lib_dir_other}");
            println!("cargo:rustc-link-lib=static=clang_rt.builtins-aarch64-android");
        } else {
            panic!("Path {linkpath} not exists");
        }
    }
}
