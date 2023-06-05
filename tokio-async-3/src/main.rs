use tokio::time::{sleep, Duration};
use log::Level;

async fn sleeper(name: &str) {
    log::info!("{}: Sleeping", name);
    sleep(Duration::from_secs(1)).await;
    log::info!("{}: Awake!", name);
}

async fn run() {
    sleeper("first");
    sleeper("second").await;
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Took {:?} seconds", end - start);
}
