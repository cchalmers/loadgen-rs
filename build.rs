use std::env;
use std::path::PathBuf;

fn main() {
    // let bindings = bindgen::Builder::default()
    //     .header("cbits/lg.hpp")
    //     .whitelist_function("mlperf.*")
    //     .enable_cxx_namespaces()
    //     .clang_arg("-x")
    //     .clang_arg("c++")
    //     .generate()
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=mlperf_loadgen");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cbits/ccc.h");
    println!("cargo:rerun-if-changed=cbits/ccc.cc");
}
