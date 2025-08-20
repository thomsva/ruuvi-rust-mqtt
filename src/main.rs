use bluer;
use tokio_stream::StreamExt; // brings .next() into scope

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a session (connects to bluez)
    let session = bluer::Session::new().await?;

    // Use a specific adapter ("hci0" is usually the first one)
    let adapter = session.adapter("hci0")?;

    println!("Adapter name: {}", adapter.name());
    println!("Address:      {}", adapter.address().await?);
    println!("Powered:      {}", adapter.is_powered().await?);
    println!("Discoverable: {}", adapter.is_discoverable().await?);

    println!("Scanning for devices...");

    let mut events = adapter.discover_devices().await?;

    // Loop over events for ~10 seconds
    let stop_at = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while let Some(event) = events.next().await {
        if std::time::Instant::now() > stop_at {
            break;
        }
        println!("{:?}", event);
    }

    println!("Scan finished.");

    Ok(())
}
