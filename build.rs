use std::env;
use std::path::PathBuf;

fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("cbits/ccc.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    let bindings = bindgen::Builder::default()
        .header("cbits/lg.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_recursively(false)
        .whitelist_function("mlperf::c.*")
        .whitelist_type("mlperf::c::.*")
        .whitelist_type("mlperf::TestSettings")
        .whitelist_type("mlperf::LogSettings")
        .whitelist_type("mlperf::LoggingMode")
        .whitelist_type("mlperf::LogOutputSettings")
        .whitelist_type("std::string")
        .opaque_type("std::string")
        // this doesn't seem to work so we patch it later instead
        // .new_type_alias("std::string")

        // don't copy because it constains a c++ std::string
        .no_copy("mlperf::LogOutputSettings")
        .no_copy("mlperf::LogSettings")
        .whitelist_type("mlperf::QuerySample.*")
        .whitelist_type("mlperf::Test.*")
        .whitelist_type("mlperf::ResponseId")
        .size_t_is_usize(true)
        // there's a method on TestSettings which we can't use because it uses C++ strings
        .ignore_methods()
        .impl_debug(true)
        .enable_cxx_namespaces()
        // these two ares are the only way I could get it to work with c++
        .clang_arg("-x")
        .clang_arg("c++")
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        .generate()
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    // bindgen's `opaque_type` doesn't seems to woth with `new_type_alias` so manually patch it.
    // Also add some documentation and make the interior non-public (which bindgen's type alias
    // doesn't do).
    let string_match = "pub type string = \\(.*\\);";
    let string_replacement = "#[repr(transparent)]\\n/// A C++ string\\npub struct string(\\1);";
    let sed_expr = format!("s:{}:{}:", string_match, string_replacement);

    let output = std::process::Command::new("sed")
        .args(&["-ie", &sed_expr, bindings_path.to_str().expect("bad bindings path")])
        .output()
        .expect("failed to patch bindings");

    if !output.status.success() {
        use std::io::{self, Write};
        eprintln!("patching failed:");
        io::stderr().write_all(&output.stderr).unwrap();
    }

    println!("cargo:rustc-link-lib=mlperf_loadgen");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cbits/ccc.h");
    println!("cargo:rerun-if-changed=cbits/ccc.cc");
}
