use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("cbits/lg.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_recursively(false)
        .whitelist_function("mlperf::c.*")
        .whitelist_type("mlperf::c::.*")
        .whitelist_type("mlperf::TestSettings")
        .whitelist_type("mlperf::QuerySample.*")
        .whitelist_type("mlperf::Test.*")
        .whitelist_type("mlperf::ResponseId")
        .size_t_is_usize(true)
        .ignore_methods()
        .impl_debug(true)
        .enable_cxx_namespaces()
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=mlperf_loadgen");
}
