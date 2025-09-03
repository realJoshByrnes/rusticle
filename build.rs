fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();

    println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
    println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE");
    println!("cargo:rustc-link-lib=kernel32");

    if profile == "release" {
        println!("cargo:rustc-link-arg=/ALIGN:8");
        println!("cargo:rustc-link-arg=/DEBUG:NONE");
        println!("cargo:rustc-link-arg=/SAFESEH:NO");
        println!("cargo:rustc-link-arg=/MERGE:.pdata=.text");
        println!("cargo:rustc-link-arg=/STUB:dos_stub.bin");
        println!("cargo:rustc-link-arg=/EMITTOOLVERSIONINFO:NO");
        println!("cargo:rustc-link-arg=/RELEASE");
        println!("cargo:rustc-link-arg=/EMITPOGOPHASEINFO");
    }
}
