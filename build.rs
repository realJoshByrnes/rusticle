fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();

    if profile == "release" {
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
        println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE");
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-arg=/ALIGN:8");
    }
}
