fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();

    if profile == "release" {
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
        println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE"); // or WINDOWS
        println!("cargo:rustc-link-lib=kernel32");
    }
}
