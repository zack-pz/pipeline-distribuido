use shared::SensorReading;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rand::Rng;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!(">>> SENSOR INICIADO (Intentando conectar al Edge...)");
    loop {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8001").await {
            let mut rng = rand::thread_rng();
            let reading = SensorReading {
                sensor_id: "sensor_angel".to_string(),
                value: rng.gen_range(15.0..40.0),
                timestamp_ms: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            };
            let json = serde_json::to_string(&reading).unwrap();
            stream.write_all(json.as_bytes()).await?;
            println!("Enviado al Edge: {:.2} C", reading.value);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
