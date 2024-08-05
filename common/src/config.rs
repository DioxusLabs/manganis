use std::path::PathBuf;

/// Get the base path for assets defined by the MG_BASEPATH environment variable
///
/// The basepath should always start and end with a `/`
///
/// If no basepath is set, the default is `/` which is the root of the assets folder.
pub fn base_path() -> PathBuf {
    match option_env!("MG_BASEPATH") {
        Some(path) => {
            let path = path.trim_end_matches('/').trim_start_matches('/');
            PathBuf::from(format!("/{path}/"))
        }
        None => "/".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_path_works() {
        assert_eq!(base_path(), PathBuf::from("/"));
    }
}
