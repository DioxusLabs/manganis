// Check if the MANGANIS_SUPPORT environment variable is set to true. If it is not found, then warn the user that the assets macro will not work.

use std::path::Path;

fn main() {

    // let config_path = manganis_common::Config::config_path();
    // println!("cargo:rerun-if-changed={}", config_path.display());

    // let manganis_support = std::env::var("MANGANIS_SUPPORT");
    // println!("cargo:rerun-if-env-changed=MANGANIS_SUPPORT");

    // if manganis_support.as_deref() != Ok("true") {
    //     println!("cargo:warning=It looks like you are compiling with a tool that does not support collecting assets. The assets that the assets macro collects may not be available to the application.\nHINT: If you are using cargo to run this program, try using the dioxus-cli instead (https://crates.io/crates/dioxus-cli).");
    // }
}
