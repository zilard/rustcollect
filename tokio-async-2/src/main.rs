use tokio::time::{sleep, Duration};
use tokio::io::AsyncReadExt;
use log::Level;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper(name: &str) {
    log::info!("{}: Sleeping", name);
    sleep(Duration::from_secs(1)).await;
    log::info!("{}: Awake!", name);
}

async fn reader() {
    log::info!("Reading some beeg data");
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes", contents.len());
    println!("{:?}", contents);

    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(40)");
        fib(40);
        log::info!("Done computing fib(40)");
    }).await.unwrap();
}

async fn run() {
    /*
    tokio::join!{
        sleeper(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
    };
    */
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

    /*
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();

    rt.block_on(future);
    */
}
