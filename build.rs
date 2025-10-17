extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::sync::{Arc, RwLock};    
use std::path::{PathBuf, Path};
use std::process::Command;

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};

#[cfg(all(feature = "win32", target_os = "windows"))]
mod build_win32;
#[cfg(all(feature = "win32", target_os = "windows"))]
use build_win32 as inner;

#[cfg(all(feature = "cocoa", target_os = "macos"))]
mod build_cocoa;
#[cfg(all(feature = "cocoa", target_os = "macos"))]
use build_cocoa as inner;

#[cfg(feature = "qt5")]
mod build_qt;
#[cfg(feature = "qt5")]
use build_qt as inner;

#[cfg(feature = "gtk3")]
mod build_gtk;
#[cfg(feature = "gtk3")]
use build_gtk as inner;

#[allow(unreachable_code)]
fn main() {
    if !Path::new("sys/webview/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init", "--recursive"]).status().unwrap();
    }

    inner::main();

    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .derive_default(true)
        .header("sys/webview/core/include/webview/webview.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .formatter(bindgen::Formatter::Rustfmt)
        .parse_callbacks(Box::new(MacroCallback { macros: Arc::new(RwLock::new(HashSet::new())) }))
        .generate()
        .expect("Unable to generate bindings");

    let dest_path = PathBuf::from(env::var("OUT_DIR").expect("Unable to get OUT_DIR")).join("bindings.rs");
    bindings.write_to_file(dest_path).expect("Couldn't write bindings!");           
}

#[cfg(not(any(feature = "win32", feature = "cocoa", feature = "qt5", feature = "gtk3")))]
mod inner {
    pub fn main() {
	     println!("You have to pick a backend feature explicitly. For now available are \"win32\", \"cocoa\", \"gtk3\" and \"qt5\".");
    }
}

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        if name == "FP_INFINITE"
            || name == "FP_ZERO"
            || name == "FP_NAN"
            || name == "FP_SUBNORMAL"
            || name == "FP_NORMAL"
        {
            return MacroParsingBehavior::Ignore
        }
        MacroParsingBehavior::Default
    }
} 