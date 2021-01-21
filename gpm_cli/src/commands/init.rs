use anyhow::Context;
use gpm_core::{config::GpmConfig, package::Package};
use std::fs::create_dir_all;
use std::result::Result;

pub fn init(config: &GpmConfig) -> Result<(), anyhow::Error> {
    let profile_path = &config.profile_path;
    create_dir_all(profile_path)
        .with_context(|| format!("can't create the destination folder {:?}", profile_path))?;
    let new_package = Package::new_in_folder(profile_path.clone());
    new_package
        .save_configuration()
        .context("can't save the newly created package")?;
    println!(
        "A template was initialised in {:?}. You should now customize it.",
        profile_path
    );
    Ok(())
}
