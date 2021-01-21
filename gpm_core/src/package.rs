use crate::stored_package_information::StoredPackageInformation;
use crate::{constants::LOCK_FILE_PATH, package_information::PackageInformation};
use crate::{
    constants::{JSON_CONFIG_PATH, TOML_CONFIG_PATH},
    lockfile::LockFile,
};
use anyhow::Context;
use std::{
    fs::File,
    io::{self, Write},
};

use std::io::ErrorKind;
use std::io::Read;

use std::path::Path;
use std::path::PathBuf;

pub struct Package {
    pub information: PackageInformation,
    pub lockfile: LockFile,
    root_folder: PathBuf,
}

impl Package {
    /// Load the package from the given root folder. It need either a config.toml or a config.json
    /// to load successfully (with the priority given to config.toml)
    pub fn load_from_folder(root_folder: PathBuf) -> anyhow::Result<Self> {
        let lockfile_path = root_folder.join("gpm.lock");
        let lockfile = match LockFile::load_file(&lockfile_path) {
            Ok(v) => v,
            Err(err) => match err.downcast_ref::<io::Error>() {
                None => return Err(err),
                Some(io_err) => match io_err.kind() {
                    ErrorKind::NotFound => LockFile::default(),
                    _ => return Err(err),
                },
            },
        };
        // try to load the config.toml file
        let config_toml_path = root_folder.join(TOML_CONFIG_PATH);
        match File::open(&config_toml_path) {
            Ok(mut toml_file) => {
                return Self::load_from_toml_reader(root_folder, &mut toml_file, lockfile)
            }
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
            Ok(mut json_file) => {
                return Self::load_from_json_reader(root_folder, &mut json_file, lockfile)
            }
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
        lockfile: LockFile,
    ) -> anyhow::Result<Self> {
        let information = StoredPackageInformation::new_from_toml_reader(toml_file)?.into();
        Ok(Self {
            information,
            root_folder,
            lockfile,
        })
    }

    fn load_from_json_reader(
        root_folder: PathBuf,
        json_file: &mut impl Read,
        lockfile: LockFile,
    ) -> anyhow::Result<Self> {
        let information = StoredPackageInformation::new_from_json_reader(json_file)?.into();
        Ok(Self {
            information,
            root_folder,
            lockfile,
        })
    }

    /// Create a new [`Package`] with default value telling the user to modify them.
    pub fn new_in_folder(root_folder: PathBuf) -> Self {
        Self {
            root_folder,
            information: PackageInformation::new(
                "your name",
                "package_id",
                "0.1", 
                "Display Name",
                "A short description of your mod",
                "a text describing the license of the content of this package (consider using a version of the creative-common one)"
            ),
            lockfile: LockFile::default(),
        }
    }

    /// Return the root folder where this [`Package`] is present (where the data/config.toml is saved)
    pub fn root_folder(&self) -> &Path {
        &self.root_folder
    }

    /// Save the configuration in the config.toml file. You need to also call [`Self::save_lockfile`] to save the lockfile.
    pub fn save_configuration(&self) -> anyhow::Result<()> {
        let config_toml_path = self.root_folder.join(TOML_CONFIG_PATH);
        let toml = StoredPackageInformation::from(&self.information)
            .generate_toml()
            .context("can't generate the toml content for the package information")?;
        let mut config_file = File::create(&config_toml_path).with_context(|| {
            format!(
                "can't create/open for write the file at {:?}",
                config_toml_path
            )
        })?;
        config_file.write_all(&toml).with_context(|| {
            format!(
                "can't write the content of the configuration file in {:?}",
                config_toml_path
            )
        })?;
        Ok(())
    }

    /// Save the lockfile for this package. You also need to call [`Self::save_configuration`] to save this package configuration.
    pub fn save_lockfile(&self) -> anyhow::Result<()> {
        self.lockfile.write_file(&self.lockfile_path())
    }

    /// Return the path to the lockfile of this package, be it existant or not.
    pub fn lockfile_path(&self) -> PathBuf {
        self.root_folder().join(LOCK_FILE_PATH)
    }

    /// Save both the lockfile and the configuration path. This isn't atomic, so it may result in corrupted configuration or lockfile files.
    pub fn save_configuration_and_lockfile(&self) -> anyhow::Result<()> {
        self.save_configuration()
            .context("can't save the configuration file")?;
        self.save_lockfile().context("can't save the lockfile")
    }
}
