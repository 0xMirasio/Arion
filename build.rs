use walkdir::WalkDir;
use std::path::Path;

fn watch_files_with_ext<P: AsRef<Path>>(dir: P, ext: &str) {
    for entry in WalkDir::new(dir) {
        let path = match entry {
            Ok(e) => e.path().to_path_buf(),
            Err(_) => continue,
        };

        if path.extension().map_or(false, |e| e == ext) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn main() {
    let mut build = cxx_build::bridge("bindings/rust/src/common/config.rs");
    build
        .std("c++17")
        .include("include")
        .include("lib/unicorn/include")
        .include("lib/capstone/include")
        .include("lib/keystone/include")
        .include("lib/lief/include")
        .include("lib/spdlog/include")
        .include("lib/lief/build/include")
        .define("ARION_ONLY", Some("1"))
        .flag_if_supported("-std=c++17")
        .flag("-Wno-reorder")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-redundant-move")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-variable")
        .flag("-Wno-pessimizing-move")
        .flag("-Wno-catch-value")
        .flag("-Wno-maybe-uninitialized")
        .flag("-Wno-restrict");

    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        let path = entry.path();
    
        if path.extension().map_or(false, |ext| ext == "cpp") {
            build.file(&path);
        }
    }     

    build.compile("arion_engine");

    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=unicorn");
    println!("cargo:rustc-link-lib=keystone");
    println!("cargo:rustc-link-lib=capstone");
    println!("cargo:rustc-link-lib=LIEF");
    println!("cargo:rustc-link-lib=spdlog");
    println!("cargo:rustc-link-lib=arion");

    watch_files_with_ext("src", "cpp");
    watch_files_with_ext("bindings/rust/src", "rs");
    watch_files_with_ext("include", "hpp");
}