pub mod display;
pub mod install;
pub mod lockfile;
pub mod mod_storage;
pub mod package;
pub mod package_writer;
pub mod store_project;
pub mod tool;

pub mod constants {
    pub const TOML_CONFIG_PATH: &str = "config.toml";
    pub const JSON_CONFIG_PATH: &str = "config.json";
    pub const IGNORE_PATH: &str = ".modignore";
    pub const MOD_STORAGE_SEPARATOR: &str = "|";
    pub const STORAGE_TEMPORY_FOLDER_NAME: &str = ".tmp";
}
