use std::time::Instant;

use clap::Parser;
use phoenix_core::transaction::TreeLeaf;
use phoenix_core::Note;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use benchmark::{rng_with_index, Args, NOTE_SIZE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut file = File::open(args.file).await?;

    let mut notes = vec![];
    file.read_to_end(&mut notes).await?;
    let notes: Vec<Note> = notes
        .chunks_exact(NOTE_SIZE)
        .into_iter()
        .map(|chunk| {
            let TreeLeaf {
                note,
                block_height: _,
            } = rkyv::from_bytes(chunk).unwrap();
            note
        })
        .collect();

    let sk = phoenix_core::SecretKey::random(&mut rng_with_index(0, b"SSK"));
    let vk = phoenix_core::ViewKey::from(&sk);

    let before = Instant::now();

    println!("Syncing with new phoenix core from genesis");

    notes.into_iter().for_each(|note| {
        vk.owns(&note);
    });

    println!("Syncing finished");
    println!("Elapsed time: {:.2?}", before.elapsed());

    Ok(())
}
