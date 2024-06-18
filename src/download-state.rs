use clap::Parser;
use futures_util::StreamExt;
use reqwest::Body;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use benchmark::Args;

const TRANSFER_CONTRACT: &str = "0100000000000000000000000000000000000000000000000000000000000000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut file = File::create(args.file).await?;

    let client = reqwest::Client::new();
    let uri = "https://nodes.dusk.network";
    let target_type = 1;
    let target = TRANSFER_CONTRACT;

    let buffer = Vec::from([
        15, 0, 0, 0, 108, 101, 97, 118, 101, 115, 95, 102, 114, 111, 109, 95, 112, 111, 115, 0, 0,
        0, 0, 0, 0, 0, 0,
    ]);

    let mut request = client
        .post(format!("{uri}/{target_type}/{target}"))
        .body(Body::from(buffer))
        .header("Content-Type", "application/octet-stream")
        .header("rusk-version", "0.7.0");

    request = request.header("Rusk-Feeder", "1");

    let response = request.send().await?;
    let mut stream = response.bytes_stream();

    let mut n = 0;
    while let Some(http_chunk) = stream.next().await {
        n += 1;
        file.write_all(&http_chunk?.to_vec()).await?;
    }
    println!("Wrote {n} Notes");

    Ok(())
}
