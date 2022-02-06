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
    pub mac_address: String,
    pub enabled: bool,
    pub speed: i64,
    pub mtu: i64,
    pub name: String,
    pub state: String,
    #[serde(rename = "type")]
    pub port_type: String,
    pub statistics: Option<PortStatistics>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortNode {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortStatistics {
    pub status: String,
    pub device: PortStatisticsDevice,
    pub throughput_raw: PortStatisticsThroughputRaw,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortStatisticsThroughputRaw {
    pub read: u64,
    pub write: u64,
    pub total: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortStatisticsDevice {
    pub receive_raw: PortStatisticsCounters,
    pub transmit_raw: PortStatisticsCounters,
    pub link_down_count_raw: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PortStatisticsCounters {
    pub errors: u64,
    pub discards: u64,
    pub packets: u64,
}

pub fn update_ethernet(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://{}{}?fields=**",
        filer.address,
        constants::API_ETHERNET
    );
    let raw_ports = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!(
                "Request for ethernet ports on {} failed - {}",
                filer.name,
                e
            );
        }
    };

    let port_list: PortList = match serde_json::from_str(&raw_ports) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for ethernet port information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    for port in port_list.records {
        debug!(
            "Updating metrics for networking ethernet port enabled {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.enabled
        );
        if port.enabled {
            exporter::ETHERNET_ENABLED
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .set(1);
        } else {
            exporter::ETHERNET_ENABLED
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .set(0);
        }

        debug!(
            "Updating metrics for networking ethernet port speed {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.speed
        );
        exporter::ETHERNET_SPEED
            .with_label_values(&[&filer.name, &port.node.name, &port.name])
            .set(port.speed * (1024 ^ 2));

        debug!(
            "Updating metrics for networking ethernet port mtu {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.mtu
        );
        exporter::ETHERNET_MTU
            .with_label_values(&[&filer.name, &port.node.name, &port.name])
            .set(port.mtu);

        debug!(
            "Updating metrics for networking etthernet port state {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.state
        );
        let mut up: i64 = 0;
        let mut ok: bool = true;
        match port.state.as_str() {
            "up" => {
                up = 1;
            }
            "down" => {
                up = 0;
            }
            _ => {
                error!(
                    "Invalid port state {} for ethernet port {} on node {} of filer {}",
                    port.state, port.name, port.node.name, filer.name
                );
                ok = false;
            }
        };
        if ok {
            exporter::ETHERNET_UP
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .set(up);
        }

        debug!(
            "Updating metrics for networking ethernet port type {} {} {} -> {}",
            filer.name, port.node.name, port.name, port.port_type
        );
        let mut vlan: i64 = 0;
        let mut physical: i64 = 0;
        let mut lag: i64 = 0;
        ok = true;
        match port.port_type.as_str() {
            "vlan" => {
                vlan = 1;
            }
            "physical" => {
                physical = 1;
            }
            "lag" => {
                lag = 1;
            }
            _ => {
                error!(
                    "Invalid value {} for port type of port {} on node {} of filer {}",
                    port.port_type, port.name, port.node.name, filer.name
                );
                ok = false;
            }
        };
        if ok {
            exporter::ETHERNET_TYPE
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "vlan"])
                .set(vlan);
            exporter::ETHERNET_TYPE
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "physical"])
                .set(physical);
            exporter::ETHERNET_TYPE
                .with_label_values(&[&filer.name, &port.node.name, &port.name, "lag"])
                .set(lag);
        }

        if let Some(stat) = port.statistics {
            if stat.status == "ok" {
                debug!("Updating metrics for networking ethernet stattistics throughput_raw read {} {} {} -> {}", filer.name, port.node.name, port.name, stat.throughput_raw.read);
                let old_rx = exporter::ETHERNET_RX
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .get();
                if old_rx > stat.throughput_raw.read {
                    // XXX: Because we don't know the type/maximal value of the counter used by the filer, we can't calculate the new value. Just reset it.
                    exporter::ETHERNET_RX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .reset();
                } else {
                    exporter::ETHERNET_RX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .inc_by(stat.throughput_raw.read - old_rx)
                }

                debug!("Updating metrics for networking ethernet stattistics throughput_raw write {} {} {} -> {}", filer.name, port.node.name, port.name, stat.throughput_raw.write);
                let old_tx = exporter::ETHERNET_TX
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .get();
                if old_tx > stat.throughput_raw.write {
                    // XXX: Because we don't know the type/maximal value of the counter used by the filer, we can't calculate the new value. Just reset it.
                    exporter::ETHERNET_TX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .reset();
                } else {
                    exporter::ETHERNET_TX
                        .with_label_values(&[&filer.name, &port.node.name, &port.name])
                        .inc_by(stat.throughput_raw.read - old_tx)
                }
            } else {
                warn!(
                    "Filer {} reports a state of {} for ethernet port {} on node {}",
                    filer.name, stat.status, port.name, port.node.name
                );
            }

            debug!("Updating metrics for networking ethernet statistics device receive_raw errors {} {} {} -> {}", filer.name, port.node.name, port.name, stat.device.receive_raw.errors);
            let old_rx_err = exporter::ETHERNET_RX_ERROR
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .get();
            if old_rx_err > stat.device.receive_raw.errors {
                exporter::ETHERNET_RX_ERROR
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .reset();
            } else {
                exporter::ETHERNET_RX_ERROR
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .inc_by(stat.device.receive_raw.errors - old_rx_err);
            }

            debug!("Updating metrics for networking ethernet statistics device receive_raw discards {} {} {} -> {}", filer.name, port.node.name, port.name, stat.device.receive_raw.discards);
            let old_rx_dsc = exporter::ETHERNET_RX_DISCARD
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .get();
            if old_rx_dsc > stat.device.receive_raw.discards {
                exporter::ETHERNET_RX_DISCARD
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .reset();
            } else {
                exporter::ETHERNET_RX_DISCARD
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .inc_by(stat.device.receive_raw.errors - old_rx_dsc);
            }

            debug!("Updating metrics for networking ethernet statistics device receive_raw packets {} {} {} -> {}", filer.name, port.node.name, port.name, stat.device.receive_raw.packets);
            let old_rx_pck = exporter::ETHERNET_RX_PACKET
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .get();
            if old_rx_pck > stat.device.receive_raw.packets {
                exporter::ETHERNET_RX_PACKET
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .reset();
            } else {
                exporter::ETHERNET_RX_PACKET
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .inc_by(stat.device.receive_raw.packets - old_rx_pck);
            }

            debug!("Updating metrics for networking ethernet statistics device transmit_raw errors {} {} {} -> {}", filer.name, port.node.name, port.name, stat.device.transmit_raw.errors);
            let old_tx_err = exporter::ETHERNET_TX_ERROR
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .get();
            if old_tx_err > stat.device.transmit_raw.errors {
                exporter::ETHERNET_TX_ERROR
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .reset();
            } else {
                exporter::ETHERNET_TX_ERROR
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .inc_by(stat.device.transmit_raw.errors - old_tx_err);
            }

            debug!("Updating metrics for networking ethernet statistics device transmit_raw discards {} {} {} -> {}", filer.name, port.node.name, port.name, stat.device.transmit_raw.discards);
            let old_tx_dsc = exporter::ETHERNET_TX_DISCARD
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .get();
            if old_tx_dsc > stat.device.transmit_raw.discards {
                exporter::ETHERNET_TX_DISCARD
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .reset();
            } else {
                exporter::ETHERNET_TX_DISCARD
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .inc_by(stat.device.transmit_raw.errors - old_tx_dsc);
            }

            debug!("Updating metrics for networking ethernet statistics device transmit_raw packets {} {} {} -> {}", filer.name, port.node.name, port.name, stat.device.transmit_raw.packets);
            let old_tx_pck = exporter::ETHERNET_TX_PACKET
                .with_label_values(&[&filer.name, &port.node.name, &port.name])
                .get();
            if old_tx_pck > stat.device.transmit_raw.packets {
                exporter::ETHERNET_TX_PACKET
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .reset();
            } else {
                exporter::ETHERNET_TX_PACKET
                    .with_label_values(&[&filer.name, &port.node.name, &port.name])
                    .inc_by(stat.device.transmit_raw.packets - old_tx_pck);
            }
        }
    }
    Ok(())
}
