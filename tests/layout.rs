//! Layout assertions for AeaRecord.
//!
//! Enforces:
//! - Total struct size == 136 bytes
//! - Struct alignment == 8 bytes
//! - Exact byte offsets matching include/aea.h and docs/ICD.md

use aea::AeaRecord;
use core::mem::{align_of, offset_of, size_of, size_of_val};

#[test]
fn test_record_size_and_alignment() {
    assert_eq!(size_of::<AeaRecord>(), 136, "AeaRecord size must be exactly 136 bytes");
    assert_eq!(align_of::<AeaRecord>(), 8, "AeaRecord alignment must be 8 bytes");
}

#[test]
fn test_field_offsets() {
    assert_eq!(offset_of!(AeaRecord, version), 0, "offset of version must be 0");
    assert_eq!(offset_of!(AeaRecord, body_naif), 4, "offset of body_naif must be 4");
    assert_eq!(offset_of!(AeaRecord, time_sec), 8, "offset of time_sec must be 8");
    assert_eq!(offset_of!(AeaRecord, time_nsec), 16, "offset of time_nsec must be 16");
    assert_eq!(offset_of!(AeaRecord, frame), 20, "offset of frame must be 20");
    assert_eq!(offset_of!(AeaRecord, time_scale), 22, "offset of time_scale must be 22");
    assert_eq!(offset_of!(AeaRecord, pos_m), 24, "offset of pos_m must be 24");
    assert_eq!(offset_of!(AeaRecord, vel_mps), 48, "offset of vel_mps must be 48");
    assert_eq!(offset_of!(AeaRecord, quat_xyzw), 72, "offset of quat_xyzw must be 72");
    assert_eq!(offset_of!(AeaRecord, hash), 104, "offset of hash must be 104");
}

#[test]
fn test_field_sizes() {
    let dummy = aea::zeroed();
    assert_eq!(size_of_val(&dummy.version), 4);
    assert_eq!(size_of_val(&dummy.body_naif), 4);
    assert_eq!(size_of_val(&dummy.time_sec), 8);
    assert_eq!(size_of_val(&dummy.time_nsec), 4);
    assert_eq!(size_of_val(&dummy.frame), 2);
    assert_eq!(size_of_val(&dummy.time_scale), 2);
    assert_eq!(size_of_val(&dummy.pos_m), 24);
    assert_eq!(size_of_val(&dummy.vel_mps), 24);
    assert_eq!(size_of_val(&dummy.quat_xyzw), 32);
    assert_eq!(size_of_val(&dummy.hash), 32);
}
