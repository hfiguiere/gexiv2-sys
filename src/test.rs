// Copyright © 2017 Felix A. Crux <felixc@felixcrux.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Basic tests for gexiv2.

extern crate libc;

use std::ffi;
use std::ptr;

use super::*;


static MINI_PNG: [u8; 67] = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0,
                             0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73,
                             68, 65, 84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0,
                             0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];

unsafe fn make_new_metadata() -> *mut GExiv2Metadata {
    let mut err: *mut GError = ptr::null_mut();
    let metadata = gexiv2_metadata_new();

    let ok = gexiv2_metadata_open_buf(
        metadata, MINI_PNG.as_ptr(), MINI_PNG.len() as libc::c_long, &mut err);
    if ok != 1 {
        match ffi::CStr::from_ptr((*err).message).to_str() {
            Ok(v) => panic!(v.to_string()),
            Err(_) => panic!("Unknown error")
        }
    }

    metadata
}


#[test]
fn initialize() {
    unsafe {
        assert_eq!(gexiv2_initialize(), 1);
    }
}

#[test]
fn get_version() {
    unsafe {
        assert!(gexiv2_get_version() > 0);
    }
}


// Image information.

#[test]
fn metadata_get_supports_exif() {
    unsafe {
        let meta = make_new_metadata();
        assert_eq!(gexiv2_metadata_get_supports_exif(meta), 1);
    }
}

#[test]
fn metadata_get_supports_iptc() {
    unsafe {
        let meta = make_new_metadata();
        assert_eq!(gexiv2_metadata_get_supports_iptc(meta), 1);
    }
}

#[test]
fn metadata_get_supports_xmp() {
    unsafe {
        let meta = make_new_metadata();
        assert_eq!(gexiv2_metadata_get_supports_xmp(meta), 1);
    }
}

#[test]
fn metadata_get_mime_type() {
    unsafe {
        let meta = make_new_metadata();
        let result = gexiv2_metadata_get_mime_type(meta);
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "image/png");
    }
}

#[test]
fn metadata_get_pixel_width() {
    unsafe {
        let meta = make_new_metadata();
        assert_eq!(gexiv2_metadata_get_pixel_width(meta), 1);
    }
}

#[test]
fn metadata_get_pixel_height() {
    unsafe {
        let meta = make_new_metadata();
        assert_eq!(gexiv2_metadata_get_pixel_height(meta), 1);
    }
}


// Tag information functions.

#[test]
fn metadata_is_exif_tag() {
    unsafe {
        let exif_tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let not_exif_tag = ffi::CString::new("Iptc.Application2.Keywords").unwrap();
        assert_eq!(gexiv2_metadata_is_exif_tag(exif_tag.as_ptr()), 1);
        assert_eq!(gexiv2_metadata_is_exif_tag(not_exif_tag.as_ptr()), 0);
    }
}

#[test]
fn metadata_is_iptc_tag() {
    unsafe {
        let iptc_tag = ffi::CString::new("Iptc.Application2.Keywords").unwrap();
        let not_iptc_tag = ffi::CString::new("Xmp.dc.Description").unwrap();
        assert_eq!(gexiv2_metadata_is_iptc_tag(iptc_tag.as_ptr()), 1);
        assert_eq!(gexiv2_metadata_is_iptc_tag(not_iptc_tag.as_ptr()), 0);
    }
}

#[test]
fn metadata_is_xmp_tag() {
    unsafe {
        let xmp_tag = ffi::CString::new("Xmp.dc.Description").unwrap();
        let not_xmp_tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        assert_eq!(gexiv2_metadata_is_xmp_tag(xmp_tag.as_ptr()), 1);
        assert_eq!(gexiv2_metadata_is_xmp_tag(not_xmp_tag.as_ptr()), 0);
    }
}

#[test]
fn metadata_get_tag_label() {
    unsafe {
        let tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let result = gexiv2_metadata_get_tag_label(tag.as_ptr());
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "Image Description");
    }
}

#[test]
fn metadata_get_tag_description() {
    unsafe {
        let tag = ffi::CString::new("Exif.Image.FillOrder").unwrap();
        let result = gexiv2_metadata_get_tag_description(tag.as_ptr());
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "The logical order of bits within a byte");
    }
}

#[test]
fn metadata_get_tag_type() {
    unsafe {
        let tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let result = gexiv2_metadata_get_tag_type(tag.as_ptr());
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "Ascii");
    }
}


// Logging

#[test]
fn log_get_and_set_level() {
    unsafe {
        assert_eq!(gexiv2_log_get_level(), GExiv2LogLevel::WARN);
        gexiv2_log_set_level(GExiv2LogLevel::INFO);
        assert_eq!(gexiv2_log_get_level(), GExiv2LogLevel::INFO);
    }
}
