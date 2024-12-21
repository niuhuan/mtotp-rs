extern crate hmac;

use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::error::*;

pub fn hotp(secret: &[u8], counter: u64, digits: usize) -> Result<String> {
    let counter = counter.to_be_bytes();
    let mut hmac = Hmac::<Sha1>::new_from_slice(secret.as_ref())?;
    hmac.update(&counter);
    let finalize = hmac.finalize().into_bytes();
    let slice = finalize.as_slice();
    let offset = (slice[19] & 0xf) as usize;
    let code = ((slice[offset] & 0x7f) as u32) << 24 |
        ((slice[offset + 1] & 0xff) as u32) << 16 |
        ((slice[offset + 2] & 0xff) as u32) << 8 |
        ((slice[offset + 3] & 0xff) as u32);
    let code = code % 10u32.pow(digits as u32);
    Ok(format!("{:0>width$}", code, width = digits))
}

pub fn totp(secret: &str, time_step: u64, digits: usize) -> Result<String> {
    let counter = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() / time_step;
    let secret = match base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret) {
        None => return Err(Error::message("secret not base32")),
        Some(data) => data,
    };
    hotp(secret.as_slice(), counter, digits)
}

