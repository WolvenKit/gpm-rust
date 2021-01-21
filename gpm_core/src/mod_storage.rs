use crate::constants::{MOD_STORAGE_SEPARATOR, STORAGE_TEMPORY_FOLDER_NAME};

use crate::tool::remove_path_if_exist;
use anyhow::Context;
use std::fs;
use std::fs::{create_dir_all, rename};
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
//TODO: decide about a final separator and path escaped. For now, separator is | and there is no escaping

/// A [`ModStorage`] determine were to store unpacked mod.
///
/// It use a root folder, then create subfolder to store it's mod.
pub struct ModStorage {
    root_folder: PathBuf,
}

impl ModStorage {
    /// Create a new [`ModStorage`] with the given root folder
    pub fn new(root_folder: PathBuf) -> Self {
        Self { root_folder }
    }

    /// Get the path to the folder the mod with the given identifier and version
    /// should be stored. Doesn't create any folder/file on the file system.
    pub fn mod_folder(&self, identifier: &str, version: &str) -> PathBuf {
        self.root_folder
            .join(&self.get_mod_store_name(identifier, version))
    }

    fn get_mod_store_name(&self, identifier: &str, version: &str) -> String {
        format!("{}{}{}", identifier, MOD_STORAGE_SEPARATOR, version)
    }

    /// Check if a mod exist in this store.
    ///
    /// A mod exist is considered as existing if the store directory for this mod
    /// exist and is a folder (or a symlink to a folder)
    pub fn check_mod_stored(&self, identifier: &str, version: &str) -> io::Result<bool> {
        let expected_mod_folder = self.mod_folder(identifier, version);
        match fs::metadata(&expected_mod_folder) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    Ok(true)
                } else {
                    Err(io::Error::new(
                        ErrorKind::Other,
                        format!("{:?} isn't a folder, as mod should be", expected_mod_folder),
                    ))
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(false),
                _ => Err(err),
            },
        }
    }

    /// return the tempory folder for a package, but doesn't make any check on the filesystem.
    fn get_tempory_folder_path(&self, identifier: &str, version: &str) -> PathBuf {
        let tempory_folder_path = self.root_folder.join(STORAGE_TEMPORY_FOLDER_NAME);
        return tempory_folder_path.join(&self.get_mod_store_name(identifier, version));
    }

    /// Create a folder to store data of a package while building. Having it separate to the final
    /// installation path allow to first build there, then install it via move/hardlink, being
    /// atomic (immediate) on most journalized file system when they are on the same file system.
    ///
    /// This ensure that the target path is an empty folder. If it already exist, it is remove.
    pub fn create_tempory_folder_for_package(
        &self,
        identifier: &str,
        version: &str,
    ) -> anyhow::Result<PathBuf> {
        let package_tempory_folder = self.get_tempory_folder_path(identifier, version);

        // remove what already exist there
        remove_path_if_exist(&package_tempory_folder)
            .context("can't remove the old content in the tempory path")?;

        // create the empty dir
        create_dir_all(&package_tempory_folder).with_context(|| {
            format!(
                "can't create the tempory folder {:?}",
                package_tempory_folder
            )
        })?;
        Ok(package_tempory_folder)
    }

    /// Assign a tempory folder for a package. Equivalent to
    /// [`Self::create_tempory_folder_for_package`], but ensure the tempory destination path
    /// doesn't exist, while it's parent folder exist.
    pub fn assign_tempory_folder_for_package(
        &self,
        identifier: &str,
        version: &str,
    ) -> anyhow::Result<PathBuf> {
        let package_tempory_folder = self.get_tempory_folder_path(identifier, version);
        if let Some(parent_directory) = package_tempory_folder.parent() {
            create_dir_all(parent_directory).with_context(|| {
                format!("can't create the tempory folder {:?}", parent_directory)
            })?;
        };
        remove_path_if_exist(&package_tempory_folder)
            .with_context(|| format!("can't remove the old content in the tempory path"))?;
        Ok(package_tempory_folder)
    }

    /// Install a mod by moving the content from the given path in the appropriate
    /// folder for the given mod identifier and version. The source path need to
    /// be in the same filesystem. It is hightly recommand to create it with
    /// [`Self::create_tempory_folder_for_package`].
    ///
    /// Delete the content of the destination path if present.
    pub fn install_mod_by_move(
        &self,
        source_path: &Path,
        identifier: &str,
        version: &str,
    ) -> anyhow::Result<()> {
        let install_path = &self.mod_folder(identifier, version);
        remove_path_if_exist(&install_path).context("can't remove the old destination path")?;
        rename(source_path, install_path).with_context(|| {
            format!(
                "unable the mod from the tempory folder {:?} into the final location {:?}",
                source_path, install_path
            )
        })?;
        Ok(())
    }

    /// get the root folder
    pub fn root_folder(&self) -> &Path {
        &self.root_folder
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::MOD_STORAGE_SEPARATOR;
    use crate::mod_storage::ModStorage;
    use std::path::PathBuf;

    #[test]
    fn test_mod_storage() {
        let root_folder = PathBuf::from("/mod");
        let storage = ModStorage::new(root_folder.clone());
        assert_eq!(storage.root_folder(), &root_folder);
        assert_eq!(
            storage.mod_folder("hello_world", "1.0.0"),
            PathBuf::from(format!("/mod/hello_world{}1.0.0", MOD_STORAGE_SEPARATOR))
        );
    }
}
