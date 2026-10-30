#[cfg(target_os = "linux")]
fn get_config() -> &'static str {
    println!("cargo:rerun-if-changed=adapters/liblinuxadapter/adapter.h");
    println!("cargo:rerun-if-changed=adapters/liblinuxadapter/adapter.c");
    cc::Build::new()
        .file("adapters/liblinuxadapter/adapter.c")
        .compile("linuxadapter");
    "linuxadapter"
}

#[cfg(target_os = "linux")]
fn print_config() {
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=static=linuxadapter");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=Xtst");
}

fn main() {
    let libname = get_config();

    println!("cargo:rustc-link-search=native={}", libname);
    print_config();
}
