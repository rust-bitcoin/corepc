fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").ok();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").ok();

    if target_arch.as_deref() == Some("wasm32") && target_os.as_deref() == Some("unknown") {
        println!("cargo:rustc-cfg=bitreq_wasm");
    }
}
