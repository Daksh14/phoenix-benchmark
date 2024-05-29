// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use dusk_pki::Ownable;
use futures_util::stream::StreamExt;
use old_phoenix::transaction::TreeLeaf as OldTreeLeaf;
use phoenix_core::transaction::TreeLeaf;
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use reqwest::Body;
use sha2::{Digest, Sha256};
use std::io::Write;

use std::time::Instant;

const TRANSFER_CONTRACT: &str = "0100000000000000000000000000000000000000000000000000000000000000";

pub fn rng_with_index(seed: &[u8; 64], index: u64, termination: &[u8]) -> ChaCha12Rng {
    let mut hash = Sha256::new();

    hash.update(seed);
    hash.update(index.to_le_bytes());
    hash.update(termination);

    let hash = hash.finalize().into();
    ChaCha12Rng::from_seed(hash)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut all_notes = vec![];

    while let Some(http_chunk) = stream.next().await {
        all_notes.extend(http_chunk?.to_vec());
    }

    let seed: [u8; 64] = [
        62, 107, 200, 150, 136, 164, 160, 96, 51, 120, 7, 147, 214, 247, 92, 129, 153, 233, 162,
        121, 45, 209, 233, 4, 196, 182, 194, 50, 226, 95, 225, 223, 1, 209, 19, 127, 114, 244, 87,
        156, 95, 12, 108, 92, 35, 150, 211, 217, 255, 93, 119, 231, 14, 95, 93, 185, 216, 108, 32,
        89, 33, 225, 58, 184,
    ];

    let sk = phoenix_core::SecretKey::random(&mut rng_with_index(&seed, 0, b"SSK"));
    let vk = phoenix_core::ViewKey::from(&sk);

    let before = Instant::now();

    println!("Syncing with new phoenix core from genesis");

    all_notes.chunks_exact(632).for_each(|bytes| {
        let TreeLeaf { block_height, note } = match rkyv::from_bytes(bytes).ok() {
            Some(a) => a,
            None => {
                panic!("failed to deserialize note");
            }
        };

        vk.owns(&note);
    });

    println!("Syncing finished");
    println!("Elapsed time: {:.2?}", before.elapsed());

    let before = Instant::now();

    println!("Syncing with old phoenix core from genesis");

    let old_ssk = dusk_pki::SecretSpendKey::random(&mut rng_with_index(&seed, 0, b"SSK"));
    let old_vk = old_ssk.view_key();

    all_notes.chunks_exact(632).for_each(|bytes| {
        let OldTreeLeaf { block_height, note } = match rkyv::from_bytes(bytes).ok() {
            Some(a) => a,
            None => {
                panic!("failed to deserialize note");
            }
        };

        old_vk.owns(&note);
    });

    println!("Elapsed time: {:.2?}", before.elapsed());

    let before = Instant::now();

    all_notes.chunks_exact(632).for_each(|bytes| {
        let OldTreeLeaf { block_height, note } = match rkyv::from_bytes(bytes).ok() {
            Some(a) => a,
            None => {
                panic!("failed to deserialize note");
            }
        };
    });

    println!("Elapsed time for no check: {:.2?}", before.elapsed());

    Ok(())
}
