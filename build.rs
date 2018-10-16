extern crate cc;
extern crate bindgen;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    

    // Build C code.
    cc::Build::new()
        .include("include")
        .file("src/reader.c")
        .compile("reader");

    // Get the output path
    let out_dir = env::var("OUT_DIR").unwrap();

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