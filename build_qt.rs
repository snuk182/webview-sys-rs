extern crate cmake;

pub fn main() {
    let mut config = cmake::Config::new("sys/webview");
    config.generator("Ninja");
    config.define("WEBVIEW_BUILD_STATIC_LIBRARY", "true");
    config.define("WEBVIEW_IS_TOP_LEVEL_BUILD", "false");
    config.define("WEBVIEW_QT", "true");
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
    println!("cargo:rustc-link-lib=dylib=Qt5WebEngineWidgets");
    println!("cargo:rustc-link-lib=dylib=Qt5WebChannel");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}