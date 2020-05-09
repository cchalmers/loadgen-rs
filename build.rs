use std::env;
use std::path::PathBuf;

fn main() {
    // cxx_build::bridge("src/main.rs")
    //     .file("cbits/ccc.cc")
    //     .cpp(true)
    //     .flag("-std=c++11")
    //     .compile("cxxbridge-demo");

    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("cbits/lg.hpp")
    //     .clang_arg("-std=c++11")
    //     // .clang_arg("-stdlib=libc++")
    //     // .clang_arg(format!("-I{}/include", "/nix/store/zz745gvx9npx24qgbjrm7s4dz9h1fhh2-clang-7.1.0/lib/clang/7.1.0"))
    //     // .clang_arg(format!("-I{}", "/nix/store/qxjwahr99kk0fmp37dm25185m9ngkfls-libc++-8.0.1/include/c++/v1"))
    //     .clang_arg(format!("-I{}/include", env::var("LOADGEN_PATH").unwrap()))
    //     .enable_cxx_namespaces()
    //     .clang_arg("-x c++")
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // bindings
    // //     .write_to_file("bindings.rs")
    // //     .expect("Couldn't write bindings!");

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
