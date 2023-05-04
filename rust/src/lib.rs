mod utils;

use wasm_bindgen::prelude::*;

use std::io::{Cursor, Read, Seek, SeekFrom,};

use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn decrypt_xchacha20poly1305(
    key: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(GenericArray::from_slice(&key));
    let xnonce = XNonce::from_slice(&nonce);

    let plaintext = cipher.decrypt(&xnonce, &ciphertext[..]);

    plaintext.unwrap_or(Vec::new())
}

#[wasm_bindgen]
pub fn hash_blake3(input: Vec<u8>) -> Vec<u8> {
    let digest = blake3::hash(&input);
    digest.as_bytes().to_vec()
}

#[wasm_bindgen]
pub fn verify_integrity(
    chunk_bytes: Vec<u8>,
    offset: u64,
    bao_outboard_bytes: Vec<u8>,
    blake3_hash: Vec<u8>,
) -> u8 {
    let res = verify_integrity_internal(
        chunk_bytes,
        offset,
        bao_outboard_bytes,
        from_vec_to_array(blake3_hash),
    );

    if res.is_err() {
        0
    }else{
        42
    }
}
pub fn from_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    core::convert::TryInto::try_into(v)
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn verify_integrity_internal(
    chunk_bytes: Vec<u8>,
    offset: u64,
    bao_outboard_bytes: Vec<u8>,
    blake3_hash: [u8; 32],
) -> anyhow::Result<u8> {
    let mut slice_stream = abao::encode::SliceExtractor::new_outboard(
        FakeSeeker::new(&chunk_bytes[..]),
        Cursor::new(&bao_outboard_bytes),
        offset,
        262144,
    );

    let mut decode_stream = abao::decode::SliceDecoder::new(
        &mut slice_stream,
        &abao::Hash::from(blake3_hash),
        offset,
        262144,
    );
    let mut decoded = Vec::new();
    decode_stream.read_to_end(&mut decoded)?;

    Ok(1)
}


struct FakeSeeker<R: Read> {
    reader: R,
    bytes_read: u64,
}

impl<R: Read> FakeSeeker<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            bytes_read: 0,
        }
    }
}

impl<R: Read> Read for FakeSeeker<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        self.bytes_read += n as u64;
        Ok(n)
    }
}

impl<R: Read> Seek for FakeSeeker<R> {
    fn seek(&mut self, _: SeekFrom) -> std::io::Result<u64> {
        // Do nothing and return the current position.
        Ok(self.bytes_read)
    }
}