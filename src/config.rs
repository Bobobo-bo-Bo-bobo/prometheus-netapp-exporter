use crate::constants;
use crate::http;
use crate::register;

use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Configuration {
    pub filer: Vec<NetAppConfiguration>,
    #[serde(skip)]
    pub register: ScrapeTargets,
    #[serde(skip)]
    pub register_mask: u64,
}

#[derive(Deserialize, Clone)]
pub struct NetAppConfiguration {
    pub address: String,
    pub ca_cert: Option<String>,
    pub insecure_ssl: Option<bool>,
    pub name: String,
    pub timeout: Option<u64>,
    pub user: String,
    pub password: String,
    pub targets: Option<ScrapeTargets>,
    #[serde(skip)]
    pub targets_mask: u64,
    #[serde(skip)]
    pub http_client: Option<reqwest::blocking::Client>,
}

#[derive(Clone, Default, Deserialize)]
pub struct ScrapeTargets {
    pub aggregates: Option<bool>,
    pub volumes: Option<bool>,
}

impl std::fmt::Debug for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Configuration")
            .field("filer", &self.filer)
            .field("register", &self.register)
            .field("register_mask", &self.register_mask)
            .finish()
    }
}

impl std::fmt::Debug for NetAppConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetAppConfiguration")
            .field("address", &self.address)
            .field("ca_cert", &self.ca_cert)
            .field("insecure_ssl", &self.insecure_ssl)
            .field("name", &self.name)
            .field("targets", &self.targets)
            .field("targets_mask", &self.targets_mask)
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl std::fmt::Debug for ScrapeTargets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScrapeTargets")
            .field("aggregates", &self.aggregates)
            .field("volumes", &self.volumes)
            .finish()
    }
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let unparsed = fs::read_to_string(f)?;
    let mut config: Configuration = serde_yaml::from_str(unparsed.as_str())?;

    for filer in config.filer.iter_mut() {
        if let Some(target) = &filer.targets {
            filer.targets_mask = register::build_target_masks(&target);
            //            register::targets(&mut config.register, &target);
            // Pre-build client structures
            let insecure_ssl = filer
                .insecure_ssl
                .unwrap_or(constants::DEFAULT_INSECURE_SSL);
            let ca_file = filer.ca_cert.clone().unwrap_or_default();
            let timeout_sec = filer.timeout.unwrap_or(constants::DEFAULT_TIMEOUT);
            filer.http_client = match http::build_client(insecure_ssl, &ca_file, timeout_sec) {
                Ok(v) => Some(v),
                Err(e) => {
                    bail!(
                        "can't build HTTP client structure for {}: {}",
                        filer.name,
                        e
                    );
                }
            };
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
