use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SensorReading {
    pub sensor_id: String,
    pub value: f32,
    pub timestamp_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EdgeReport {
    pub edge_id: String,
    pub window_avg: f32,
    pub anomaly: bool,
    pub timestamp_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Heartbeat {
    pub node_id: String,
    pub role: String,
    pub timestamp_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CoordStatus {
    pub coord_id: String,
    pub active_nodes: u32,
    pub system_load: f32,
}
