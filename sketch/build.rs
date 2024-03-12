use std::env;
use std::path::PathBuf;

use bindgen;
use glob::glob;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Get library path from environment variable
    let lib_path = env::var("LIB_PATH").unwrap_or("/usr/local/lib".to_string());

    // Check if minisketch library exists in the specified directory
    let minisketch_files = glob(&format!("{}/*minisketch*", lib_path)).expect("Failed to read glob pattern");

    if minisketch_files.count() == 0 {
        // If not found, build the library
        build_lib();
    }

    generate_bindings();

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=minisketch");
}
fn build_lib() {
    use std::process::Command;

    // Run ./autogen.sh
    let autogen = Command::new("./autogen.sh")
        .status()
        .expect("Failed to run autogen.sh");

    if !autogen.success() {
        panic!("autogen.sh failed");
    }

    // Run ./configure
    let configure = Command::new("./configure")
        .status()
        .expect("Failed to run configure");

    if !configure.success() {
        panic!("configure failed");
    }

    // Run make install
    let make_install = Command::new("make")
        .arg("install")
        .status()
        .expect("Failed to run make install");

    if !make_install.success() {
        panic!("make install failed");
    }
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .generate_comments(false)
        .header("minisketch/include/minisketch.h")
        .whitelist_type("minisketch")
        .opaque_type("minisketch")
        // We'll redefine Clone, Copy and Drop by utilizing minisketch_clone() and minisketch_destroy()
        .no_copy("minisketch")
        .whitelist_function("minisketch_.+") // Bind to all minisketch_...() functions
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
