use std::{env, path::Path};

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    // manganis can be configured!
    // you do this by setting MG_CONFIG_PATH when compiling manganis
    //
    // mangannis will then watch that file in this build script looking for changes
    // if you want to change this, you can set a new MG_CONFIG_PATH
    //
    // however, once a config is being watched, the config is now sticky - we stop watching for the
    // mg_config var. From then on, we only watch that file, and re-run when it changes (or is deleted).
    // We also will watch the "MG_CONFIG_PATH_RESET" env var which will cause this build script to re-run. You
    // don't want to use that unless you really need to reset the config.
    let file_with_watch_path = scratch::path("manganis_config").join("mg_pointer.lock");

    if let Ok(config_path) = std::env::var("MG_CONFIG_PATH") {
        std::fs::write(&file_with_watch_path, config_path).unwrap();
    }

    if let Ok(contents) = std::fs::read_to_string(&file_with_watch_path) {
        // the contents will be the path to the config file
        let config_path = Path::new(&contents);
        // panic!("file at {:?} {:?}", file_with_watch_path, config_path);
        println!("cargo:rerun-if-changed={}", config_path.display());
    } else {
        // don't write the file, just wait for MG_CONFIG_PATH to be set
        println!("cargo:rerun-if-env-changed=MG_CONFIG_PATH");
    }

    // set the MG_BASEPATH env var
    println!("cargo:rustc-env=MG_BASEPATH={}", "hi");

    // and always listen for MG_CONFIG_PATH_RESET
    // shouldn't be needed, but just in case. this will cause double-runs
    println!("cargo:rerun-if-env-changed=MG_CONFIG_PATH_RESET");
}

// we need to make the basepath "sticky"
//
// basically if SET_BASE_PATH shows up in the environment, we need to set the basepath to the value
// but it might not always be set, and that shouldn't *unset* the basepath
//
// The primary issue here is that we have cache thrasing issues.

// let top_level = std::env::var("CARGO_MANIFEST_DIR").unwrap();
// let top_level_toml = Path::new(&top_level).join("Cargo.toml");

// // Rerun the macro if the workspace changes
// let output = std::process::Command::new(env!("CARGO"))
//     .arg("locate-project")
//     .arg("--workspace")
//     .arg("--message-format=plain")
//     .arg("--manifest-path")
//     .arg(top_level_toml)
//     .output()
//     .unwrap()
//     .stdout;

// let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
// let path = cargo_path.parent().unwrap().to_path_buf();
// println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
// let out_dir = std::env::var("OUT_DIR").unwrap();

// let set_base_path = std::env::var("SET_BASE_PATH");

// let fprint = scratch::path("fprint").join("fprint.lock");
// if !fprint.exists() {
//     std::fs::write(&fprint, ".").unwrap();
// }

// let set_base_path = match set_base_path {
//     // if the SET_BASE_PATH env var is set, we need to set the basepath to the value by writing to the lock file
//     Ok(set) => {
//         if set.is_empty() {
//             std::fs::write(&fprint, ".").unwrap();
//             ".".to_string()
//         } else {
//             std::fs::write(&fprint, &set).unwrap();
//             set
//         }
//     }
//     Err(_) => std::fs::read_to_string(&fprint).unwrap(),
// };

// println!("cargo:rustc-env=MG_BASEPATH={}", set_base_path);
// panic!("{:?}", fprint.display());
// println!("cargo:rerun-if-changed={}", &fprint.display());

// if set_base_path.is_err() {

// }
// // let out_dir = env!("OUT_DIR");
// println!("cargo:rerun-if-env-changed=MANGANIS_SUPPORT");

// let mut build_vars = String::new();
// build_vars.push_str(&format!("OUT_DIR={}_____", out_dir));
// for (key, value) in std::env::vars() {
//     build_vars.push_str(&format!("{}={}_________", key, value));
// }

// // println!("cargo:rerun-if-env-changed=MANGANIS_SUPPORT");
// // println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");

// // println!("cargo:rerun-if-changed={}", path.display());

// // and then set that directory into the program
// println!("cargo:rustc-env=MG_WORKSPACE={}", build_vars);
// println!("cargo:rustc-env-var=MG_WORKSPACE={}", );
// let dir = scratch::path("manganis-assets");
// println!("cargo:rustc-env=MG_WORKSPACE={}", dir.display());
// println!("cargo:rerun-if-env-changed=MG_WORKSPACE");
