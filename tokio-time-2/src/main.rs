use tokio::time;

async fn sleepy() {
    log::info!("Starting sleepy");
    time::sleep(time::Duration::from_secs(10)).await;
    log::info!("Ending sleepy");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;

    if let Err(_) = time::timeout(
        time::Duration::from_secs(2), sleepy(),
    ).await {
        log::info!("Sleepy timed out")
    }

    Ok(())
}