#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::panic::PanicInfo;

use dusk_pki::SecretSpendKey;
use phoenix_core::transaction::TreeLeaf;
use phoenix_core::Note;

use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;

use sha2::{Digest, Sha256};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

const NOTE_SIZE: usize = 632;

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

#[no_mangle]
fn sync() {
    let notes_bytes = include_bytes!("../../notes.bin");

    let notes: Vec<Note> = notes_bytes
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

    let ssk = SecretSpendKey::random(&mut rng_with_index(0, b"SSK"));
    let vk = ssk.view_key();

    notes.into_iter().for_each(|note| {
        vk.owns(&note);
    });
}
