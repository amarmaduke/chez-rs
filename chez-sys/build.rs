
use std::path::PathBuf;
use std::process::Command;
use std::env;

fn main() {
    let cargo_manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let chez_source_path = PathBuf::from(cargo_manifest).join("ChezScheme");
    let cargo_output = env::var("OUT_DIR").unwrap();
    Command::new("./configure")
        .current_dir(&chez_source_path)
        .arg("--disable-curses")
        .arg("--disable-x11")
        .arg("--libkernel")
        .status()
        .expect("Configuration failed!");
    Command::new("make")
        .current_dir(&chez_source_path)
        .status()
        .expect("Building failed!");
    let chez_output = chez_source_path.join("a6le");
    let chez_kernel = chez_output.join("boot/a6le");
    let chez_lz4_lib = chez_output.join("lz4/lib");
    let chez_z_lib = chez_output.join("zlib");
    Command::new("mv")
        .current_dir(&chez_kernel)
        .arg("libkernel.a")
        .arg("libchez.a")
        .status()
        .expect("Failed to rename library.");
    println!("cargo:rustc-link-search={}", chez_kernel.display());
    println!("cargo:rustc-link-lib=chez");
    println!("cargo:rustc-link-search={}", chez_lz4_lib.display());
    println!("cargo:rustc-link-lib=lz4");
    println!("cargo:rustc-link-search={}", chez_z_lib.display());
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:include={}", chez_output.display());
}