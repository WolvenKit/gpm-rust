use anyhow::Context;
use gpm_core::config::GpmConfig;
use gpm_core::install::install_from_path;

pub struct InstallParameter {
    /// The input folder, is an arbitrary [`String`], as it may represent some thing different than a path on the local fs, like an https remote.
    pub input: String,
}

pub fn install(parameter: InstallParameter, config: GpmConfig) -> anyhow::Result<()> {
    let store = config.default_store();
    let installed_package = install_from_path(&store, &parameter.input)?;
    let installed_id = installed_package
        .information
        .identifier
        .context("An error happened while getting the identifier of the newly installed package")?;
    let mut profile = config.profile()?;
    profile.information.dependencies.insert(installed_id.clone());
    profile
        .save_configuration()
        .context("error saving the profile")?;
    println!(
        "the package {:?} has been successfully added to the current profile.",
        installed_id
    );
    Ok(())
}
