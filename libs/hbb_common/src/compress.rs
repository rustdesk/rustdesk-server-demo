use std::cell::RefCell;
use zstd::bulk::{Compressor, Decompressor};
use zstd::stream::{Encoder, Decoder};

thread_local! {
    static COMPRESSOR: RefCell<Compressor<'static>> = RefCell::new(Compressor::new(0).unwrap());
    static DECOMPRESSOR: RefCell<Decompressor<'static>> = RefCell::new(Decompressor::new().unwrap());
}

/// The library supports regular compression levels from 1 up to ZSTD_maxCLevel(),
/// which is currently 22. Levels >= 20
/// Default level is ZSTD_CLEVEL_DEFAULT==3.
/// value 0 means default, which is controlled by ZSTD_CLEVEL_DEFAULT
pub fn compress(data: &[u8], level: i32) -> Vec<u8> {
    let mut out = Vec::new();
    COMPRESSOR.with(|c| {
        if let Ok(mut c) = c.try_borrow_mut() {
            c.set_compression_level(level);
            match c.compress(data) {
                Ok(res) => out = res,
                Err(err) => {
                    crate::log::debug!("Failed to compress: {}", err);
                }
            }
        }
    });
    out
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    DECOMPRESSOR.with(|d| {
        if let Ok(mut d) = d.try_borrow_mut() {
            const MAX: usize = 1024 * 1024 * 64;
            const MIN: usize = 1024 * 1024;
            let mut n = 30 * data.len();
            if n > MAX {
                n = MAX;
            }
            if n < MIN {
                n = MIN;
            }
            match d.decompress(data, n) {
                Ok(res) => out = res,
                Err(err) => {
                    crate::log::debug!("Failed to decompress: {}", err);
                }
            }
        }
    });
    out
}
