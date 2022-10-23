extern crate bindgen;

use std::env;
use std::path::PathBuf;

use cmake;

fn invoke_cmake(){
  // Builds the project in the directory located in `libfoo`, installing it
  // into $OUT_DIR
  let dst = cmake::build("libvideoc");

  println!("cargo:rustc-link-search=native={}", dst.display());
  println!("cargo:rustc-link-lib=dylib=videoc");
}

fn invoke_buildgen(){
  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  // Tell cargo to look for shared libraries in the specified directory
  // println!("cargo:rustc-link-search=/path/to/lib");
  println!("cargo:rustc-link-search=libvideoc/install/libs");

  // Tell cargo to tell rustc to link the system bzip2
  // shared library.
  println!("cargo:rustc-link-lib=videoc");

  // Tell cargo to invalidate the built crate whenever the wrapper changes
  println!("cargo:rerun-if-changed=wrapper_videoc.h");

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .header("wrapper_videoc.h")
    // .header("libvideoc/install/include/renderframe.h")
    //Only public interface to library
    .allowlist_function("genSomeData")
    .allowlist_function("freeData")
    .allowlist_function("renderfrom")
    // .allowlist_file(out_path.join("install/include/videoc.h").to_str().unwrap())
    // .allowlist_file(out_path.join("install/include/renderframe.h").to_str().unwrap())
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}

fn main() {
  invoke_cmake();
  invoke_buildgen();
}
