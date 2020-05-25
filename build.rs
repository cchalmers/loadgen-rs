use std::env;
use std::path::PathBuf;

fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("cbits/ccc.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

//     let bindings = bindgen::Builder::default()
//         .header("cbits/lg.hpp")
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         .whitelist_recursively(false)
//         .whitelist_function("mlperf::c.*")
//         .whitelist_type("mlperf::c::.*")
//         .whitelist_type("mlperf::TestSettings")
//         .whitelist_type("mlperf::LogSettings")
//         .whitelist_type("mlperf::LoggingMode")
//         .whitelist_type("mlperf::LogOutputSettings")
//         .new_type_alias("std::string")
//         .whitelist_type("std::string")
//         .opaque_type("std::string")
//         .no_copy("mlperf::LogOutputSettings")
//         .no_copy("mlperf::LogSettings")
//         // .default_alias_style(bindgen::AliasVariation::NewType)
//         // .opaque_type("mlperf::LogOutputSettings")
//         .whitelist_type("mlperf::QuerySample.*")
//         .whitelist_type("mlperf::Test.*")
//         .whitelist_type("mlperf::ResponseId")
//         .size_t_is_usize(true)
//         .ignore_methods()
//         .impl_debug(true)
//         .enable_cxx_namespaces()
//         .clang_arg("-x")
//         .clang_arg("c++")
//         .layout_tests(false)
//         // .rustified_non_exhaustive_enum("Test.*")
//         .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
//         .generate()
//         .expect("Unable to generate bindings");

//     // // Write the bindings to the $OUT_DIR/bindings.rs file.
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings
//         .write_to_file(out_path.join("bindings.rs"))
//         .expect("Couldn't write bindings!");

//     bindings
//         .write_to_file("src/bindings.rs")
//         .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=mlperf_loadgen");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cbits/ccc.h");
    println!("cargo:rerun-if-changed=cbits/ccc.cc");
}
