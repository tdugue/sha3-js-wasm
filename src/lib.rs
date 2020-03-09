use sha3::Digest;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sha224(a: &str) -> String {
    hex::encode(sha3::Sha3_224::digest(a.as_bytes()).to_vec())
}

#[wasm_bindgen]
pub fn sha256(a: &str) -> String {
    hex::encode(sha3::Sha3_256::digest(a.as_bytes()).to_vec())
}

#[wasm_bindgen]
pub fn sha384(a: &str) -> String {
    hex::encode(sha3::Sha3_384::digest(a.as_bytes()).to_vec())
}

#[wasm_bindgen]
pub fn sha512(a: &str) -> String {
    hex::encode(sha3::Sha3_512::digest(a.as_bytes()).to_vec())
}

#[wasm_bindgen]
pub fn keccak224(a: &str) -> String {
    hex::encode(sha3::Keccak224::digest(a.as_bytes()).to_vec())
}

#[wasm_bindgen]
pub fn keccak256(a: &str) -> String {
    hex::encode(sha3::Keccak256::digest(a.as_bytes()).to_vec())
}

#[wasm_bindgen]
pub fn keccak384(a: &str) -> String {
    hex::encode(sha3::Keccak384::digest(a.as_bytes()).to_vec())
}

#[wasm_bindgen]
pub fn keccak512(a: &str) -> String {
    hex::encode(sha3::Keccak512::digest(a.as_bytes()).to_vec())
}
