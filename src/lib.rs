use std::path::PathBuf;

use clap::Parser;
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use sha2::{Digest, Sha256};

pub const NOTE_SIZE: usize = 632;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The file to write the notes to
    #[clap(default_value = "notes.bin")]
    pub file: PathBuf,
}

const SEED: [u8; 64] = [
    62, 107, 200, 150, 136, 164, 160, 96, 51, 120, 7, 147, 214, 247, 92, 129, 153, 233, 162, 121,
    45, 209, 233, 4, 196, 182, 194, 50, 226, 95, 225, 223, 1, 209, 19, 127, 114, 244, 87, 156, 95,
    12, 108, 92, 35, 150, 211, 217, 255, 93, 119, 231, 14, 95, 93, 185, 216, 108, 32, 89, 33, 225,
    58, 184,
];

pub fn rng_with_index(index: u64, termination: &[u8]) -> ChaCha12Rng {
    let mut hash = Sha256::new();

    hash.update(&SEED);
    hash.update(index.to_le_bytes());
    hash.update(termination);

    let hash = hash.finalize().into();
    ChaCha12Rng::from_seed(hash)
}
