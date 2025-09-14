fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();
    let bitness = std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os != "windows" {
        println!("cargo:warning=Unsupported operating system: {target_os}");
        panic!("This crate can only be built for Windows targets.");
    }

    match target_arch.as_str() {
        "x86" | "x86_64" => {
            // Supported architectures
        }
        _ => {
            // Note: aarch64 support is not implemented simply because I don't have the hardware.
            // Feel free to submit a PR if you'd like to add it!
            println!("cargo:warning=Unsupported architecture: {target_arch}");
            panic!("This crate currently supports only i686 and x86_64 architectures.");
        }
    }

    match target_env.as_str() {
        "msvc" => {
            // We don't use the Microsoft Visual C++ Redistributable, so we set our custom entry point
            println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
            // Force linker to use the console subsystem (Windows XP+)
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
        "gnu" => {
            // Skip crt0.o and related runtime setup
            println!("cargo:rustc-link-arg=-nostartfiles");
            // Force linker to use the console subsystem (Windows NT 4.0+)
            println!("cargo:rustc-link-arg=-Wl,--subsystem,console");

            if profile == "release" {
                println!("cargo:rustc-link-arg=-Wl,--strip-all");
            }
        }
        _ => {
            println!("cargo:warning=Unsupported toolchain: {target_env}");
            panic!("This crate currently supports only the MSVC and GNU Windows toolchains.");
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
