use tokio::io::AsyncReadExt;
use log::Level;

async fn sleeper() {
    log::info!("Sleeping");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}


async fn reader() {
    log::info("Reading some beeg data");
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();

    rt.block_on(future);
}