//! Example of how to access metadata from a byte buffer.
//!
//! To run it, try:
//!   $ cargo run --example open_buf

extern crate gexiv2_sys as gexiv2;
extern crate libc;

use std::ffi;
use std::ptr;

static MINI_PNG: [u8; 67] = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0,
                             0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73,
                             68, 65, 84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0,
                             0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];

pub unsafe fn make_new_metadata() -> *mut gexiv2::GExiv2Metadata {
    let mut err: *mut gexiv2::GError = ptr::null_mut();
    let metadata = gexiv2::gexiv2_metadata_new();

    let ok = gexiv2::gexiv2_metadata_open_buf(
        metadata, MINI_PNG.as_ptr(), MINI_PNG.len() as libc::c_long, &mut err);
    if ok != 1 {
        match ffi::CStr::from_ptr((*err).message).to_str() {
            Ok(v) => panic!(v.to_string()),
            Err(_) => panic!("Unknown error")
        }
    }

    metadata
}

#[allow(dead_code)]
fn main() {
    unsafe {
        let metadata = make_new_metadata();

        let media_type = gexiv2::gexiv2_metadata_get_mime_type(metadata);
        println!("{}", ffi::CStr::from_ptr(media_type).to_str().unwrap());

        gexiv2::gexiv2_metadata_free(metadata);
    }
}
