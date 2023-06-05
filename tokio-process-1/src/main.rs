use tokio::io::AsyncWriteExt;
use tokio::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = process::Command::new("sort");

    cmd.stdout(std::process::Stdio::piped());
    cmd.stdin(std::process::Stdio::piped());

    let mut child = cmd.spawn()?;

    let animals: &[&str] = &["dog", "bird", "frog", "cat", "fish"];

    // Child process
    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stding");

    // Write our animals to the child process
    stdin
        .write(animals.join("\n").as_bytes())
        .await
        .expect("could not write to stind");

    drop(stdin);

    let op = child.wait_with_output().await?;

    println!("sorted:\n\n{}", std::str::from_utf8(&op.stdout)?);
    Ok(())
}