#!/usr/bin/env python3
"""
Abrams Event Address (AEA-STATE/1) — Zero-Dependency Reference Tool & Decoder
Standard: 136-byte unpacked C struct, Little-Endian prefix [0, 104), SHA-256 trailer.
Python stdlib only: struct, hashlib, math, sys, json, argparse.
"""

import sys
import os
import struct
import hashlib
import math
import json
import argparse

RECORD_BYTES = 136
PREFIX_BYTES = 104
HASH_BYTES = 32

FRAME_NAMES = {
    0: "INVALID (0)",
    1: "ITRF2020 (Earth-fixed)",
    2: "GCRS (Geocentric Celestial)",
    3: "BCRS (Barycentric Celestial)",
    4: "BODY (Body-fixed)"
}

TIME_SCALE_NAMES = {
    0: "INVALID (0)",
    1: "TAI (International Atomic Time)",
    2: "TT (Terrestrial Time)",
    3: "TCB (Barycentric Coordinate Time)",
    4: "TDB (Barycentric Dynamical Time)"
}

def canon_f64(x: float) -> float:
    """Canonicalize -0.0 to +0.0 per AEA-STATE/1 specification."""
    return 0.0 if x == 0.0 else x

def parse_record(data: bytes) -> dict:
    """Parse 136 raw bytes into field dictionary."""
    if len(data) != RECORD_BYTES:
        raise ValueError(f"Expected {RECORD_BYTES} bytes, got {len(data)}")

    v, body, t_sec, t_nsec, frame, scale = struct.unpack_from("<IIqIHH", data, 0)
    pos = list(struct.unpack_from("<3d", data, 24))
    vel = list(struct.unpack_from("<3d", data, 48))
    quat = list(struct.unpack_from("<4d", data, 72))
    digest = data[104:136]

    return {
        "version": v,
        "body_naif": body,
        "time_sec": t_sec,
        "time_nsec": t_nsec,
        "frame": frame,
        "frame_name": FRAME_NAMES.get(frame, f"UNKNOWN ({frame})"),
        "time_scale": scale,
        "time_scale_name": TIME_SCALE_NAMES.get(scale, f"UNKNOWN ({scale})"),
        "pos_m": pos,
        "vel_mps": vel,
        "quat_xyzw": quat,
        "hash_hex": digest.hex()
    }

def encode_prefix(rec: dict) -> bytes:
    """Encode 104-byte Little-Endian prefix with -0.0 canonicalization."""
    buf = bytearray(PREFIX_BYTES)
    struct.pack_into("<IIqIHH", buf, 0,
                     rec["version"],
                     rec["body_naif"],
                     rec["time_sec"],
                     rec["time_nsec"],
                     rec["frame"],
                     rec["time_scale"])

    for i in range(3):
        struct.pack_into("<d", buf, 24 + i * 8, canon_f64(rec["pos_m"][i]))
    for i in range(3):
        struct.pack_into("<d", buf, 48 + i * 8, canon_f64(rec["vel_mps"][i]))
    for i in range(4):
        struct.pack_into("<d", buf, 72 + i * 8, canon_f64(rec["quat_xyzw"][i]))

    return bytes(buf)

def compute_hash(rec: dict) -> bytes:
    """Compute SHA-256 over 104-byte canonical prefix."""
    prefix = encode_prefix(rec)
    return hashlib.sha256(prefix).digest()

def seal_record(rec: dict) -> bytes:
    """Encode complete 136-byte binary record with computed SHA-256 trailer."""
    prefix = encode_prefix(rec)
    digest = hashlib.sha256(prefix).digest()
    return prefix + digest

def verify_record(data: bytes) -> tuple[bool, str, str]:
    """Verify SHA-256 trailer against canonical prefix. Returns (valid, expected_hex, stored_hex)."""
    rec = parse_record(data)
    expected = compute_hash(rec).hex()
    stored = rec["hash_hex"]
    return (expected == stored, expected, stored)

def ecef_to_wgs84(x: float, y: float, z: float) -> tuple[float, float, float]:
    """
    Convert Cartesian ECEF (x, y, z) in metres to WGS 84 (lat_deg, lon_deg, height_m)
    using Bowring's closed-form algorithm.
    Caption: WGS 84 ellipsoid inverse; tagged ITRF2020 for the test. Not an official IERS station coordinate.
    """
    a = 6378137.0
    f = 1.0 / 298.257223563
    b = a * (1.0 - f)
    e2 = (a*a - b*b) / (a*a)
    ep2 = (a*a - b*b) / (b*b)

    p = math.hypot(x, y)
    if p < 1e-6:
        lat = 90.0 if z > 0 else -90.0
        lon = 0.0
        h = abs(z) - b
        return (lat, lon, h)

    theta = math.atan2(z * a, p * b)
    lat = math.atan2(
        z + ep2 * b * math.sin(theta)**3,
        p - e2 * a * math.cos(theta)**3
    )
    lon = math.atan2(y, x)

    N = a / math.sqrt(1.0 - e2 * math.sin(lat)**2)
    h = p / math.cos(lat) - N

    return (math.degrees(lat), math.degrees(lon), h)

# Built-in Canonical Known-Answer Test Vector
CAMBRIDGE_KAT = {
    "version": 1,
    "body_naif": 399,
    "time_sec": 1246190426,  # TAI corresponding to UTC 2009-06-28 12:00:00 (TAI = UTC + 34s)
    "time_nsec": 0,
    "frame": 1,              # ITRF2020
    "time_scale": 1,         # TAI
    "pos_m": [3916909.5232774816, 8057.749278632174, 5016918.648210711],
    "vel_mps": [0.0, 0.0, 0.0],
    "quat_xyzw": [0.0, 0.0, 0.0, 1.0],
    "expected_hash": "9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e"
}

def cmd_decode(args):
    with open(args.file, "rb") as f:
        data = f.read()
    if len(data) != RECORD_BYTES:
        sys.stderr.write(f"Error: File is {len(data)} bytes; expected exactly {RECORD_BYTES} bytes.\n")
        sys.exit(1)

    rec = parse_record(data)
    valid, expected_hex, stored_hex = verify_record(data)

    status_tag = "[SEAL VERIFIED]" if valid else "[SEAL CORRUPTED / MISMATCH]"
    print(f"{status_tag} (SHA-256: {stored_hex})")
    print(f"  Wire Version: {rec['version']}")
    print(f"  Body NAIF:    {rec['body_naif']} ({'Earth' if rec['body_naif'] == 399 else 'Other'})")
    print(f"  Timestamp:    {rec['time_sec']}.{rec['time_nsec']:09d} ({rec['time_scale_name']})")
    print(f"  Frame:        {rec['frame_name']}")
    print(f"  Position (m): [{rec['pos_m'][0]:.4f}, {rec['pos_m'][1]:.4f}, {rec['pos_m'][2]:.4f}]")
    print(f"  Velocity m/s: [{rec['vel_mps'][0]:.4f}, {rec['vel_mps'][1]:.4f}, {rec['vel_mps'][2]:.4f}]")
    print(f"  Attitude Q:   [{rec['quat_xyzw'][0]:.4f}, {rec['quat_xyzw'][1]:.4f}, {rec['quat_xyzw'][2]:.4f}, {rec['quat_xyzw'][3]:.4f}]")

    if rec['frame'] == 1 and rec['body_naif'] == 399:
        lat, lon, h = ecef_to_wgs84(rec['pos_m'][0], rec['pos_m'][1], rec['pos_m'][2])
        print(f"  Geodetic (WGS84 Ellipsoid Inverse, tagged ITRF2020):")
        print(f"    Lat: {lat:.7f} deg, Lon: {lon:.7f} deg, Height: {h:.2f} m")
        print(f"    Map: https://www.openstreetmap.org/?mlat={lat:.6f}&mlon={lon:.6f}#map=16/{lat:.6f}/{lon:.6f}")

    if not valid:
        print(f"  WARNING: Expected hash: {expected_hex}")
        sys.exit(1)

def cmd_verify(args):
    with open(args.file, "rb") as f:
        data = f.read()
    valid, expected_hex, stored_hex = verify_record(data)
    if valid:
        print(f"OK: SHA-256 integrity seal verified ({stored_hex}).")
        sys.exit(0)
    else:
        sys.stderr.write(f"FAIL: Integrity seal mismatch (stored: {stored_hex}, expected: {expected_hex}).\n")
        sys.exit(1)

def cmd_json(args):
    with open(args.file, "rb") as f:
        data = f.read()
    rec = parse_record(data)
    valid, _, _ = verify_record(data)
    rec["seal_valid"] = valid
    print(json.dumps(rec, indent=2))

def cmd_kats(args):
    out_dir = args.out_dir
    os.makedirs(out_dir, exist_ok=True)

    c_bytes = seal_record(CAMBRIDGE_KAT)
    c_hash = c_bytes[104:].hex()
    assert c_hash == CAMBRIDGE_KAT["expected_hash"], f"Cambridge hash mismatch: {c_hash}"
    c_path = os.path.join(out_dir, "cambridge.bin")
    with open(c_path, "wb") as f:
        f.write(c_bytes)
    print(f"Generated {c_path} ({len(c_bytes)} bytes) -> SHA-256: {c_hash} [MATCH]")

def main():
    parser = argparse.ArgumentParser(description="Abrams Event Address (AEA-STATE/1) Reference Tool")
    sub = parser.add_subparsers(dest="command", required=True)

    dec_p = sub.add_parser("decode", help="Decode and inspect a 136-byte AEA binary record")
    dec_p.add_argument("file", help="Path to 136-byte .bin file")
    dec_p.set_defaults(func=cmd_decode)

    ver_p = sub.add_parser("verify", help="Verify SHA-256 integrity seal of an AEA binary record")
    ver_p.add_argument("file", help="Path to 136-byte .bin file")
    ver_p.set_defaults(func=cmd_verify)

    js_p = sub.add_parser("json", help="Export AEA binary record as JSON")
    js_p.add_argument("file", help="Path to 136-byte .bin file")
    js_p.set_defaults(func=cmd_json)

    kat_p = sub.add_parser("kats", help="Generate and verify official Known-Answer Test binary files")
    kat_p.add_argument("--out-dir", default=".", help="Directory to emit cambridge.bin")
    kat_p.set_defaults(func=cmd_kats)

    args = parser.parse_args()
    args.func(args)

if __name__ == "__main__":
    main()
