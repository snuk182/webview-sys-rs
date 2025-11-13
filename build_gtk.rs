extern crate cmake;

pub fn main() {
    let mut config = cmake::Config::new("sys/webview");
    config.generator("Ninja");
    config.define("WEBVIEW_BUILD_STATIC_LIBRARY", "true");
    config.define("WEBVIEW_IS_TOP_LEVEL_BUILD", "false");
    config.define("WEBVIEW_GTK", "true");
    config.define("CMAKE_CXX_STANDARD", "11");

    if cfg!(debug_assertions) {
        config.define("CMAKE_BUILD_TYPE", "Debug");
    } else {
        config.define("CMAKE_BUILD_TYPE", "Release");
    }
    config.build();

    let mut config = cmake::Config::new("sys/webview");
    config.no_build_target(true);
    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/build/core", dst.display());
    println!("cargo:rustc-link-lib=static=webviewd");
    println!("cargo:rustc-link-lib=dylib=webkit2gtk-4.1");
    println!("cargo:rustc-link-lib=dylib=javascriptcoregtk-4.1");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}