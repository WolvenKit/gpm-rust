use crate::mod_storage::ModStorage;
use crate::package_information::PackageInformation;
use crate::stored_package_information::StoredPackageInformation;
use crate::{constants::JSON_CONFIG_PATH, package::Package};
use anyhow::Context;
use std::env::current_dir;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PackageInput {
    LocalPath(PathBuf),
}

impl PackageInput {
    pub fn solve_input(input: &str, current_dir: &Path) -> anyhow::Result<Self> {
        if input.starts_with('.') || input.starts_with('/') {
            let mut package_dir = PathBuf::from(input);
            if package_dir.is_relative() {
                package_dir = current_dir.join(&package_dir);
            };
            Ok(Self::LocalPath(package_dir))
        } else {
            return Err(anyhow::Error::msg(format!(
                "{:?} is not a reconized input. Did you meant ./{:?} ?",
                input, input
            )));
        }
    }
}

pub fn install_from_path(store: &ModStorage, input: &str) -> anyhow::Result<Package> {
    // A mod installation follow those step:
    // 1: install the mod
    // 2: install its dependancies (from the lock file)
    let package_input = PackageInput::solve_input(
        input,
        &current_dir().context("can't get the current working dir")?,
    )
    .context("can't solve the input path")?;
    let package = install_from_package_input(store, &package_input)?;
    println!("warning, the depencies installation is not yet implemented");

    Ok(package)
}

pub fn install_from_package_input(
    store: &ModStorage,
    package_input: &PackageInput,
) -> anyhow::Result<Package> {
    // 1: get the mod metadata
    // 1: install this mod by :
    // 2.1: allocate a tempory folder (identified by mod id and version)
    // 2.2: copy/decompress/download to this tempory folder
    // 2.3: move it to the final destnationzip
    println!("installing the mod from the path {:?}", package_input);
    match package_input {
        PackageInput::LocalPath(source_path) => {
            // assume it is a zip file. later version shoudl also check if it a project folder.
            let compressed_reader = File::open(source_path).context("can't open the input path")?;
            let mut zip_reader = zip::read::ZipArchive::new(compressed_reader)
                .context("can't parse the zip file")?;
            // 1. get the config.json
            let package_information: PackageInformation = {
                let mut config_file = zip_reader.by_name(JSON_CONFIG_PATH).with_context(|| {
                    format!(
                        "can't open the {} file in the compressed archive",
                        JSON_CONFIG_PATH,
                    )
                })?;
                StoredPackageInformation::new_from_json_reader(&mut config_file)
                    .with_context(|| {
                        format!(
                            "can't read the {} file in the compressed archive",
                            JSON_CONFIG_PATH,
                        )
                    })?
                    .into()
            };
            let identifier = package_information
                .identifier
                .as_ref()
                .context("the mod identifier is absent")?;
            let version = package_information
                .version
                .as_ref()
                .context("the mod version is absent")?;
            println!(
                "the mod is {:?} version {:?}. extracting",
                identifier, version
            );
            // 2.1
            let unpack_folder = store.assign_tempory_folder_for_package(identifier, version)?;
            // 2.2
            zip_reader
                .extract(&unpack_folder)
                .with_context(|| format!("can't decompress the package into {:?}", unpack_folder))
                .unwrap();
            // 2.3
            store
                .install_mod_by_move(&unpack_folder, &identifier, &version)
                .context("can't move the folder into the destination folder")?;
            println!("the mod {:?} is now installed", identifier);
            Ok(Package::load_from_folder(store.mod_folder(identifier, version)).context("internal error: unable to get information about the mod, even when it was successfully installed")?)
        }
    }
}
