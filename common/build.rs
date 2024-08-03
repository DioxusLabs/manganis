use std::{collections::HashMap, path::Path};

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

    // If we're resetting the config, we need to delete the file
    if std::env::var("MG_CONFIG_PATH_RESET").is_ok() {
        _ = std::fs::remove_file(&file_with_watch_path);
    }

    if let Ok(config_path) = std::env::var("MG_CONFIG_PATH") {
        std::fs::write(&file_with_watch_path, config_path).unwrap();
    }

    if let Ok(contents) = std::fs::read_to_string(&file_with_watch_path) {
        // the contents will be the path to the config file
        let config_path = Path::new(&contents);
        println!("cargo:rerun-if-changed={}", config_path.display());

        // Now actually write the contents of that config file to the rustc env
        let config = std::fs::read_to_string(config_path).unwrap();
        let entries = config
            .lines()
            .filter_map(|line| {
                let mut split = line.split_terminator('=');
                let key = split.next()?;
                let value = split.next()?;
                Some((key, value))
            })
            .collect::<HashMap<_, _>>();

        // set the MG_BASEPATH env var
        println!(
            "cargo:rustc-env=MG_BASEPATH={}",
            entries
                .get("MG_BASEPATH")
                .map(|x| x.to_string())
                .unwrap_or_else(|| ".".to_string())
        );
    } else {
        // don't write the file, just wait for MG_CONFIG_PATH to be set
        println!("cargo:rerun-if-env-changed=MG_CONFIG_PATH");

        // set the MG_BASEPATH env var
        println!("cargo:rustc-env=MG_BASEPATH={}", ".");
    }

    // and always listen for MG_CONFIG_PATH_RESET
    // shouldn't be needed, but just in case. this will cause double-runs
    println!("cargo:rerun-if-env-changed=MG_CONFIG_PATH_RESET");
}
