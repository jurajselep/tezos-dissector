use libc::{c_char, c_int, c_uint, c_void};

/// Struct that represents static data on C side
#[repr(C)]
pub struct T3zosDissectorInfo {
    pub hf_payload_len: c_int,
    pub hf_packet_counter: c_int,
    pub hf_connection_msg: c_int,
    pub hf_decrypted_msg: c_int,
    pub hf_error: c_int,

    pub hf_debug: c_int,
}