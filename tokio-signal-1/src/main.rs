use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("waiting for ctrl-c");
    
    signal::ctrl_c().await?;

    println!("received ctrl-c event");

    Ok(())
}