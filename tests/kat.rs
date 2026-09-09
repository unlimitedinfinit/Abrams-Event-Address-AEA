//! Known-Answer Test (KAT) Runner for AEA-STATE/1.
//!
//! Stephen Hawking's Cambridge reception (2009).
//! Input: WGS 84 ellipsoid ECEF tagged ITRF2020; not an official IERS station coordinate.
//! Zero crate dependencies.

use aea::{aea_seal, aea_verify, AeaRecord, AEA_ERR_MISMATCH, AEA_OK};

fn to_hex(bytes: &[u8; 32]) -> [u8; 64] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut hex = [0u8; 64];
    for i in 0..32 {
        hex[i * 2] = HEX[(bytes[i] >> 4) as usize];
        hex[i * 2 + 1] = HEX[(bytes[i] & 0x0f) as usize];
    }
    hex
}

#[test]
fn test_kat_cambridge() {
    let mut rec = AeaRecord {
        version: 1,
        body_naif: 399,
        time_sec: 1246190426,
        time_nsec: 0,
        frame: 1,
        time_scale: 1,
        pos_m: [3916909.5232774816, 8057.749278632174, 5016918.648210711],
        vel_mps: [0.0, 0.0, 0.0],
        quat_xyzw: [0.0, 0.0, 0.0, 1.0],
        hash: [0u8; 32],
    };

    let rc = unsafe { aea_seal(&mut rec) };
    assert_eq!(rc, AEA_OK, "aea_seal must return AEA_OK (0)");

    let hex = to_hex(&rec.hash);
    assert_eq!(
        &hex[..],
        b"9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e",
        "Cambridge KAT hash mismatch against committed vector"
    );

    let v_rc = unsafe { aea_verify(&rec) };
    assert_eq!(v_rc, AEA_OK, "aea_verify must return AEA_OK (0) on sealed record");

    // Tamper test: Flip one bit in pos_m[0]
    rec.pos_m[0] += 0.0001;
    let t_rc = unsafe { aea_verify(&rec) };
    assert_eq!(t_rc, AEA_ERR_MISMATCH, "aea_verify must return AEA_ERR_MISMATCH (1) on tampered record");
}
