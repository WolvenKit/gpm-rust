use super::commands;
use std::path::PathBuf;

use gpm_core::config::GpmConfig;
use tempfile::tempdir;

fn get_test_data_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("test_data");
    path.canonicalize().unwrap()
}

#[test]
fn test_command_create_install_package() {
    let test_dir = tempdir().unwrap();

    let mut config = GpmConfig::empty();
    config.store_path = test_dir.path().join("store");
    config.profile_path = test_dir.path().join("profile");
    config.can_change_profile = true;

    commands::init::init(&config).unwrap();

    let package_path = test_dir.path().join("test_mod.zip");
    commands::package::package(commands::package::PackageParameter {
        input_dir: PathBuf::from(get_test_data_dir().join("test_mod")),
        output_file: package_path.clone(),
    })
    .unwrap();

    commands::install_local::install_local(
        commands::install_local::InstallLocalParameter {
            input: package_path.to_path_buf(),
        },
        config,
    )
    .unwrap();
}
