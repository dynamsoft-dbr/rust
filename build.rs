extern crate cc;
extern crate bindgen;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    
    // Get the output path
    let out_dir = env::var("OUT_DIR").unwrap();
    let package_offset = out_dir.find("package").unwrap_or(0);

    if package_offset == 0 {
        // Generates Rust FFI bindings.
        let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/reader.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/reader.rs")
            .expect("Couldn't write bindings!");
    }

    // Build C code.
    cc::Build::new()
        .include("include")
        .file("src/reader.c")
        .compile("reader");

    // Copy *.dll & .lib to the output path
    let dll_src: String = String::from("./platforms/win/DynamsoftBarcodeReaderx64.dll");
    let dll_dest_path = Path::new(&out_dir).join("DynamsoftBarcodeReaderx64.dll");
    let _dll_result = fs::copy(dll_src, dll_dest_path);

    let lib_src: String = String::from("./platforms/win/DBRx64.lib");
    let lib_dest_path = Path::new(&out_dir).join("DBRx64.lib");
    let _lib_result = fs::copy(lib_src, lib_dest_path);

    // Link Dynamsoft Barcode Reader.
    // println!("cargo:rustc-link-search={}", env!("DBR_LIB"));
    println!("cargo:rustc-link-search={}", &out_dir);
    println!("cargo:rustc-link-lib=DBRx64");
}
