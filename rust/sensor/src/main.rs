use shared::SensorReading;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::env;
use rand::Rng;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let edge_addr = env::var("EDGE_ADDR").unwrap_or_else(|_| "127.0.0.1:8001".to_string());
    let sensor_id = env::var("SENSOR_ID").unwrap_or_else(|_| "sensor_angel".to_string());
    let interval_ms = env::var("INTERVAL_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(1000);

    println!(">>> SENSOR INICIADO (Intentando conectar al Edge...)");
    loop {
        if let Ok(mut stream) = TcpStream::connect(&edge_addr).await {
            let mut rng = rand::thread_rng();
            let reading = SensorReading {
                sensor_id: sensor_id.clone(),
                value: rng.gen_range(15.0..40.0),
                timestamp_ms: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                unit: "C".to_string(),
            };
            let json = serde_json::to_string(&reading).unwrap();
            stream.write_all(json.as_bytes()).await?;
            println!("Enviado al Edge: {:.2} C", reading.value);
        }
        tokio::time::sleep(Duration::from_millis(interval_ms)).await;
    }
}
