use std::os::raw::c_void;
use crate::xxhash_bindings::XXH3_64bits_withSecret;

pub fn xxh3_u64_secret(dat: &[u8], secret: &[u8]) -> u64 {
    unsafe {
        XXH3_64bits_withSecret(
            dat.as_ptr() as *const c_void, dat.len(),
            secret.as_ptr() as *const c_void, secret.len())
    }
}
