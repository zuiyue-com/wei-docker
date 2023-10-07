// use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub names: String,
    pub image: String,
    pub created: String,
    pub status: String,
    pub ports: String,
    pub size: String
}

#[derive(Serialize, Deserialize)]
pub struct ContainerStat {
    pub id: String,
    pub name: String,
    pub gpu: String,
    pub ip_address: String,
    pub cpu_percentage: String,
    pub mem_usage: String,
    pub mem_limit: String,
    pub mem_percentage: String,
    pub net_io: String,
    pub block_io: String,
    pub pids: String,
}

// #[derive(Deserialize)]
// pub struct State {
//     #[serde(rename = "Status")]
//     pub status: String,
// }

// #[derive(Deserialize)]
// pub struct DeviceRequest {
//     pub Driver: String,
//     pub Count: u32,
//     pub DeviceIDs: Vec<String>,
//     pub Capabilities: Vec<Vec<String>>,
//     pub Options: HashMap<String, String>,
// }

// #[derive(Deserialize)]
// pub struct HostConfig {
//     pub DeviceRequests: Option<Vec<DeviceRequest>>,
// }

// #[derive(Deserialize)]
// pub struct ContainerInspect {
//     #[serde(rename = "State")]
//     pub state: State,
//     #[serde(rename = "HostConfig")]
//     pub host_config: HostConfig,
// }