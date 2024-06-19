use std::time::Instant;

use clap::Parser;
use old_phoenix::transaction::TreeLeaf as OldTreeLeaf;
use old_phoenix::Note;
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
            let OldTreeLeaf {
                note,
                block_height: _,
            } = rkyv::from_bytes(chunk).unwrap();
            note
        })
        .collect();

    let old_ssk = dusk_pki::SecretSpendKey::random(&mut rng_with_index(0, b"SSK"));
    let old_vk = old_ssk.view_key();

    let before = Instant::now();

    println!("Syncing with old phoenix core from genesis");

    notes.into_iter().for_each(|note| {
        old_vk.owns(&note);
    });

    println!("Elapsed time: {:.2?}", before.elapsed());

    Ok(())
}
