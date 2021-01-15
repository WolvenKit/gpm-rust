use gpm_core::config::GpmConfig;
use gpm_core::install::install_from_path;

pub struct InstallParameter {
    /// The input folder, is an arbitrary [`String`], as it may represent some thing different than a path on the local fs, like an https remote.
    pub input: String,
}

pub fn install(parameter: InstallParameter, config: GpmConfig) -> anyhow::Result<()> {
    let store = config.default_store();
    install_from_path(&store, &parameter.input)?;
    todo!("package has been installed to the store, but has not been added to the depency of this profile");
}
