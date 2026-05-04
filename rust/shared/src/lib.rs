use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SensorReading {
    pub sensor_id: String,
    pub value: f32,
    pub timestamp_ms: u64,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EdgeReport {
    pub edge_id: String,
    pub window_avg: f32,
    pub anomaly_detected: bool,
    pub sample_count: u32,
    pub latency_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Heartbeat {
    pub node_id: String,
    pub role: String,
    pub timestamp_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CoordStatus {
    pub active_edges: u32,
    pub total_readings: u64,
    pub anomalies_last_min: u32,
    pub uptime_s: u64,
}
