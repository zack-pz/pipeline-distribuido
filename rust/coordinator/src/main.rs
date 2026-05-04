use shared::{EdgeReport, Heartbeat};
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use std::env;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = env::var("COORDINATOR_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:9001".to_string());
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("COORDINATOR VIGILANDO (Datos y Heartbeats)");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await?;
        let msg = &buf[..n];

        // Intentar deserializar como Reporte
        if let Ok(report) = serde_json::from_slice::<EdgeReport>(msg) {
            info!(tipo = "DATA", node = %report.edge_id, avg = %report.window_avg, anomaly = %report.anomaly_detected, latency_ms = %report.latency_ms, "Mensaje");
        } 
        // Si no es reporte, intentar como Heartbeat
        else if let Ok(hb) = serde_json::from_slice::<Heartbeat>(msg) {
            info!(tipo = "HEARTBEAT", node = %hb.node_id, ts = %hb.timestamp_ms, "Latido recibido");
        }
    }
}
