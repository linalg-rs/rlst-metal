// build.rs

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    cc::Build::new()
        .file("metal/rlst_metal.m")
        .compile("rlst_metal");

    let sdk_path = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk";

    let bindings = bindgen::Builder::default()
        .header("metal/rlst_metal.h")
        .clang_args(["-x", "objective-c"])
        .clang_args(&["-isysroot", sdk_path])
        .allowlist_function("rlst.*")
        .allowlist_type("RLST.*")
        .allowlist_var("RLST.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir.clone());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.");

    println!("cargo:warning={}", out_dir);
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=rlst_metal");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
    println!("cargo:rerun-if-changed=src/rlst_metal.m");
}
