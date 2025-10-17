extern crate cmake;

pub fn main() {
    let mut config = cmake::Config::new("sys/webview");
    config.generator("Ninja");
    config.define("WEBVIEW_BUILD_STATIC_LIBRARY", "true");
    config.define("WEBVIEW_IS_TOP_LEVEL_BUILD", "false");
    config.define("WEBVIEW_GTK", "true");

    if cfg!(debug_assertions) {
        config.define("CMAKE_BUILD_TYPE", "Debug");
    } else {
        config.define("CMAKE_BUILD_TYPE", "Release");
    }
    config.build();

    let mut config = cmake::Config::new("sys/webview");
    config.no_build_target(true);
    config.no_build_target(true).no_build_target(true);
    config.build();
}