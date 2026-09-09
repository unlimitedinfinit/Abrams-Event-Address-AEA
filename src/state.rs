//! AEA-STATE/1 encode + seal + verify.
//!
//! Explicit Little-Endian prefix serialization over offsets [0, 104).
//! Never hashes raw struct padding. Canonicalizes -0.0 to +0.0.

use crate::sha256::sha256_32;

pub const RECORD_BYTES: usize = 136;
pub const PREFIX_BYTES: usize = 104;
pub const HASH_BYTES: usize = 32;

pub const VERSION_WIRE: u32 = 1;

/* Coordinate Frame Constants */
pub const FRAME_INVALID: u16 = 0;
pub const FRAME_ITRF2020: u16 = 1;
pub const FRAME_GCRS: u16 = 2;
pub const FRAME_BCRS: u16 = 3;
pub const FRAME_BODY: u16 = 4;

/* Time Scale Constants */
pub const TIME_INVALID: u16 = 0;
pub const TIME_TAI: u16 = 1;
pub const TIME_TT: u16 = 2;
pub const TIME_TCB: u16 = 3;
pub const TIME_TDB: u16 = 4;

/* Return / Status Codes */
pub const AEA_OK: i32 = 0;
pub const AEA_ERR_MISMATCH: i32 = 1;
pub const AEA_ERR_NULL_PTR: i32 = -1;
pub const AEA_ERR_VERSION: i32 = -2;
pub const AEA_ERR_NSEC_BOUND: i32 = -3;
pub const AEA_ERR_FRAME: i32 = -4;
pub const AEA_ERR_TIME_SCALE: i32 = -5;
pub const AEA_ERR_NON_FINITE: i32 = -6;

/**
 * 136-byte Canonical AEA State Record.
 * Standard C layout (#[repr(C)]), naturally 8-aligned.
 */
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AeaRecord {
    pub version: u32,
    pub body_naif: u32,
    pub time_sec: i64,
    pub time_nsec: u32,
    pub frame: u16,
    pub time_scale: u16,
    pub pos_m: [f64; 3],
    pub vel_mps: [f64; 3],
    pub quat_xyzw: [f64; 4],
    pub hash: [u8; 32],
}

#[inline]
pub fn canon(x: f64) -> f64 {
    if x == 0.0 { 0.0 } else { x }
}

#[inline]
fn write_u16_le(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

#[inline]
fn write_u32_le(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

#[inline]
fn write_i64_le(buf: &mut [u8], off: usize, v: i64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

#[inline]
fn write_f64_le(buf: &mut [u8], off: usize, v: f64) {
    buf[off..off + 8].copy_from_slice(&canon(v).to_le_bytes());
}

/**
 * Serializes the 104-byte Little-Endian prefix [0, 104) into a byte buffer.
 * Performs explicit Little-Endian serialization; does not perform a raw memcpy.
 */
pub fn encode_prefix(r: &AeaRecord) -> [u8; PREFIX_BYTES] {
    let mut b = [0u8; PREFIX_BYTES];
    write_u32_le(&mut b, 0, r.version);
    write_u32_le(&mut b, 4, r.body_naif);
    write_i64_le(&mut b, 8, r.time_sec);
    write_u32_le(&mut b, 16, r.time_nsec);
    write_u16_le(&mut b, 20, r.frame);
    write_u16_le(&mut b, 22, r.time_scale);
    write_f64_le(&mut b, 24, r.pos_m[0]);
    write_f64_le(&mut b, 32, r.pos_m[1]);
    write_f64_le(&mut b, 40, r.pos_m[2]);
    write_f64_le(&mut b, 48, r.vel_mps[0]);
    write_f64_le(&mut b, 56, r.vel_mps[1]);
    write_f64_le(&mut b, 64, r.vel_mps[2]);
    write_f64_le(&mut b, 72, r.quat_xyzw[0]);
    write_f64_le(&mut b, 80, r.quat_xyzw[1]);
    write_f64_le(&mut b, 88, r.quat_xyzw[2]);
    write_f64_le(&mut b, 96, r.quat_xyzw[3]);
    b
}

/**
 * Validates the contents of an AeaRecord.
 * Returns AEA_OK (0) or negative error code on invalid data.
 */
pub fn validate(r: &AeaRecord) -> i32 {
    if r.version != VERSION_WIRE {
        return AEA_ERR_VERSION;
    }
    if r.time_nsec >= 1_000_000_000 {
        return AEA_ERR_NSEC_BOUND;
    }
    if r.frame < FRAME_ITRF2020 || r.frame > FRAME_BODY {
        return AEA_ERR_FRAME;
    }
    if r.time_scale < TIME_TAI || r.time_scale > TIME_TDB {
        return AEA_ERR_TIME_SCALE;
    }
    for &x in r.pos_m.iter().chain(r.vel_mps.iter()).chain(r.quat_xyzw.iter()) {
        if !x.is_finite() {
            return AEA_ERR_NON_FINITE;
        }
    }
    AEA_OK
}

/**
 * Validates, canonicalizes negative zeros, computes SHA-256 over prefix [0, 104),
 * and writes the 32-byte digest into record.hash.
 */
pub fn seal(record: &mut AeaRecord) -> i32 {
    let err = validate(record);
    if err != AEA_OK {
        return err;
    }
    for x in &mut record.pos_m {
        *x = canon(*x);
    }
    for x in &mut record.vel_mps {
        *x = canon(*x);
    }
    for x in &mut record.quat_xyzw {
        *x = canon(*x);
    }
    let prefix = encode_prefix(record);
    let digest = sha256_32(&prefix);
    record.hash.copy_from_slice(&digest);
    AEA_OK
}

/**
 * Validates and compares the recomputed digest over prefix [0, 104) with record.hash.
 * Comparison is executed in a fixed 32-iteration loop.
 */
pub fn verify(record: &AeaRecord) -> i32 {
    let err = validate(record);
    if err != AEA_OK {
        return err;
    }
    let prefix = encode_prefix(record);
    let digest = sha256_32(&prefix);
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= record.hash[i] ^ digest[i];
    }
    if diff == 0 {
        AEA_OK
    } else {
        AEA_ERR_MISMATCH
    }
}

pub fn zeroed() -> AeaRecord {
    AeaRecord {
        version: VERSION_WIRE,
        body_naif: 0,
        time_sec: 0,
        time_nsec: 0,
        frame: FRAME_ITRF2020,
        time_scale: TIME_TAI,
        pos_m: [0.0; 3],
        vel_mps: [0.0; 3],
        quat_xyzw: [0.0, 0.0, 0.0, 1.0],
        hash: [0u8; 32],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seal_and_verify_roundtrip() {
        let mut rec = zeroed();
        rec.body_naif = 399;
        rec.pos_m = [100.0, 200.0, 300.0];
        assert_eq!(seal(&mut rec), AEA_OK);
        assert_ne!(rec.hash, [0u8; 32]);
        assert_eq!(verify(&rec), AEA_OK);
    }

    #[test]
    fn test_negative_zero_canonicalization() {
        let mut rec1 = zeroed();
        rec1.pos_m = [0.0, 0.0, 0.0];
        assert_eq!(seal(&mut rec1), AEA_OK);

        let mut rec2 = zeroed();
        rec2.pos_m = [-0.0, 0.0, -0.0];
        assert_eq!(seal(&mut rec2), AEA_OK);

        assert_eq!(rec1.hash, rec2.hash);
        assert_eq!(rec2.pos_m[0].to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn test_validation_errors() {
        let mut rec = zeroed();
        rec.version = 2;
        assert_eq!(seal(&mut rec), AEA_ERR_VERSION);

        rec = zeroed();
        rec.time_nsec = 1_000_000_000;
        assert_eq!(seal(&mut rec), AEA_ERR_NSEC_BOUND);

        rec = zeroed();
        rec.frame = 0;
        assert_eq!(seal(&mut rec), AEA_ERR_FRAME);
        rec.frame = 5;
        assert_eq!(seal(&mut rec), AEA_ERR_FRAME);

        rec = zeroed();
        rec.time_scale = 0;
        assert_eq!(seal(&mut rec), AEA_ERR_TIME_SCALE);
        rec.time_scale = 5;
        assert_eq!(seal(&mut rec), AEA_ERR_TIME_SCALE);

        rec = zeroed();
        rec.pos_m[0] = f64::NAN;
        assert_eq!(seal(&mut rec), AEA_ERR_NON_FINITE);

        rec = zeroed();
        rec.vel_mps[1] = f64::INFINITY;
        assert_eq!(seal(&mut rec), AEA_ERR_NON_FINITE);

        rec = zeroed();
        rec.quat_xyzw[3] = f64::NEG_INFINITY;
        assert_eq!(seal(&mut rec), AEA_ERR_NON_FINITE);
    }

    #[test]
    fn test_tamper_detection() {
        let mut rec = zeroed();
        assert_eq!(seal(&mut rec), AEA_OK);

        // Tamper with position
        rec.pos_m[0] += 0.001;
        assert_eq!(verify(&rec), AEA_ERR_MISMATCH);

        // Restore position and tamper with hash
        rec.pos_m[0] -= 0.001;
        assert_eq!(verify(&rec), AEA_OK);
        rec.hash[0] ^= 0x01;
        assert_eq!(verify(&rec), AEA_ERR_MISMATCH);
    }
}
