use crate::register;

use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub filer: Vec<NetAppConfiguration>,
    #[serde(skip)]
    pub register: ScrapeTargets,
    #[serde(skip)]
    pub register_mask: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NetAppConfiguration {
    pub address: String,
    pub ca_cert: Option<String>,
    pub insecure_ssl: Option<bool>,
    pub name: String,
    pub password: String,
    pub targets: Option<ScrapeTargets>,
    #[serde(skip)]
    pub targets_mask: u64,
    pub timeout: Option<u64>,
    pub user: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ScrapeTargets {
    pub aggregates: Option<bool>,
    pub chassis: Option<bool>,
    pub cifs: Option<ScrapeTargetCIFS>,
    pub ethernet: Option<bool>,
    pub fibrechannel: Option<bool>,
    pub jobs: Option<bool>,
    pub nfs: Option<ScrapeTargetNFS>,
    pub quotas: Option<bool>,
    pub volumes: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ScrapeTargetCIFS {
    pub client_ip: Option<bool>,
    pub mapped_user: Option<bool>,
    pub user: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ScrapeTargetNFS {
    pub client_ip: Option<bool>,
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let unparsed = fs::read_to_string(f)?;
    let mut config: Configuration = serde_yaml::from_str(unparsed.as_str())?;

    for filer in config.filer.iter_mut() {
        if let Some(target) = &filer.targets {
            filer.targets_mask = register::build_target_masks(target);
        }
    }

    validate_configuration(&config)?;

    Ok(config)
}

fn validate_configuration(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    for filer in &cfg.filer {
        if filer.address.is_empty() {
            bail!("address is not configured");
        }
        if filer.name.is_empty() {
            bail!("name is not configured");
        }
        if let Some(t) = filer.timeout {
            if t == 0 {
                bail!("illegal value for timeout: 0");
            }
        }
    }

    Ok(())
}
