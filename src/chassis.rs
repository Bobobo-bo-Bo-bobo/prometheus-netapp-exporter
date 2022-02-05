use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;

use log::{debug, error};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct ChassisList {
    pub records: Vec<Chassis>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Chassis {
    pub id: String,
    pub state: String,
    pub shelves: Option<Vec<ChassisShelves>>,
    pub nodes: Option<Vec<ChassisNodes>>,
    pub frus: Option<Vec<ChassisFRU>>,
    pub usbs: Option<ChassisUSB>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChassisShelves {
    pub uid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChassisNodes {
    pub uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChassisFRU {
    pub state: String,
    pub id: String,
    #[serde(rename = "type")]
    pub fru_type: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChassisUSB {
    pub supported: bool,
    pub enabled: bool,
    pub ports: Vec<ChassisUSBPort>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChassisUSBPort {
    pub connected: bool,
}

pub fn update_chassis(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://{}{}?fields=**",
        filer.address,
        constants::API_CHASSIS
    );
    let raw_chassis = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!(
                "Request for cluster chassis on {} failed - {}",
                filer.name,
                e
            );
        }
    };

    let chassis_list: ChassisList = match serde_json::from_str(&raw_chassis) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for cluster chassis information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    for chassis in chassis_list.records {
        debug!(
            "Updating metrics for cluster chassis state {} {} -> {}",
            filer.name, chassis.id, chassis.state
        );
        let mut is_ok: bool = true;
        let mut ok: i64 = 0;
        let mut error: i64 = 0;
        match chassis.state.as_str() {
            "ok" => {
                ok = 1;
            }
            "error" => {
                error = 1;
            }
            _ => {
                error!(
                    "Invalid value {} for state of chassis {} of filer {}",
                    chassis.state, chassis.id, filer.name
                );
                is_ok = false;
            }
        };

        if is_ok {
            exporter::CHASSIS_STATE
                .with_label_values(&[&filer.name, &chassis.id, "ok"])
                .set(ok);
            exporter::CHASSIS_STATE
                .with_label_values(&[&filer.name, &chassis.id, "error"])
                .set(error);
        }

        if let Some(s) = chassis.shelves {
            debug!(
                "Updating metrics for cluster chassis shelves {} {} -> {}",
                filer.name,
                chassis.id,
                s.len()
            );
            exporter::CHASSIS_SHELVES
                .with_label_values(&[&filer.name, &chassis.id])
                .set(s.len() as i64);
        }

        if let Some(n) = chassis.nodes {
            debug!(
                "Updating metrics for cluster chassis nodes {} {} -> {}",
                filer.name,
                chassis.id,
                n.len()
            );
            exporter::CHASSIS_NODES
                .with_label_values(&[&filer.name, &chassis.id])
                .set(n.len() as i64);
        }

        if let Some(frus) = chassis.frus {
            for fru in frus {
                debug!(
                    "Updating metrics for cluster chssis frus {} {} {} {} -> {}",
                    filer.name, chassis.id, fru.id, fru.fru_type, fru.state
                );
                let mut is_ok: bool = true;
                let mut ok: i64 = 0;
                let mut error: i64 = 0;
                match fru.state.as_str() {
                    "ok" => {
                        ok = 1;
                    }
                    "error" => {
                        error = 1;
                    }
                    _ => {
                        error!(
                            "Invalid state {} of FRU {} on chassis {} of filer {}",
                            fru.state, fru.id, chassis.id, filer.name
                        );
                        is_ok = false;
                    }
                };

                if is_ok {
                    exporter::CHASSIS_FRU_STATE
                        .with_label_values(&[
                            &filer.name,
                            &chassis.id,
                            &fru.id,
                            "ok",
                            &fru.fru_type,
                        ])
                        .set(ok);
                    exporter::CHASSIS_FRU_STATE
                        .with_label_values(&[
                            &filer.name,
                            &chassis.id,
                            &fru.id,
                            "error",
                            &fru.fru_type,
                        ])
                        .set(error);
                }
            }
        }

        if let Some(usb) = chassis.usbs {
            debug!(
                "Updating metrics for cluster chassis usbs supported {} {} -> {}",
                filer.name, chassis.id, usb.supported
            );
            if usb.supported {
                exporter::CHASSIS_USB_SUPPORTED
                    .with_label_values(&[&filer.name, &chassis.id])
                    .set(1);
            } else {
                exporter::CHASSIS_USB_SUPPORTED
                    .with_label_values(&[&filer.name, &chassis.id])
                    .set(0);
            }

            debug!(
                "Updating metrics for cluster chassis usbs enabled {} {} -> {}",
                filer.name, chassis.id, usb.enabled
            );
            if usb.enabled {
                exporter::CHASSIS_USB_ENABLED
                    .with_label_values(&[&filer.name, &chassis.id])
                    .set(1);
            } else {
                exporter::CHASSIS_USB_ENABLED
                    .with_label_values(&[&filer.name, &chassis.id])
                    .set(0);
            }
            let mut connected: i64 = 0;
            let mut not_connected: i64 = 0;
            for port in usb.ports {
                if port.connected {
                    connected += 1;
                } else {
                    not_connected += 1;
                }
            }
            debug!(
                "Updating metrics cluster chssis usbs ports connected {} {} connected -> {}",
                filer.name, chassis.id, connected
            );
            exporter::CHASSIS_USB_PORT_CONNECTED
                .with_label_values(&[&filer.name, &chassis.id, "connected"])
                .set(connected);
            debug!(
                "Updating metrics cluster chssis usbs ports not_connected {} {} connected -> {}",
                filer.name, chassis.id, not_connected
            );
            exporter::CHASSIS_USB_PORT_CONNECTED
                .with_label_values(&[&filer.name, &chassis.id, "disconnected"])
                .set(not_connected);
        }
    }

    Ok(())
}
