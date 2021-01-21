use crate::package::Package;
use crate::{mod_storage::ModStorage, tool::canonicalize_folder};
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
    #[serde(default)]
    can_change_profile: Option<bool>,
}

/// represent the configuration of gpm. At the moment, it is loaded (if possible). The path for it
/// is relative to the folder in which you run the command, and is called gpm_config.json.
///
/// default value are otherwise provided.
pub struct GpmConfig {
    pub store_path: PathBuf,
    pub profile_path: PathBuf,
    pub can_change_profile: bool,
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

        let profile_path = canonicalize_folder(&if let Some(default_profile_path) =
            config.default_profile_path
        {
            default_profile_path
        } else {
            PathBuf::from(".")
        })
        .context("can't canonicalize the profile path")?;

        let can_change_profile = config.can_change_profile.unwrap_or(true);

        Ok(Self {
            store_path,
            profile_path,
            can_change_profile,
        })
    }

    /// return a GpmConfig that is totally independent of global config, with default value
    /// focused on test. Path will be set to a canary nonexistant path.
    ///
    /// Use [`Self::load_config`] if you want to load correct default value.
    pub fn empty() -> Self {
        let default_path = PathBuf::from("./non_existant_folder_UkjhXrTwJf");
        Self {
            store_path: default_path.clone(),
            profile_path: default_path.clone(),
            can_change_profile: false,
        }
    }

    /// get a [`ModStorage`] object that use the store defined in the config.
    pub fn default_store(&self) -> ModStorage {
        ModStorage::new(self.store_path.clone())
    }

    /// return the root [`Package`], known as the profile
    pub fn profile(&self) -> anyhow::Result<Package> {
        Package::load_from_folder(self.profile_path.to_path_buf())
            .context("can't load the default profile")
    }
}
