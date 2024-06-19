use std::time::Instant;

use clap::Parser;
use phoenix_core::transaction::TreeLeaf as OldTreeLeaf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use benchmark::{Args, NOTE_SIZE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut file = File::open(args.file).await?;

    let mut notes = vec![];
    file.read_to_end(&mut notes).await?;

    let before = Instant::now();

    println!("Syncing with noop from genesis");

    notes.chunks_exact(NOTE_SIZE).for_each(|bytes| {
        let OldTreeLeaf {
            block_height: _,
            note: _,
        } = match rkyv::from_bytes(bytes).ok() {
            Some(a) => a,
            None => {
                panic!("failed to deserialize note");
            }
        };
    });

    println!("Elapsed time: {:.2?}", before.elapsed());

    Ok(())
}
