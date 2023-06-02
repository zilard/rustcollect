extern crate tokio;
use tokio::io::*;
use tokio::process::Command;

use std::process::Stdio;

//#[tokio::main]
pub async fn run_process(
    text: String,
) -> std::result::Result<
    tokio::io::Lines<BufReader<tokio::process::ChildStdout>>,
    Box<dyn std::error::Error>,
> {
    let mut cmd = Command::new(text);
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let lines = BufReader::new(stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async {
        let status = child.await.expect("child process encountered an error");

        println!("child status was: {}", status);
    });
    Ok(lines)
}
