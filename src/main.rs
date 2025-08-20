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

    println!("Scanning for Ruuvi devices...");

    println!("Scan finished.");
    let mut events = adapter.discover_devices().await?;

    // Loop over events for ~5 seconds
    let stop_at = std::time::Instant::now() + std::time::Duration::from_secs(5);

    while let Some(event) = events.next().await {
        if std::time::Instant::now() > stop_at {
            break;
        }
        // AdapterEvent is not a Result, don't use ?
        if let bluer::AdapterEvent::DeviceAdded(addr) = event {
            // adapter.device() is not async, just unwrap or ? directly
            let device = adapter.device(addr.clone())?;

            // Filter by Ruuvi MAC prefix
            if device.address().to_string().starts_with("F1:CC:CA") {
                let name = device.name().await?.unwrap_or_default();
                let rssi = device.rssi().await?.unwrap_or(0);
                let manufacturer_data = device.manufacturer_data().await?.unwrap_or_default();

                println!("Ruuvi Device: {}", device.address());
                println!("  Name: {}", name);
                println!("  RSSI: {}", rssi);
                println!("  Manufacturer data: {:?}", manufacturer_data);
                println!("----------------------------------");
            } else {
                println!("Not ruuvi device: {}", device.address());
            }
        }
    }
    println!("Scan complete!");

    Ok(())
}
