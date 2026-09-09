# Abrams Event Address (AEA)

[![Standard: AEA-STATE/1](https://img.shields.io/badge/Wire%20Standard-AEA--STATE%2F1%20(136%20B)-0284c7.svg)](#2-the-wire-reality-aea-state1-136-bytes)
[![C-ABI](https://img.shields.io/badge/C--ABI-Naturally%20Aligned%20(136%20B)-10b981.svg)](include/aea.h)
[![Rust: no_std](https://img.shields.io/badge/Rust-%23!%5Bno__std%5D-orange.svg)](src/lib.rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-blue.svg)](LICENSE)
[![Verification: SHA-256 KAT](https://img.shields.io/badge/KAT%20Seal-Passed%20(9f6cc109...)-success.svg)](tests/kat/cambridge_v1.json)

**A deterministic 4D spacetime event addressing protocol and zero-heap `#![no_std]` integrity sealer for autonomous systems, flight blackboxes, and cross-domain state continuity.**

---

The **Abrams Event Address (AEA)** is a deterministic 4D spacetime coordinate standard and a 136-byte integrity-checked record for physical events. Rooted in relativistic astrometry and cryptographic verification, AEA replaces ambiguous, isolated conventions with explicit 4D event indexing across terrestrial, cislunar, and interplanetary frames.

While conventional navigation outputs locate an object only on a local chart at a single civil second, physical events exist in 4D spacetime. AEA-STATE/1 binds the governing physics directly to the telemetry: declaring the central gravitating body (NAIF ID), the astrometric reference chart (ITRF2020, GCRS, BCRS), continuous atomic time (TAI, TT, TDB), Cartesian kinematics, attitude quaternion, and a 32-byte SHA-256 integrity digest over the canonical prefix.

In communication-denied multi-agent operations or multi-decade archival logs, two platforms often hold state numbers that cannot be correlated because implicit frame or clock conventions were lost. AEA-STATE/1 binds the tags directly to the data so a consumer never has to guess the chart or time scale. It does not replace GPS, ITRF, GCRS, ROS 2 odometry, or CCSDS OEM; it is the zero-heap, single-epoch binary companion they emit. It does not navigate or propagate trajectories; it ensures that once state is estimated, where and when that event occurred remains mathematically unambiguous, verifiable, and permanent.

---

### The Canonical Event Address

```text
AEA:399:TAI:1246190426.0:ITRF2020:3916909.5232774816,8057.749278632174,5016918.648210711:0,0,0:0,0,0,1:9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e
```

> **Stephen Hawking Time-Traveler Reception** · Cambridge, UK · `2009-06-28 12:00 UTC`  
> Input: WGS 84 ellipsoid ECEF, tagged `ITRF2020` for test vector. Not an official IERS station coordinate.  
> 
> **Verify in 5 seconds (Python standard library only):**
> ```bash
> python tools/aea.py verify tests/kat/cambridge.bin
> # OK: SHA-256 integrity seal verified (9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e).
> ```

---

## 1. Spacetime Addressing: Four Foundational Principles

1. **Explicit Coordinate Charts (No Frame Ambiguity):** A Cartesian coordinate $(X, Y, Z)$ without a reference chart tag cannot distinguish between an Earth-fixed rotating crust (ITRF2020), an inertial geocentric sphere (GCRS), or a solar system barycentric origin (BCRS). AEA requires declaring the reference chart.
2. **Monotonic Atomic Time (No Leap-Second Arithmetic):** Civil time scales (UTC) insert discontinuous leap seconds, introducing arithmetic errors across epoch differences. High-integrity mission kinematics require continuous atomic time (TAI) or coordinate time (TT, TDB) measured from 1970-01-01T00:00:00 on the declared time scale.
3. **Event-Epoch Geodesy (Plate Motion):** Terrestrial coordinates in ITRF are valid strictly at the event epoch. Comparing coordinates across multi-year baselines requires the caller to apply a Plate Motion Model (PMM).
4. **Cryptographic Integrity Sealing:** Flight logs stored in non-volatile memory are vulnerable to single-event upsets (SEUs) and bit-flips from cosmic radiation. The SHA-256 trailer computed across the canonical Little-Endian prefix immediately detects bit-rot, memory corruption, or packet truncation without requiring a network connection or external PKI.

---

## 2. The Wire Reality: AEA-STATE/1 (136 Bytes)

To serve flight computers, microcontrollers, and radiation-hardened space hardware, the 4D address is implemented as an **unpacked, naturally aligned 136-byte C-ABI structure**:

```
+----------------------------------------------------------------------------+
|                       AEA-STATE/1 WIRE LAYOUT (136 BYTES)                  |
+---------------------------------------------------+------------------------+
| Bytes 0..103: Canonical Telemetry Prefix (104 B)  | Bytes 104..135: Seal   |
| Version, Body ID, TAI Epoch, Frame, Pos, Vel, Quat| SHA-256 Prefix Digest  |
+---------------------------------------------------+------------------------+
```

### Architectural Properties
* **Zero Heap (`#![no_std]`):** No `alloc`, no `malloc`, no heap allocations, no dynamic resizing. Safe for bare-metal bootloaders and flight loops.
* **Natural 8-Byte Alignment:** Natural alignment prevents hardware bus faults on strict RISC processors (SPARC LEON, ARM Cortex-M) without packing directives (`repr(C)`, no `#pragma pack`).
* **Deterministic Little-Endian Serialization:** Canonical serialization guarantees byte-for-byte identical SHA-256 digests across little-endian and big-endian platforms.
* **Negative-Zero Canonicalization:** IEEE-754 `-0.0` is canonicalized to `+0.0` during hashing to eliminate floating-point sign ambiguity.
* **Zero Dependencies:** Pure Rust core with a vendored, zero-dependency SHA-256 implementation. Zero external crates.

---

## 3. Wire Layout Specification

The `AeaRecord` structure occupies exactly 136 bytes in memory (offsets `[0, 104)` prefix + offsets `[104, 136)` trailer):

| Offset | Size (B) | Type | Field | Physical Definition |
|:---:|:---:|:---|---|---|
| `0` | 4 | `uint32_t` | `version` | Wire format version (`1` for `AEA-STATE/1`). |
| `4` | 4 | `uint32_t` | `body_naif` | NAIF body identifier (`399` = Earth geocenter, `499` = Mars, `0` = none). |
| `8` | 8 | `int64_t` | `time_sec` | Continuous integer seconds elapsed since 1970-01-01T00:00:00 on the declared `time_scale`. |
| `16` | 4 | `uint32_t` | `time_nsec` | Fractional nanoseconds ($0 \le \text{nsec} \le 999,999,999$). |
| `20` | 2 | `uint16_t` | `frame` | Reference frame enum (`1` = ITRF2020, `2` = GCRS, `3` = BCRS, `4` = BODY). |
| `22` | 2 | `uint16_t` | `time_scale` | Time scale enum (`1` = TAI atomic time, `2` = TT, `3` = TCB, `4` = TDB). |
| `24` | 24 | `double[3]` | `pos_m` | Cartesian position $[X, Y, Z]$ in metres in the declared frame. |
| `48` | 24 | `double[3]` | `vel_mps` | Cartesian velocity $[V_x, V_y, V_z]$ in metres/second in the declared frame. |
| `72` | 32 | `double[4]` | `quat_xyzw`| Unit attitude quaternion $[q_x, q_y, q_z, q_w]$. Identity null: `[0,0,0,1]`. |
| `104` | 32 | `uint8_t[32]`| `hash` | SHA-256 digest computed strictly over the 104-byte prefix. |

---

## 4. Quick-Start & 5-Second Verification

Clone this repository and verify the canonical Cambridge test vector in 5 seconds with zero external dependencies:

### Option A: Python Reference CLI (Zero Dependencies)
Requires only standard Python 3.8+:
```bash
# 1. Verify the canonical Cambridge 2009 Known-Answer Test
python tools/aea.py verify tests/kat/cambridge.bin
# Output: OK: SHA-256 integrity seal verified (9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e).

# 2. Decode the record into human-readable fields and WGS 84 coordinates
python tools/aea.py decode tests/kat/cambridge.bin

# 3. Export as structured JSON
python tools/aea.py json tests/kat/cambridge.bin
```

### Option B: In-Browser Offline Inspector
Open `tools/inspector.html` in any modern web browser (Chrome, Firefox, Safari, Edge).  
Drag and drop `tests/kat/cambridge.bin` or click **"⚡ Load Cambridge 2009 Test Vector"**.  
*Runs 100% client-side via the W3C Web Crypto API with zero network calls.*

### Option C: Build and Test the Rust Flight Core
```bash
# Run unit tests and known-answer test suite
cargo test

# Build bare-metal static library for ARM Cortex-M4/M7
cargo build --target thumbv7em-none-eabihf --release
```

### Option D: Standalone C Integration (No Rust Toolchain Needed)
Existing C/C++ flight software can link `include/aea.h` directly:
```bash
cd examples/c_caller
make run
# Compiles main.c, seals the record, asserts byte-for-byte SHA-256 KAT match,
# and verifies tamper detection (+1mm perturbation fails hash).
```

---

## 5. Scope & Engineering Boundaries

To maintain rigorous aerospace engineering integrity, the boundaries of `aea` are strictly defined:

* **What it IS:** A single-epoch denied-link state sealer, flight blackbox serialization schema, and FAIR archival standard.
* **What it is NOT:** It is **not** a navigation filter (Kalman filter / EKF), **not** an orbital propagator, **not** a fluid dynamics solver, and **not** a new theory of gravity.
* **Companion Role:** It operates as the binary companion to ROS 2 odometry (`nav_msgs/Odometry`) and CCSDS Ephemeris Messages (OEM ISO 26900), providing an integrity-checked, byte-stable record on microcontroller silicon when links drop.

> **Runtime Residue Note:** `libaea.a` contains only the minimal `core` and `compiler_builtins` object code emitted by `rustc`. There is no C standard library runtime, no Rust standard library, no unwinding runtime, no threads, and no dynamic heap allocation.

---

## 6. Repository Layout

```text
├── Cargo.toml                 # Package definition (MIT OR Apache-2.0, staticlib)
├── rust-toolchain.toml        # Pinned compiler (1.96.0) + bare-metal target
├── LICENSE                    # Dual license file
├── NOTICE                     # Vendored SHA-256 provenance
├── README.md                  # This document
│
├── include/
│   └── aea.h                  # Canonical C/C++ public API header
│
├── src/
│   ├── lib.rs                 # #![no_std] entry point, panic abort handler, C ABI
│   ├── state.rs               # AeaRecord layout, seal, and verify implementation
│   └── sha256.rs              # Vendored zero-dependency FIPS 180-4 SHA-256
│
├── docs/
│   ├── ICD.md                 # Interface Control Document (wire specification)
│   └── SCOPE.md               # Scope boundaries (what AEA is and is not)
│
├── tests/
│   ├── layout.rs              # Struct size (136 B) and field offset regression tests
│   ├── kat.rs                 # Known-Answer Test runner (Cambridge 2009)
│   └── kat/
│       ├── cambridge.bin      # 136-byte canonical test vector binary
│       ├── cambridge.hash     # Expected SHA-256 hex string (9f6cc109...)
│       └── cambridge_v1.json  # Annotated JSON vector with geodetic provenance
│
├── tools/
│   ├── aea.py                 # Zero-dependency Python reference decoder & CLI
│   └── inspector.html         # Standalone client-side browser binary inspector
│
└── examples/
    └── c_caller/
        ├── main.c             # Standalone C example
        └── Makefile           # Pure C compilation against staticlib
```

---

## 7. Contributions & Bug Reports

Issues are welcome for ICD specification discrepancies, byte-layout edge cases, or compiler toolchain compatibility; this repository is maintained as a published specification reference, not an open-ended community forum.

---

## 8. License

Dual-licensed under either:
* **MIT License** ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
* **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
