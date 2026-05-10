use base64::Engine;
use rand::RngCore;
use sha2::{Digest, Sha256};

pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);

    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_token(token: &str) -> String {
    let hash = Sha256::digest(token.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

pub fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;

    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }

    result == 0
}
