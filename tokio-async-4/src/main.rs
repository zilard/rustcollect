use tokio::time::{sleep, Duration};
use log::Level;

async fn sleepy() {
    log::info!("Sleeping");
    sleep(Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn do_something_fun() {
    log::info!("Doing something fun");
    sleep(Duration::from_secs(1)).await;
    log::info!("fun was had");
}

async fn run() {
    tokio::spawn(async {
        sleepy().await;
    });
    do_something_fun().await;
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Took {:?} seconds", end - start);
}
