fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-cdylib-link-arg=-fPIC");
    println!("cargo:rustc-link-lib=dylib=curl");
}
