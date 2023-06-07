extern crate bindgen;
use std::env;
use std::path::PathBuf;


fn main() {
    // let header_path = "bindings/wrapper.h";

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/home/nuc2kor/Desktop/devspace/test/Inteoperability/c_program");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=static=lib_registration");
    
    // println!("cargo:rustc-link-search=/home/nuc2kor/Desktop/devspace/test/Inteoperability/rust_prog/target/release/build/rust_prog-b89858f78142982d/out");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=bindings/wrapper.h");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("bindings/wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // // Specify the path to the C source file
    // let source_path = "../c_program/registration.c";

    // // Build the C source file into a static library using cc
    // cc::Build::new()
    //     .file(source_path)
    //     .compile("registration");

    

}