// use std::process::Command;
//
// fn main() {
//     println!("cargo:rerun-if-changed=build.rs");
//     if cfg!(target_os = "macos") {
//         let linker = Command::new("cc").arg("--version").status();
//         if let Err(_) = linker {
//             eprintln!("Error: link to default cc linker.");
//         } else {
//             println!("cargo:rustc-link-arg-bins=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld.ld64.lld");
//         }
//     }
// }