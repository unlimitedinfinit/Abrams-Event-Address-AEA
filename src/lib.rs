//! aea-core — Zero-heap, #![no_std] Abrams Event Address (AEA-STATE/1) sealer.
//!
//! Zero crate dependencies. Pure C ABI:
//! - aea_seal
//! - aea_verify
//! - aea_version

#![cfg_attr(all(not(test), target_os = "none"), no_std)]
#![deny(warnings)]

mod sha256;
pub mod state;

pub use state::{
    canon, encode_prefix, seal, validate, verify, zeroed, AeaRecord,
    AEA_ERR_FRAME, AEA_ERR_MISMATCH, AEA_ERR_NON_FINITE, AEA_ERR_NSEC_BOUND,
    AEA_ERR_NULL_PTR, AEA_ERR_TIME_SCALE, AEA_ERR_VERSION, AEA_OK,
    FRAME_BCRS, FRAME_BODY, FRAME_GCRS, FRAME_INVALID, FRAME_ITRF2020,
    HASH_BYTES, PREFIX_BYTES, RECORD_BYTES, TIME_INVALID, TIME_TAI,
    TIME_TCB, TIME_TDB, TIME_TT, VERSION_WIRE,
};

#[cfg(all(not(test), target_os = "none"))]
use core::panic::PanicInfo;

#[cfg(all(not(test), target_os = "none"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/**
 * Validates record fields, encodes the 104-byte Little-Endian prefix,
 * computes the SHA-256 digest, and writes the 32-byte digest into record->hash.
 *
 * @param record Pointer to mutable AeaRecord.
 * @return AEA_OK (0) on success, or negative error code on invalid data.
 */
#[no_mangle]
pub unsafe extern "C" fn aea_seal(record: *mut AeaRecord) -> i32 {
    if record.is_null() {
        return AEA_ERR_NULL_PTR;
    }
    state::seal(&mut *record)
}

/**
 * Validates record fields, recomputes the SHA-256 digest over the 104-byte prefix,
 * and compares it with record->hash in a fixed 32-iteration loop.
 *
 * @param record Pointer to const AeaRecord.
 * @return AEA_OK (0) on match, AEA_ERR_MISMATCH (1) on mismatch, or negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn aea_verify(record: *const AeaRecord) -> i32 {
    if record.is_null() {
        return AEA_ERR_NULL_PTR;
    }
    state::verify(&*record)
}

/**
 * Returns library ABI version as ((MAJOR << 16) | MINOR).
 * For v1.0, returns 0x00010000 (65536).
 */
#[no_mangle]
pub extern "C" fn aea_version() -> u32 {
    (1u32 << 16) | 0u32
}
