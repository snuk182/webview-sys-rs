extern crate cmake;


pub fn main() {
    #[cfg(all(feature = "win32", not(target_os = "windows")))]
    {
        panic!("The feature `win32` is available on the Windows platforms only.");
    }
    let mut config = cmake::Config::new("sys\\webview");
    if cfg!(target_env = "msvc") {
        config.generator("Visual Studio 17 2022");
    } else {
        config.generator("MinGW Makefiles");
        config.define("WEBVIEW_USE_COMPAT_MINGW", "true");
    }
    config.define("WEBVIEW_USE_BUILTIN_MSWEBVIEW2", "true");
    config.define("WEBVIEW_BUILD_STATIC_LIBRARY", "true");
    config.define("WEBVIEW_IS_TOP_LEVEL_BUILD", "false");
    config.define("WEBVIEW_EDGE", "true");
    config.define("CMAKE_CXX_STANDARD", "11");
    config.cxxflag("/EHsc");
    config.static_crt(true);
    config.build();

    let mut config = cmake::Config::new("sys\\webview");
    config.no_build_target(true);
    config.cxxflag("/EHsc");
    if cfg!(debug_assertions) {
        config.define("CMAKE_BUILD_TYPE", "Debug");
    } else {
        config.define("CMAKE_BUILD_TYPE", "Release");
    }
    config.static_crt(true);
    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/build/core/{}", dst.display(), if cfg!(debug_assertions) { "Debug" } else { "Release" });
    println!("cargo:rustc-link-lib=static=webview_staticd");
    if cfg!(debug_assertions) {
        println!("cargo::rustc-link-lib=dylib=msvcrtd");
    }
}
