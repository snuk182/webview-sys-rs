extern crate cmake;

pub fn main() {
    #[cfg(all(feature = "win32", not(target_os = "windows")))]
    {
        panic!("The feature `win32` is available on the Windows platforms only.");
    }

    let mut config = cmake::Config::new(".");
    config.generator("Ninja")
        .define("WEBVIEW_USE_COMPAT_MINGW", "true")
        .define("WEBVIEW_USE_BUILTIN_MSWEBVIEW2", "true")
        .define("WEBVIEW_BUILD_STATIC_LIBRARY", "true")
        .build_target("build");
    if cfg!(debug_assertions) {
        config.define("CMAKE_BUILD_TYPE", "Debug");
    } else {
        config.define("CMAKE_BUILD_TYPE", "Release");
    }
    config.build();
}