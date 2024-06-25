use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

// The prefix to the link arg for a path to the current working directory.
const MG_WORKDIR_ARG_NAME: &str = "mg-working-dir;";

/// Intercept the linker for object files.
///
/// Takes the arguments used in a CLI and returns a list of paths to `.rlib` or `.o` files to be searched for asset sections.
pub fn linker_intercept(args: std::env::Args) -> Option<(PathBuf, Vec<PathBuf>)> {
    let args = args.collect::<Vec<String>>();

    // let data = format!("{:?}", args);
    // fs::write("./mg-linker-intercept-out-args", data).unwrap();
    // println!("{:?}", args);

    let mut working_dir = std::env::current_dir().unwrap();

    let Some(arg1) = args.get(1) else {
        return None;
    };
    let is_command_file = arg1.starts_with("@");

    let linker_args = match is_command_file {
        true => {
            let path = args[1].trim().trim_start_matches("@");
            let file_binary = fs::read(path).unwrap();

            // This may be a utf-16le file. Let's try utf-8 first.
            let content = match String::from_utf8(file_binary.clone()) {
                Ok(s) => s,
                Err(_) => {
                    // Convert Vec<u8> to Vec<u16> to convert into a String
                    let binary_u16le: Vec<u16> = file_binary
                        .chunks_exact(2)
                        .into_iter()
                        .map(|a| u16::from_le_bytes([a[0], a[1]]))
                        .collect();

                    String::from_utf16_lossy(&binary_u16le)
                }
            };

            // Gather linker args
            let mut linker_args = Vec::new();
            let lines = content.lines();

            for line in lines {
                let line_parsed = line.to_string();
                let line_parsed = line_parsed.trim_end_matches("\"").to_string();
                let line_parsed = line_parsed.trim_start_matches("\"").to_string();

                linker_args.push(line_parsed);
            }

            linker_args
        }
        false => {
            let args = &args[1..args.len()];
            Vec::from(args)
        }
    };

    // Parse through linker args for `.o` or `.rlib` files.
    let mut object_files: Vec<PathBuf> = Vec::new();
    for item in linker_args {
        // Get the working directory so it isn't lost.
        if item.starts_with(MG_WORKDIR_ARG_NAME) {
            let split: Vec<_> = item.split(";").collect();
            working_dir = PathBuf::from(split[1]);
            continue;
        }

        if item.ends_with(".o") || item.ends_with(".rlib") {
            object_files.push(PathBuf::from(item));
        }
    }

    // debugging
    let data = format!("{:?}", object_files);
    fs::write(
        format!("{}/mg-linker-intercept-out", working_dir.display()),
        data,
    )
    .unwrap();

    Some((working_dir, object_files))
}

/// Calls cargo to build the project, passing this current executable as the linker.
pub fn start_linker_intercept<I, S>(cwd: Option<&Path>, args: I) -> Result<(), std::io::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let exec_path = std::env::current_exe().unwrap();

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("rustc");
    cmd.args(args);
    cmd.arg("--");

    let linker_arg = format!("-Clinker={}", exec_path.display());
    cmd.arg(linker_arg);

    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }

    // Since we have some serious child process stuff going on, (About 3 levels deep)
    // we need to make sure the current working directory makes it through correctly.
    let working_dir = std::env::current_dir().unwrap();
    let working_dir_arg = format!("-Clink-arg=mg-working-dir;{}", working_dir.display());
    cmd.arg(working_dir_arg);

    cmd.spawn()?.wait()?;
    Ok(())
}
