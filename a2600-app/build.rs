fn main() {
    if let Ok(target_os) = std::env::var("CARGO_CFG_TARGET_OS") {
        if target_os == "android" {
            println!("cargo:rustc-link-lib=c"); // memset, memcpy...
            println!("cargo:rustc-link-lib=android"); // ANativeWindow...
            println!("cargo:rustc-link-lib=log"); // __android_log_write
        }
    }
}
