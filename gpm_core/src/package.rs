use crate::constants::{JSON_CONFIG_PATH, TOML_CONFIG_PATH};
use crate::package_information::PackageInformation;
use crate::stored_package_information::StoredPackageInformation;
use anyhow::Context;
use std::fs::File;

use std::io::ErrorKind;
use std::io::Read;

use std::path::Path;
use std::path::PathBuf;

pub struct Package {
    pub information: PackageInformation,
    root_folder: PathBuf,
}

impl Package {
    /// Load the package from the given root folder. It need either a config.toml or a config.json
    /// to load successfully (with the priority given to config.toml)
    pub fn load_from_folder(root_folder: PathBuf) -> anyhow::Result<Self> {
        // try to load the config.toml file
        let config_toml_path = root_folder.join(TOML_CONFIG_PATH);
        match File::open(&config_toml_path) {
            Ok(mut toml_file) => return Self::load_from_toml_reader(root_folder, &mut toml_file),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => (),
                _ => {
                    return Err(err).context(format!("can't open the file {:?}", config_toml_path))
                }
            },
        };
        // the toml file doesn't exist, load the json one
        let config_json_path = root_folder.join(JSON_CONFIG_PATH);
        match File::open(&config_json_path) {
            Ok(mut json_file) => return Self::load_from_json_reader(root_folder, &mut json_file),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    return Err(err).context(format!(
                        "neither {:?} or {:?} can be loaded for the package at {:?}",
                        config_toml_path, config_json_path, root_folder
                    ))?
                }
                _ => {
                    return Err(err).context(format!("can't open the file {:?}", config_json_path))
                }
            },
        }
    }

    fn load_from_toml_reader(
        root_folder: PathBuf,
        toml_file: &mut impl Read,
    ) -> anyhow::Result<Self> {
        let information = StoredPackageInformation::new_from_toml_reader(toml_file)?.into();
        Ok(Self {
            information,
            root_folder,
        })
    }

    fn load_from_json_reader(
        root_folder: PathBuf,
        json_file: &mut impl Read,
    ) -> anyhow::Result<Self> {
        let information = StoredPackageInformation::new_from_json_reader(json_file)?.into();
        Ok(Self {
            information,
            root_folder,
        })
    }

    pub fn root_folder(&self) -> &Path {
        &self.root_folder
    }
}
