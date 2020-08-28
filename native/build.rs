//extern crate neon_build;
extern crate cmake;

use cmake::Config;

fn main() {
//    neon_build::setup(); // must be called in build.rs

    let dst = Config::new("bncsutil").build();

    println!("cargo:rustc-link-search={}/build/", dst.display());

    println!("cargo:rustc-link-lib=static=bncsutil_static");

    // add project-specific build logic here...
}
