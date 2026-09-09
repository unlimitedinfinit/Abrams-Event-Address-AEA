# Interface Control Document (ICD): AEA-STATE/1

## 1. Scope & Overview
This document specifies the wire layout, binary encoding, enumeration tables, and error codes for the **Abrams Event Address** state record (`AEA-STATE/1`). 

The record is a fixed-size, 136-byte, naturally aligned, Little-Endian data structure designed for real-time telemetry freeze and denied-link state recovery on autonomous aerospace, maritime, and defense platforms.

---

## 2. Binary Wire Layout

The `AeaRecord` structure occupies exactly 136 bytes in memory. The field ordering aligns every 64-bit float and integer on an 8-byte boundary, preventing unaligned memory access penalties without packing directives.

| Offset | Size (B) | Type | Field Name | Description |
|:---:|:---:|:---|---|---|
| `0` | 4 | `uint32_t` | `version` | Wire format version. Must be `1`. |
| `4` | 4 | `uint32_t` | `body_naif` | NAIF Body ID (`399` = Earth geocenter, `499` = Mars, `0` = none / no body). |
| `8` | 8 | `int64_t` | `time_sec` | Integer seconds elapsed since 1970-01-01T00:00:00 on the declared `time_scale`. |
| `16` | 4 | `uint32_t` | `time_nsec` | Integer nanoseconds ($0 \le \text{nsec} \le 999,999,999$). |
| `20` | 2 | `uint16_t` | `frame` | Coordinate reference chart enum (`enum AeaFrame`). |
| `22` | 2 | `uint16_t` | `time_scale` | Coordinate time scale enum (`enum AeaTimeScale`). |
| `24` | 24 | `double[3]` | `pos_m` | Cartesian position $[X, Y, Z]$ in metres in the declared `frame`. |
| `48` | 24 | `double[3]` | `vel_mps` | Cartesian velocity $[V_x, V_y, V_z]$ in metres/second in the declared `frame`. |
| `72` | 32 | `double[4]` | `quat_xyzw` | Unit attitude quaternion $[q_x, q_y, q_z, q_w]$ from frame to body. Null: `[0,0,0,1]`. |
| `104` | 32 | `uint8_t[32]`| `hash` | SHA-256 digest computed over the 104-byte prefix (offsets `[0, 104)`). |

**Total Size:** Exactly 136 bytes.  
**Alignment:** 8 bytes.

### 2.1 Critical Serialization & Invariant Rules
1. **Canonical Little-Endian Encoding:** The 32-byte `hash` trailer is the SHA-256 digest of the **canonical Little-Endian encoding** of fields 0 through 103 (offsets `[0, 104)`). It is strictly **not** a raw `memcpy` of the in-memory struct, preventing compiler padding and host endianness divergence from corrupting the wire digest.
2. **Negative-Zero Canonicalization:** During prefix encoding, IEEE-754 negative zeros (`-0.0`) are canonicalized to `+0.0` for hash computation. `aea_verify` canonicalizes prefix bytes during verification; an in-memory struct holding `-0.0` will verify against a sealed `+0.0` hash without mutating caller memory.
3. **Quaternion Normalization & Null Identity Collision:** Attitude quaternion `quat_xyzw` is stored as four 64-bit `double`s. **Unit length ($\|\mathbf{q}\| = 1.0$) is NOT enforced or normalized by `libaea.a`**. Identity quaternion `[0, 0, 0, 1]` represents either an unmeasured/null attitude or an attitude aligned with chart axes (a known v1 collision).
4. **Terrestrial Coordinates & Epoch Drift:** `pos_m` in `ITRF2020` is valid strictly at the **event epoch** (`time_sec` / `time_nsec`). Because continental tectonic plates shift over time (e.g. Eurasia moves $\sim 2.21\text{ cm/year}$, or $68.5\text{ cm}$ over 31 years), comparing coordinates across multi-year baselines requires the consumer to apply an external Plate Motion Model (PMM).
5. **KAT Geodetic Provenance:** Known-Answer Test (KAT) ECEF coordinates (such as the Cambridge vector) are derived directly from the **WGS 84 reference ellipsoid**, tagged `ITRF2020` for test purposes. They are deterministic mathematical test vectors, not official IERS station coordinates.

---

## 3. Enumeration Tables

### 3.1 Coordinate Reference Frame (`enum AeaFrame`)
Value `0` is explicitly reserved as `INVALID`.

| Enum Value | Identifier | Physical Definition |
|:---:|---|---|
| `0` | `AEA_FRAME_INVALID` | Invalid frame (rejected by validation). |
| `1` | `AEA_FRAME_ITRF2020` | International Terrestrial Reference Frame 2020 (Earth-fixed Cartesian metres). |
| `2` | `AEA_FRAME_GCRS` | Geocentric Celestial Reference System (IAU, Earth center of mass, non-rotating). |
| `3` | `AEA_FRAME_BCRS` | Barycentric Celestial Reference System (IAU, Solar System Barycenter). |
| `4` | `AEA_FRAME_BODY` | Body-fixed Cartesian frame centered on `body_naif`. |

*Axes note:* GCRS and BCRS spatial axes are defined by the ICRS, realized by ICRF3 quasar positions.

### 3.2 Coordinate Time Scale (`enum AeaTimeScale`)
Value `0` is explicitly reserved as `INVALID`.

| Enum Value | Identifier | Physical Definition |
|:---:|---|---|
| `0` | `AEA_TIME_INVALID` | Invalid time scale (rejected by validation). |
| `1` | `AEA_TIME_TAI` | International Atomic Time (uniform SI seconds). |
| `2` | `AEA_TIME_TT` | Terrestrial Time ($\text{TT} = \text{TAI} + 32.184\text{ s}$). |
| `3` | `AEA_TIME_TCB` | Barycentric Coordinate Time (coordinate time of BCRS). |
| `4` | `AEA_TIME_TDB` | Barycentric Dynamical Time (rescaled TCB). |

---

## 4. API Return Codes

All API functions return signed 32-bit integers (`int32_t`):

| Code | Identifier | Meaning |
|:---:|---|---|
| `0` | `AEA_OK` | Operation succeeded, or verification matched. |
| `1` | `AEA_ERR_MISMATCH` | Verification failed: computed SHA-256 does not match `record->hash`. |
| `-1` | `AEA_ERR_NULL_PTR` | Null pointer passed to API. |
| `-2` | `AEA_ERR_VERSION` | Unsupported wire version (`version != 1`). |
| `-3` | `AEA_ERR_NSEC_BOUND` | Nanosecond value out of bounds (`time_nsec >= 1,000,000,000`). |
| `-4` | `AEA_ERR_FRAME` | Unrecognized coordinate frame enum ($< 1$ or $> 4$). |
| `-5` | `AEA_ERR_TIME_SCALE` | Unrecognized time scale enum ($< 1$ or $> 4$). |
| `-6` | `AEA_ERR_NON_FINITE` | Non-finite float (`NaN`, $+\infty$, $-\infty$) in position, velocity, or quaternion. |

---

## 5. Canonical Published String Format

For logs, human inspection, and text interchange, the binary record maps to a standard single-line string:
```text
AEA:<body>:<scale>:<sec>.<nsec>:<frame>:<X>,<Y>,<Z>:<Vx>,<Vy>,<Vz>:<qx>,<qy>,<qz>,<qw>:<sha256hex>
```
*Example (Cambridge Vector):*
```text
AEA:399:TAI:1246190426.0:ITRF2020:3916909.5232774816,8057.749278632174,5016918.648210711:0,0,0:0,0,0,1:9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e
```
*(Note: String formatting routines are not compiled into `libaea.a` to maintain zero-heap constraints).*

---

## 6. Integration Caveats

### 6.1 Panic Handler Collision
`libaea.a` provides a `#![no_std]` panic handler (`loop {}`) so that C-only toolchains can link the static archive directly without missing symbol errors.
> [!WARNING]
> **Panic Symbol Collision:** Do not link a second Rust static library that also defines a panic handler, as the linker will report duplicate `panic_impl` symbols.

### 6.2 Cryptographic Integrity vs. Provenance
The embedded SHA-256 hash guarantees data integrity against memory corruption and transmission bit-rot. It is an unkeyed digest, not an authenticated digital signature. Provenance authentication (e.g. Ed25519) should be applied as an outer container wrapper around the 136-byte record.
