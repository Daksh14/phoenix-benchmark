use std::time::Instant;

use clap::Parser;
use dusk_jubjub::{JubJubExtended, JubJubScalar, GENERATOR};
use ff::Field;
use phoenix_core::transaction::TreeLeaf;
use phoenix_core::Note;
use phoenix_core_sync::{SecretKey, ViewKey};
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use benchmark::{rng_with_index, Args, NOTE_SIZE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut file = File::open(args.file).await?;

    let mut rng = ChaChaRng::seed_from_u64(0xBAD);

    let mut notes = vec![];
    file.read_to_end(&mut notes).await?;
    let notes: Vec<(Note, JubJubExtended, JubJubExtended)> = notes
        .chunks_exact(NOTE_SIZE)
        .into_iter()
        .map(|chunk| {
            let TreeLeaf {
                note,
                block_height: _,
            } = rkyv::from_bytes(chunk).unwrap();

            let r = GENERATOR * JubJubScalar::random(&mut rng);
            let k = GENERATOR * JubJubScalar::random(&mut rng);

            (note, r, k)
        })
        .collect();

    let sk = SecretKey::random(&mut rng_with_index(0, b"SSK"));
    let vk = ViewKey::from(&sk);

    let before = Instant::now();

    println!("Syncing with new phoenix core from genesis");

    notes.into_iter().for_each(|(_, r, k)| {
        let ar = r * vk.a();
        let _ = k == ar;
    });

    println!("Syncing finished");
    println!("Elapsed time: {:.2?}", before.elapsed());

    Ok(())
}
