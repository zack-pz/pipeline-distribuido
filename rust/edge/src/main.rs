use shared::{SensorReading, EdgeReport, Heartbeat};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let edge_id = "edge_angel".to_string();
    let edge_id_clone = edge_id.clone();

    // TAREA SEPARADA: Heartbeat cada 3 segundos (Criterio B4)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3));
        loop {
            interval.tick().await;
            let hb = Heartbeat {
                node_id: edge_id_clone.clone(),
                role: "edge".to_string(),
                timestamp_ms: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            };
            
            // Intentar enviar latido al coordinador
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:9001").await {
                let json = serde_json::to_string(&hb).unwrap();
                let _ = stream.write_all(json.as_bytes()).await;
            }
        }
    });

    // FLUJO PRINCIPAL: Procesamiento de datos
    let listener = TcpListener::bind("127.0.0.1:8001").await?;
    println!(">>> EDGE NODE ACTIVO (Enviando Heartbeats cada 3s)");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await?;
        if let Ok(reading) = serde_json::from_slice::<SensorReading>(&buf[..n]) {
            let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
            let report = EdgeReport {
                edge_id: edge_id.clone(),
                window_avg: reading.value,
                anomaly_detected: reading.value > 35.0,
                sample_count: 1,
                latency_ms: now_ms.saturating_sub(reading.timestamp_ms),
            };

            if let Ok(mut coord_stream) = TcpStream::connect("127.0.0.1:9001").await {
                let json = serde_json::to_string(&report).unwrap();
                let _ = coord_stream.write_all(json.as_bytes()).await;
            }
        }
    }
}
