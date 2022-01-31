
use std::path::PathBuf;
use std::process::Command;
use std::env;

use pkg_config;

fn fallback_clone_repository() {
    pkg_config::Config::new().statik(true).probe("uuid")
        .map_err(|x| {
            println!("cargo:warning=Static version of kernel library uuid must be installed");
            x
        }).unwrap();

    let cargo_output = PathBuf::from(env::var("OUT_DIR").unwrap());
    let chez_source_path = cargo_output.join("ChezScheme");
    Command::new("git")
        .current_dir(&cargo_output)
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--branch")
        .arg("v9.5.6")
        .arg("https://github.com/cisco/ChezScheme.git")
        .status()
        .expect("Failed to copy Chez Scheme source files");
    Command::new("./configure")
        .current_dir(&chez_source_path)
        .arg("--disable-curses")
        .arg("--disable-x11")
        .arg("-m=a6le")
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
    Command::new("mv")
        .current_dir(&chez_kernel)
        .arg("petite.boot")
        .arg("scheme.boot")
        .arg(&cargo_output)
        .status()
        .expect("Failed to move boot files");
    println!("cargo:rustc-env=PETITE_BOOT_PATH={}", cargo_output.join("petite.boot").display());
    println!("cargo:rustc-env=SCHEME_BOOT_PATH={}", cargo_output.join("scheme.boot").display());
    println!("cargo:rustc-link-search={}", chez_kernel.display());
    println!("cargo:rustc-link-lib=chez");
    println!("cargo:rustc-link-search={}", chez_lz4_lib.display());
    println!("cargo:rustc-link-lib=lz4");
    println!("cargo:rustc-link-search={}", chez_z_lib.display());
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:include={}", chez_output.display());
}

fn main() {
    // TODO: try to build a dynamic library from dev dependencies on system

    fallback_clone_repository();
}
