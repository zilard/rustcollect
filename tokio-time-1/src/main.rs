use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let duration = time::Duration::from_secs(1);
    let mut when = time::interval(duration);

    when.tick().await; // Ticks immediately
    println!("Tick 1");
    when.tick().await; // Ticks after 1s
    println!("Tick 2");
    when.tick().await; // Ticks after 1s
    println!("Tick 3");

    Ok(())
}