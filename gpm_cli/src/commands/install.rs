use anyhow::Context;
use gpm_core::install::install_from_path;
use gpm_core::mod_storage::ModStorage;
use std::path::PathBuf;

pub struct InstallParameter {
    /// The input folder, is an arbitrary [`String`], as it may represent some thing different than a path on the local fs, like an https remote.
    pub input: String,
    pub store_path: PathBuf,
}

pub fn install(parameter: InstallParameter) -> anyhow::Result<()> {
    let store = ModStorage::new(
        parameter
            .store_path
            .canonicalize()
            .context("unable to get the full path of the store")?,
    );
    install_from_path(&store, &parameter.input)
}
