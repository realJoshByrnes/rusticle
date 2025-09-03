# RusticleExec

**Minimal. Executable. Rusticle.**

RusticleExec is a byte-sized Windows executable written in pure Rust, designed to explore how small a functional `.exe` can be—without sacrificing clarity, correctness, or control. This project began with a standard `cargo new` application that printed `"Hello, world!"`, producing a 134,144-byte binary on Windows (Rust 1.88.0). Through a series of deliberate, incremental changes, we reduced that footprint dramatically—without introducing any dependencies or compromising readability.

Each commit in this repository represents a single optimization step, accompanied by a version bump and a clear explanation. From tuning compiler flags to removing the VC runtime, from stripping symbols to redefining the entry point, every decision was made with precision and purpose.

## 🧭 Guiding Principles

- ✅ Use the default Rust toolchain and linker
- ✅ Keep the code readable and maintainable
- ✅ Avoid all external dependencies
- ✅ Introduce features only when implemented
- ✅ Measure and document every change

---

## 📊 Version History & Binary Size

This table tracks the evolution of RusticleExec across versions, showing how each change impacted the final `.exe` size on both `x86_64` and `i686` architectures.

| Version | Change Summary                                    | `x86_64` Size (bytes) | `i686` Size (bytes) |
|---------|---------------------------------------------------|-----------------------|---------------------|
| v1.0.0  | Initial `cargo new` project with `println!`       | 138,240               | 117,760             |
| v1.1.0  | Built in release mode (`cargo build --release`)   | 134,144               | 116,224             |
| v1.2.0  | Enabled LTO for link-time optimization            | 122,368               | 105,472             |
| v1.3.0  | Switched panic strategy from unwind to abort      | 119,296               | 103,424             |
| v1.4.0  | Switched to opt-level = "z" for size optimization | 115,200               |  99,328             |
| v1.5.0  | Replaced main() with mainCRTStartup               | 105,984               |  91,136             |
| v1.6.0  | Switched to no_std w/ Win32 API for print + exit  |   3,072               |   3,072             |
| v1.7.0  | Set `/ALIGN:8` to minimize alignment overhead.    |   1,296               |   1,176             |

<!-- cargo clean; $targets = @("x86_64-pc-windows-msvc", "i686-pc-windows-msvc"); foreach ($t in $targets) { cargo build --release --target $t; $exe = "target\$t\release\rusticle.exe"; Write-Host "$t`t$($(Get-Item $exe).Length) bytes" } -->

> 📌 *All builds use the default Rust toolchain and linker. No external crates or dependencies are introduced at any stage.*

---

## 🚀 What RusticleExec Demonstrates

- How to build a minimal Windows executable in Rust
- How to progressively reduce binary size without sacrificing clarity
- How to remove runtime dependencies like the VC runtime
- How to control the PE layout and entry point manually
- How to write Rust that feels like shellcode—but compiles like a dream

## 📦 Final Outcome

The final binary is a fraction of its original size, with no external dependencies, no runtime, and no standard library. It executes cleanly, exits predictably, and leaves behind nothing but admiration for what Rust can do when stripped to its essence.

## 📄 License & Contributions

This project is **unlicensed**. You are free to use, modify, distribute, or embed it however you like—no restrictions, no attribution required.

**Pull requests are welcome.** Whether you're optimizing further, experimenting with new techniques, or just curious about byte-level Rust, feel free to contribute.