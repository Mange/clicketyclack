#[cfg(target_os = "linux")]
fn get_config() -> &'static str {
    println!("cargo:rerun-if-changed=adapters/liblinuxadapter/adapter.h");
    println!("cargo:rerun-if-changed=adapters/liblinuxadapter/adapter.c");

    cc::Build::new()
        .file("adapters/liblinuxadapter/adapter.c")
        // Optimization must be enabled, or else this warning is emitted:
        // warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
        // Thankfully, we don't do much inside of that C code, so the need to debug is fairly small.
        // If you need to debug something during development, uncomment this and suffer the warning
        // until you have it working again.
        .opt_level(3)
        .compile("linuxadapter");

    "linuxadapter"
}

#[cfg(target_os = "linux")]
fn print_config() {
    println!("cargo:rustc-link-lib=static=linuxadapter");
    println!("cargo:rustc-link-lib=dylib=input");
    println!("cargo:rustc-link-lib=dylib=udev");
}

fn main() {
    let libname = get_config();

    println!("cargo:rustc-link-search=native={}", libname);
    print_config();
}
