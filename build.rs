fn main() {
    println!("cargo:rustc-link-lib=static=tree-sitter-python");
    println!("cargo:rustc-link-lib=static=tree-sitter-ruby");
    println!("cargo:rustc-link-lib=static=tree-sitter-javascript");
    println!("cargo:rustc-link-lib=static=tree-sitter-typescript");
    println!("cargo:rustc-link-lib=static=tree-sitter-bash");
    println!("cargo:rustc-link-lib=static=tree-sitter-rust");
}
