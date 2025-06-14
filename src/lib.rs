#![deny(clippy::all)]

mod base64_to_bytes;

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
}
