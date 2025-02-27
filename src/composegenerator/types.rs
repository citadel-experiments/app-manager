#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::composegenerator::output::types::ComposeSpecification;

// General types also relevant for the output
// Can be re-used by schemas

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(untagged)]
pub enum Permissions {
    OneDependency(String),
    AlternativeDependency(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct OutputMetadata {
    /// The app id, only set in output
    pub id: String,
    /// The name of the app
    pub name: String,
    /// The version of the app
    pub version: String,
    /// The category for the app
    pub category: String,
    /// A short tagline for the app
    pub tagline: String,
    // Developer name -> their website
    pub developers: BTreeMap<String, String>,
    /// A description of the app
    pub description: String,
    #[serde(default)]
    /// Permissions the app requires
    pub permissions: Vec<Permissions>,
    /// App repository name -> repo URL
    pub repo: BTreeMap<String, String>,
    /// A support link for the app
    pub support: String,
    /// A list of promo images for the apps
    pub gallery: Option<Vec<String>>,
    /// The path the "Open" link on the dashboard should lead to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The app's default username
    pub default_username: Option<String>,
    /// The app's default password. Can also be $APP_SEED for a random password
    pub default_password: Option<String>,
    #[serde(default = "bool::default")]
    /// True if the app only works over Tor
    pub tor_only: bool,
    /// A list of containers to update automatically (still validated by the Citadel team)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_containers: Option<Vec<String>>,
    /// For "virtual" apps, the service the app implements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control: Option<String>,
    /// True if all dependencies are installed
    pub compatible: bool,
    /// If compatible is false, the dependencies that are missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_dependencies: Option<Vec<Permissions>>,
    pub port: u16,
    pub internal_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_notes: Option<BTreeMap<String, String>>,
    pub supports_https: bool,
    pub hidden_services: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CaddyEntry {
    pub public_port: u16,
    pub internal_port: u16,
    pub container_name: String,
    pub is_primary: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ResultYml {
    pub new_tor_entries: String,
    pub new_i2p_entries: String,
    pub caddy_entries: Vec<CaddyEntry>,
    pub spec: ComposeSpecification,
    pub metadata: OutputMetadata,
}
