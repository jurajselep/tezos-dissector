// This module integrates Wireshark C API to Rust.
// It integrares only small part of the API that is required by Tezos dissector.

// Auto-generated by bindgen from wireshark/epan/packet.h
pub mod packet;
use crate::wireshark::packet::proto_item;
use crate::wireshark::packet::wmem_allocator_t;

// Contains functions from wireshark that are not imported automatically
mod ffi;
pub use ffi::{proto_tree, tcp_analysis, tvbuff_t};

use failure::Error;
use libc::{c_char, c_int, c_uint};
use std::convert::{TryFrom, TryInto};
use std::net::IpAddr;
use std::slice::from_raw_parts;

mod error;
use error::{CannotReadIPv4BytesError, CannotReadIPv6BytesError, UnexpectedAddressTypeError};

/// Convert Wireshark IP address into Rust IpAddr
impl TryFrom<packet::address> for IpAddr {
    type Error = failure::Error;

    fn try_from(addr: packet::address) -> Result<Self, Self::Error> {
        let to_ip4 = || {
            let slice = unsafe { std::slice::from_raw_parts(addr.data as *const u8, 4) };
            let arr: [u8; 4] = slice.try_into().or(Err(CannotReadIPv6BytesError))?;
            Ok(IpAddr::from(arr))
        };
        let to_ip6 = || {
            let slice = unsafe { std::slice::from_raw_parts(addr.data as *const u8, 16) };
            let arr: [u8; 16] = slice.try_into().or(Err(CannotReadIPv6BytesError))?;
            Ok(IpAddr::from(arr))
        };
        match addr.type_ {
            address_type_AT_IPv4 => to_ip4(),
            address_type_AT_IPv6 => to_ip6(),
            _ => Err(UnexpectedAddressTypeError)?,
        }
    }
}

/// Return one byte from packet buffer.
/// `tvb` means packet buffer.
/// See wireshark/epan/tvbuff.h.
pub(crate) fn tvb_get_guint8(tvb: *mut tvbuff_t, offset: c_int /* gint */) -> u8 {
    unsafe { ffi::tvb_get_guint8(tvb, offset) }
}

/// Return amount of captured data in the buffer from packet buffer.
/// `tvb` means packet buffer.
/// See wireshark/epan/tvbuff.h.
pub(crate) fn tvb_captured_length(tvb: *mut tvbuff_t) -> c_uint {
    unsafe { ffi::tvb_captured_length(tvb) }
}

/// Return computed number of bytes to end of buffer.
/// `tvb` means packet buffer.
/// See wireshark/epan/tvbuff.h.
pub(crate) fn tvb_captured_length_remaining(tvb: *mut tvbuff_t) -> c_uint {
    unsafe { ffi::tvb_captured_length_remaining(tvb) }
}

/// Add i64 to proto_tree.
/// `proto_tree` means tree-like structure that visualizes parts of dissected packets.
/// See wireshark/epan/proto.h.
pub(crate) fn proto_tree_add_int64(
    proto_tree: *mut proto_tree,
    hfindex: c_int,
    tvb: *mut tvbuff_t,
    start: c_int,
    length: c_int,
    value: i64,
) -> *mut proto_item {
    unsafe { ffi::proto_tree_add_int64(proto_tree, hfindex, tvb, start, length, value) }
}

/// Add an item to a proto_tree, using the text label registered to that item..
/// `proto_tree` means tree-like structure that visualizes parts of dissected packets.
/// `hfindex` is id of the item (See crate::wireshark::dissector_info.rs)
/// See wireshark/epan/proto.h.
pub(crate) fn proto_tree_add_item(
    proto_tree: *mut proto_tree,
    hfindex: c_int,
    tvb: *mut tvbuff_t,
    start: c_int,
    length: c_int,
    encoding: c_uint,
) {
    unsafe {
        let mut str: *const u8 = std::ptr::null_mut();
        let mut len: c_uint = 0;

        ffi::proto_tree_add_item_ret_string_and_length(
            proto_tree,
            hfindex,
            tvb,
            start,
            length,
            encoding,
            ffi::wmem_packet_scope(),
            &mut str,
            &mut len,
        );
    }
}

/// Add a string to a proto_tree.
/// `proto_tree` means tree-like structure that visualizes parts of dissected packets.
/// `hfindex` is id of the item (See crate::wireshark::dissector_info.rs)
/// See wireshark/epan/proto.h.
pub(crate) fn proto_tree_add_string(
    proto_tree: *mut proto_tree,
    hfindex: c_int,
    tvb: *mut tvbuff_t,
    start: c_int,
    length: c_int,
    value: String,
) {
    unsafe {
        let bytes_num = value.len();
        let b = value.as_bytes();

        ffi::proto_tree_add_string_format_value(
            proto_tree,
            hfindex,
            tvb,
            start,
            length,
            b.as_ptr() as *const c_char,
            b"%.*s\0".as_ptr() as *const c_char,
            bytes_num as c_int,
            b.as_ptr() as *const c_char,
        );
    }
}

/// Get raw bytes from Wireshark buffer.
pub(crate) fn get_data<'a>(tvb: *mut tvbuff_t) -> &'a [u8] {
    unsafe {
        let ptr = ffi::tvb_get_ptr(tvb, 0, -1);
        let ulen = ffi::tvb_captured_length_remaining(tvb);
        // According to Wireshark documentation:
        //   https://www.wireshark.org/docs/wsar_html/group__tvbuff.html#ga31ba5c32b147f1f1e57dc8326e6fdc21
        // `get_raw_ptr()` should not be used, but it looks as easiest solution here.
        std::slice::from_raw_parts(ptr, ulen as usize)
    }
}