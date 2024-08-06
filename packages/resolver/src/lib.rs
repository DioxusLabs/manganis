use std::path::PathBuf;

pub struct VirtualFile {}

impl VirtualFile {
    /// Return an absolute, canonicalized path to the file
    pub fn path(&self) -> PathBuf {
        todo!()
    }
}

pub fn resolve(mg_path: &str) -> Result<VirtualFile, std::io::Error> {
    todo!()
    // let root = manganis_common::manifest_dir().unwrap();
    // dbg!(&root);
    // todo!()
}

pub fn resolve_macos(mg_path: &str) -> Result<VirtualFile, std::io::Error> {
    todo!()
}
