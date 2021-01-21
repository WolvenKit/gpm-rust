use std::path::PathBuf;

use anyhow::Context;
use gpm_core::install::install_from_path;
use gpm_core::{config::GpmConfig, lockfile::LockSource};

pub struct InstallLocalParameter {
    pub input: PathBuf,
}

pub fn install_local(parameter: InstallLocalParameter, config: GpmConfig) -> anyhow::Result<()> {
    let store = config.default_store();
    let installed_package = install_from_path(&store, &parameter.input)?;
    let installed_id = installed_package
        .information
        .identifier
        .context("An error happened while getting the identifier of the newly installed package")?;
    let installed_version = installed_package
        .information
        .version
        .context("Can't get the version for the newly installed package")?;
    let mut profile = config.profile()?;
    profile
        .information
        .dependencies
        .insert(installed_id.clone());
    profile.lockfile.set_dependency_source(
        installed_id.clone(),
        LockSource::IdVersion {
            identifier: installed_id.clone(),
            version: installed_version,
        },
    );
    profile
        .save_configuration_and_lockfile()
        .context("error saving the profile")?;
    println!(
        "the package {:?} has been successfully added to the current profile.",
        installed_id
    );
    Ok(())
}
