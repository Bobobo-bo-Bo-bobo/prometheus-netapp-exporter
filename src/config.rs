use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Configuration {
    pub filer: Vec<NetAppConfiguration>,
}
#[derive(Deserialize, Clone)]
pub struct NetAppConfiguration {
    pub address: String,
    pub ca_cert: Option<String>,
    pub insecure_ssl: Option<bool>,
    pub name: String,
    pub timeout: Option<u32>,
    pub user: String,
    pub password: String,
}

impl std::fmt::Debug for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Configuration")
            .field("filer", &self.filer)
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
            .field("timeout", &self.timeout)
            .finish()
    }
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let unparsed = fs::read_to_string(f)?;
    let config: Configuration = serde_yaml::from_str(unparsed.as_str())?;

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
