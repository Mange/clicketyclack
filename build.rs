use cmake::Config;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
fn get_config() -> PathBuf {
    Config::new("adapters/liblinuxadapter").build()
}

#[cfg(target_os = "linux")]
fn print_config() {
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=static=linuxadapter");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=Xtst");
}

fn main() {
    let dst = get_config();

    println!("cargo:rustc-link-search=native={}", dst.display());
    print_config();
}
