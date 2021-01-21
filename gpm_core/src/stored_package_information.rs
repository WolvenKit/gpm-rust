use crate::package_information::{PackageInformation, PackageInformationExtraData};
/// This module contain tool used to read and write config.toml and config.json
use std::{collections::BTreeSet, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StoredPackageInformation {
    #[serde(default)]
    creator: Option<String>,
    #[serde(default)]
    identifier: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    license: Option<String>,

    #[serde(default)]
    website_url: Option<String>,
    #[serde(default)]
    dependencies: BTreeSet<String>,
    #[serde(default)]
    tags: BTreeSet<String>,
    #[serde(default)]
    install_strategies: BTreeSet<String>,
    #[serde(default)]
    extra_data: Vec<(String, String)>,
}

impl Into<PackageInformation> for StoredPackageInformation {
    fn into(mut self) -> PackageInformation {
        let extra_data = self
            .extra_data
            .drain(..)
            .map(|(key, value)| PackageInformationExtraData { key, value })
            .collect();

        PackageInformation {
            creator: self.creator,
            identifier: self.identifier,
            version: self.version,
            display_name: self.display_name,
            description: self.description,
            license: self.license,
            website_url: self.website_url,
            dependencies: self.dependencies,
            tags: self.tags,
            install_strategies: self.install_strategies,
            extra_data,
        }
    }
}

impl From<&PackageInformation> for StoredPackageInformation {
    fn from(package: &PackageInformation) -> Self {
        let extra_data = package
            .extra_data
            .iter()
            .map(|data| (data.key.clone(), data.value.clone()))
            .collect();
        Self {
            creator: package.creator.clone(),
            identifier: package.identifier.clone(),
            version: package.version.clone(),
            display_name: package.display_name.clone(),
            description: package.description.clone(),
            license: package.license.clone(),
            website_url: package.website_url.clone(),
            dependencies: package.dependencies.clone(),
            tags: package.tags.clone(),
            install_strategies: package.install_strategies.clone(),
            extra_data,
        }
    }
}

impl StoredPackageInformation {
    pub fn new_from_json_reader(reader: &mut impl Read) -> anyhow::Result<Self> {
        Ok(serde_json::from_reader::<_, Self>(reader)?)
    }

    pub fn new_from_toml_reader(reader: &mut impl Read) -> anyhow::Result<Self> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(toml::from_slice(&buffer)?)
    }

    pub fn generate_json(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&self)
    }

    pub fn generate_toml(&self) -> Result<Vec<u8>, toml::ser::Error> {
        toml::to_vec(&self)
    }
}
