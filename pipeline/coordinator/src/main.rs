use shared::{EdgeReport, Heartbeat};
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    info!("COORDINATOR VIGILANDO (Datos y Heartbeats)");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await?;
        let msg = &buf[..n];

        // Intentar deserializar como Reporte
        if let Ok(report) = serde_json::from_slice::<EdgeReport>(msg) {
            info!(tipo = "DATA", node = %report.edge_id, avg = %report.window_avg, "Mensaje");
        } 
        // Si no es reporte, intentar como Heartbeat
        else if let Ok(hb) = serde_json::from_slice::<Heartbeat>(msg) {
            info!(tipo = "HEARTBEAT", node = %hb.node_id, ts = %hb.timestamp_ms, "Latido recibido");
        }
    }
}
