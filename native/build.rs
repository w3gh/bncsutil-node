extern crate neon_build;
extern crate cmake;

fn main() {
    let dst = cmake::Config::new("bncsutil").build();

    println!("cargo:rustc-link-search={}/build/", dst.display());

    println!("cargo:rustc-link-lib=static=bncsutil_static");

    neon_build::setup(); // must be called in build.rs
}
