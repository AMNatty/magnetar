use anyhow::anyhow;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use tracing::info;

#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub struct MagnetarNetworking {
    pub host: String,
    pub port: u16,
    pub bind_addr: IpAddr,
    pub protocol: MagnetarNetworkingProtocol,
}

#[derive(Deserialize, Debug)]
pub enum MagnetarNetworkingProtocol {
    Http,
    Https,
}

impl AsRef<str> for MagnetarNetworkingProtocol {
    fn as_ref(&self) -> &str {
        match *self {
            MagnetarNetworkingProtocol::Http => "http",
            MagnetarNetworkingProtocol::Https => "https",
        }
    }
}

impl Display for MagnetarNetworkingProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

fn env_host() -> String {
    std::env::var("MAG_C_HOST")
        .expect("MAG_C_HOST or \"networking.host\" in the default configuration must be set")
}

fn env_bind_addr() -> IpAddr {
    std::env::var("MAG_C_BIND_ADDR")
        .unwrap_or_else(|_| "::".to_owned())
        .parse()
        .map_err(|e| format!("Failed to parse \"MAG_C_BIND_ADDR\": {e}"))
        .unwrap()
}

fn env_port() -> u16 {
    std::env::var("MAG_C_PORT")
        .unwrap_or_else(|_| "4939".to_owned())
        .parse()
        .expect("MAG_C_PORT must be a valid port number")
}

fn env_protocol() -> MagnetarNetworkingProtocol {
    match std::env::var("MAG_C_PROTOCOL")
        .unwrap_or_else(|_| "https".to_owned())
        .to_lowercase()
        .as_str()
    {
        "http" => MagnetarNetworkingProtocol::Http,
        "https" => MagnetarNetworkingProtocol::Https,
        _ => panic!("MAG_C_PROTOCOL must be a valid protocol"),
    }
}

impl Default for MagnetarNetworking {
    fn default() -> Self {
        MagnetarNetworking {
            host: env_host(),
            bind_addr: env_bind_addr(),
            port: env_port(),
            protocol: env_protocol(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub struct MagnetarBranding {
    pub name: String,
    pub version: String,
    pub homepage: String,
    pub repository: String,
}

fn env_brand_name() -> String {
    std::env::var("MAG_C_BR_NAME").unwrap_or_else(|_| "magnetar".to_owned())
}

fn env_brand_version() -> String {
    std::env::var("MAG_C_BR_VERSION").unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_owned())
}

fn env_brand_homepage() -> String {
    std::env::var("MAG_C_BR_HOMEPAGE")
        .unwrap_or_else(|_| "https://git.astolfo.cool/natty/magnetar".to_owned())
}

fn env_brand_repository() -> String {
    std::env::var("MAG_C_BR_REPOSITORY")
        .unwrap_or_else(|_| "https://git.astolfo.cool/natty/magnetar".to_owned())
}

impl Default for MagnetarBranding {
    fn default() -> Self {
        MagnetarBranding {
            name: env_brand_name(),
            version: env_brand_version(),
            homepage: env_brand_homepage(),
            repository: env_brand_repository(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub struct MagnetarData {
    pub database_url: String,
}

fn env_database_url() -> String {
    std::env::var("MAG_C_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("MAG_C_HOST, DATABASE_URL or \"data.database_url\" in the default configuration must be set")
}

impl Default for MagnetarData {
    fn default() -> Self {
        MagnetarData {
            database_url: env_database_url(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
#[non_exhaustive]
pub struct MagnetarConfig {
    #[serde(default)]
    pub networking: MagnetarNetworking,
    #[serde(default)]
    pub branding: MagnetarBranding,
    #[serde(default)]
    pub data: MagnetarData,
}

pub fn load_config() -> anyhow::Result<MagnetarConfig> {
    let path =
        std::env::var("MAG_CONFIG_PATH").unwrap_or_else(|_| "config/default.toml".to_owned());

    let str_cfg =
        std::fs::read_to_string(path).map_err(|e| anyhow!("Failed to load configuration: {e}"))?;

    let config =
        toml::from_str(&str_cfg).map_err(|e| anyhow!("Failed to parse configuration: {e}"))?;

    info!("Loaded configuration: {config:#?}");

    Ok(config)
}
