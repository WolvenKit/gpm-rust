use anyhow::Context;
use std::fs;
use std::fs::{remove_dir_all, remove_file};
use std::io::ErrorKind;
use std::path::Path;

/// If a file or folder exist in the given path, remove it, otherwise, do nothing.
///
/// return an error, with the relevant path contained in the text, on io failure.
pub fn remove_path_if_exist(path_to_delete: &Path) -> anyhow::Result<()> {
    match fs::metadata(&path_to_delete) {
        Ok(metadata) => {
            if metadata.is_dir() {
                remove_dir_all(&path_to_delete).with_context(|| {
                    format!("cant make a recursive deletion of {:?}", path_to_delete)
                })?;
            } else {
                remove_file(&path_to_delete)
                    .with_context(|| format!("can't remove the file at {:?}", path_to_delete))?;
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => (),
            _ => {
                return Err(err)
                    .with_context(|| format!("can't get the metadata of {:?}", path_to_delete));
            }
        },
    };
    Ok(())
}
