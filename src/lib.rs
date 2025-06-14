#![deny(clippy::all)]

mod base64_to_bytes;
mod bytes_to_base64;
mod hex_to_bytes;
mod bytes_to_hex;
mod words_to_bytes;
mod bytes_to_words;
mod random_bytes;
mod rotl;
mod rotr;

#[macro_use]
extern crate napi_derive;

const BASE64_MAP: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[napi]
pub struct Crypt {}

fn clean_base64(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/' => result.push(c),
            _ => {}
        }
    }

    result
}

#[napi]
impl Crypt {
    #[napi]
    pub fn base64_to_bytes(base64_str: String) -> Vec<u8> {
        base64_to_bytes::base64_to_bytes(base64_str)
    }

    #[napi]
    pub fn bytes_to_base64(bytes: &[u8]) -> String {
        bytes_to_base64::bytes_to_base64(bytes)
    }

    #[napi]
    pub fn hex_to_bytes(hex: String) -> Vec<u8> {
        hex_to_bytes::hex_to_bytes(&hex)
    }

    #[napi]
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes_to_hex::bytes_to_hex(bytes)
    }

    #[napi]
    pub fn words_to_bytes(words: Vec<u32>) -> Vec<u8> {
        words_to_bytes::words_to_bytes(&words)
    }

    #[napi]
    pub fn bytes_to_words(bytes: &[u8]) -> Vec<u32> {
        bytes_to_words::bytes_to_words(bytes)
    }

    #[napi]
    pub fn random_bytes(len: u32) -> Vec<u8> {
        random_bytes::random_bytes(len as usize)
    }

    #[napi]
    pub fn rotl(value: u32, shift: u32) -> u32 {
        rotl::rotl(value, shift)
    }

    #[napi]
    pub fn rotr(value: u32, shift: u32) -> u32 {
        rotr::rotr(value, shift)
    }
}
