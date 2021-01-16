use crate::mod_storage::ModStorage;

use crate::package::Package;
use anyhow::Context;
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs::File;

use std::io::ErrorKind;
use std::path::PathBuf;

// NOTE: this may need to be rewritten when we need to allow changing the settings via the user
// interface. Also, having system-wide or user-wide config would be nice. Making it an external
// library would be nice.
#[derive(Deserialize, Default)]
struct GpmConfigStored {
    #[serde(default)]
    store_path: Option<PathBuf>,
    #[serde(default)]
    default_profile_path: Option<PathBuf>,
}

/// represent the configuration of gpm. At the moment, it is loaded (if possible). The path for it
/// is relative to the folder in which you run the command, and is called gpm_config.json.
///
/// default value are otherwise provided.
pub struct GpmConfig {
    pub store_path: PathBuf,
    pub profile_path: PathBuf,
}

impl GpmConfig {
    /// load the config. For the moment, it only look at ./gpm_config.json . If it doesn't exist,
    /// it load the default setting. If field are missing, the missing field use the value from
    /// the default config.
    pub fn load_config() -> anyhow::Result<Self> {
        let config = match File::open("./gpm_config.json") {
            Ok(config_file) => {
                serde_json::from_reader(config_file).context("can't read/parse the config file")?
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => GpmConfigStored::default(),
                _ => return Err(err).context("can't read the config file")?,
            },
        };

        let store_path = if let Some(store_path) = config.store_path {
            store_path
        } else {
            ProjectDirs::from("", "WolvenKit", "gpm")
                .context(
                    "Can't get the store folder path. You can specify one manually in the configuration file.",
                )?
                .data_dir()
                .join("store")
        };

        // TODO: this will fail it the target folder doesn't exist. Just expand ./ and ~ at start
        // of path.
        let profile_path = if let Some(default_profile_path) = config.default_profile_path {
            default_profile_path
        } else {
            PathBuf::from(".")
        }
        .canonicalize()
        .context("can't get the absolute path of the current folder")?;

        Ok(Self {
            store_path,
            profile_path,
        })
    }

    /// get a [`ModStorage`] object that use the store defined in the config.
    pub fn default_store(&self) -> ModStorage {
        ModStorage::new(self.store_path.clone())
    }

    /// return the root [`Package`], known as the profile
    pub fn profile(&self) -> Package {
        todo!()
    }
}
