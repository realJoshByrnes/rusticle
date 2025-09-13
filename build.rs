fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();
    let bitness = std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();

    // We don't use the Microsoft Visual C++ Redistributable, so we set our custom entry point
    println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
    // We set our compatibility with Windows XP+
    println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE,5.01");

    if profile == "release" {
        if bitness == "32" {
            println!("cargo:rustc-link-arg=/ALIGN:4"); // i686
        } else {
            println!("cargo:rustc-link-arg=/ALIGN:8"); // x86_64
        }
        println!("cargo:rustc-link-arg=/DEBUG:NONE"); // Both
        println!("cargo:rustc-link-arg=/MERGE:.pdata=.text"); // x86_64
        println!("cargo:rustc-link-arg=/STUB:dos_stub.bin"); // Both
        println!("cargo:rustc-link-arg=/EMITTOOLVERSIONINFO:NO"); // Both
        println!("cargo:rustc-link-arg=/EMITPOGOPHASEINFO"); // Both
        println!("cargo:rustc-link-arg=/FIXED"); // i686
    }
}
