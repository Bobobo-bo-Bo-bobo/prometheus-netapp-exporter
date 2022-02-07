use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;

use log::{debug, error, warn};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct PortList {
    pub records: Vec<Port>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Port {
    pub node: PortNode,
    pub enabled: bool,
    pub name: String,
    pub state: String,
    pub statistics: Option<PortStatistics>,
    pub physical_protocol: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortNode {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortStatistics {
    pub status: String,
    pub throughput_raw: PortStatisticsThroughputRaw,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortStatisticsThroughputRaw {
    pub read: u64,
    pub write: u64,
    pub total: u64,
}

pub fn update_fibrechannel(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://{}{}?fields=**",
        filer.address,
        constants::API_FIBRECHANNEL
    );
    let raw_ports = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!(
                "Request for fibrechannel ports on {} failed - {}",
                filer.name,
                e
            );
        }
    };

    let port_list: PortList = match serde_json::from_str(&raw_ports) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for fibrechannel port information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    for port in port_list.records {
        debug!(
            "Updating metrics networking fibrechannel port state {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.state
        );
        let mut ok: bool = true;
        let mut startup: i64 = 0;
        let mut link_not_connected: i64 = 0;
        let mut online: i64 = 0;
        let mut link_disconnected: i64 = 0;
        let mut offlined_by_user: i64 = 0;
        let mut offlined_by_system: i64 = 0;
        let mut node_offline: i64 = 0;
        match port.state.as_str() {
            "startup" => {
                startup = 1;
            }
            "link_not_connected" => {
                link_not_connected = 1;
            }
            "online" => {
                online = 1;
            }
            "link_disconnected" => {
                link_disconnected = 1;
            }
            "offlined_by_user" => {
                offlined_by_user = 1;
            }
            "offlined_by_system" => {
                offlined_by_system = 1;
            }
            "node_offline" => {
                node_offline = 1;
            }
            _ => {
                error!(
                    "Invalid state value {} for fibrechannel port {} on node {} of filer {}",
                    port.state, port.name, port.node.name, filer.name
                );
                ok = false;
            }
        };
        if ok {
            exporter::FC_STATE
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "startup"])
                .set(startup);
            exporter::FC_STATE
                .with_label_values(&[
                    &filer.name,
                    &port.node.name,
                    &port.name,
                    "link_not_connected",
                ])
                .set(link_not_connected);
            exporter::FC_STATE
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "online"])
                .set(online);
            exporter::FC_STATE
                .with_label_values(&[
                    &filer.name,
                    &port.node.name,
                    &port.name,
                    "link_disconnected",
                ])
                .set(link_disconnected);
            exporter::FC_STATE
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "offlined_by_user"])
                .set(offlined_by_user);
            exporter::FC_STATE
                .with_label_values(&[
                    &filer.name,
                    &port.node.name,
                    &port.name,
                    "offlined_by_system",
                ])
                .set(offlined_by_system);
            exporter::FC_STATE
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "node_offline"])
                .set(node_offline);
        }

        debug!(
            "Updating metrics networking fibrechannel port enabled {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.enabled
        );
        if port.enabled {
            exporter::FC_ENABLED
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .set(1);
        } else {
            exporter::FC_ENABLED
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .set(0);
        }

        if let Some(stat) = port.statistics {
            if stat.status == "ok" {
                debug!("Updating metrcs for networking fibrechannel port statistics throughput_raw read {} {} {} -> {}", filer.name, port.node.name, port.name, stat.throughput_raw.read);
                let old_rx = exporter::FC_RX
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .get();
                if old_rx > stat.throughput_raw.read {
                    exporter::FC_RX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .reset();
                } else {
                    exporter::FC_RX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .inc_by(stat.throughput_raw.read - old_rx);
                }

                debug!("Updating metrcs for networking fibrechannel port statistics throughput_raw write {} {} {} -> {}", filer.name, port.node.name, port.name, stat.throughput_raw.write);
                let old_tx = exporter::FC_TX
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .get();
                if old_tx > stat.throughput_raw.write {
                    exporter::FC_TX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .reset();
                } else {
                    exporter::FC_TX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .inc_by(stat.throughput_raw.write - old_tx);
                }
            } else {
                warn!(
                    "Filer {} reports a state of {} for fibrechannel port {} on node {}",
                    filer.name, stat.status, port.name, port.node.name
                );
            }
        }

        debug!(
            "Updating metrics for networking fibrechannel port physical_protocol {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.physical_protocol
        );
        let mut ok: bool = true;
        let mut fibre_channel: i64 = 0;
        let mut ethernet: i64 = 0;
        match port.physical_protocol.as_str() {
            "fibre_channel" => {
                fibre_channel = 1;
            }
            "ethernet" => {
                ethernet = 1;
            }
            _ => {
                error!(
                    "Invalid value {} for physical_protocol of port {} on node {} of filer {}",
                    port.physical_protocol, port.name, port.node.name, filer.name
                );
                ok = false;
            }
        };
        if ok {
            exporter::FC_PHYS_PROTO
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "fibre_channel"])
                .set(fibre_channel);
            exporter::FC_PHYS_PROTO
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "ethernet"])
                .set(ethernet);
        }
    }
    Ok(())
}
