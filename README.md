# Abrams Event Address (AEA)

[![Standard: AEA-STATE/1](https://img.shields.io/badge/Wire%20Standard-AEA--STATE%2F1%20(136%20B)-0284c7.svg)](#2-the-wire-reality-aea-state1-136-bytes)
[![C-ABI](https://img.shields.io/badge/C--ABI-Naturally%20Aligned%20(136%20B)-10b981.svg)](include/aea.h)
[![Rust: no_std](https://img.shields.io/badge/Rust-%23!%5Bno__std%5D-orange.svg)](src/lib.rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-blue.svg)](LICENSE)
[![Verification: SHA-256 KAT](https://img.shields.io/badge/KAT%20Seal-Passed%20(9f6cc109...)-success.svg)](tests/kat/cambridge_v1.json)
[![CI](https://img.shields.io/badge/CI-Passing-238636.svg)](#)
[![Live Demo](https://img.shields.io/badge/Live%20Demo-Web%20Inspector-2ea44f.svg)](https://unlimitedinfinit.github.io/Abrams-Event-Address-AEA/tools/inspector.html)

**A deterministic 4D spacetime event addressing protocol and zero-heap `#![no_std]` integrity sealer for autonomous systems, flight blackboxes, and cross-domain state continuity.**

---

The **Abrams Event Address (AEA)** is a universal **4D spacetime coordinate standard** designed to anchor physical events across terrestrial, orbital, cislunar, and deep-space frameworks.

In relativistic physics and the model of **block time (eternalism)**, events do not merely happen and vanish in isolation: they occupy fixed, deterministic coordinates in the four-dimensional manifold of spacetime (where and when). While traditional navigation systems like GPS locate an object only on Earth's moving crust at the current second, AEA treats spacetime as an addressable namespace. It replaces fragmented civil conventions with deterministic 4D event indexing anchored to continuous atomic time and celestial reference frames.

To serve real-world robotics and flight avionics, this 4D coordinate is implemented as **AEA-STATE/1**: a compact, 136-byte integrity-checked record. It binds the central body, the coordinate chart (ITRF2020, GCRS, BCRS), continuous atomic time (TAI), 3D kinematics, attitude, and a SHA-256 cryptographic seal into a single atomic object. It does not replace GPS or navigation filters: it is the zero-heap, bare-metal companion they emit so that when communication drops, every vehicle retains a deterministic, verifiable record of where and when it existed in the physical cosmos.

> **Engineering Scope & Relativistic Realism Note:** AEA-STATE/1 is a coordinate tagging standard and cryptographic state sealer, not an active relativistic numerical integrator or general relativity solver. It tags kinematic state against declared reference frames (ITRF2020, GCRS, BCRS) and monotonic atomic clocks (TAI, TT, TDB) so that downstream trajectory filters can compute relativistic transformations without lost or ambiguous frame conventions. The 32-byte SHA-256 seal guarantees bit-level data integrity against corruption and single-event upsets; it verifies that the recorded telemetry is unchanged, rather than asserting absolute external truth.

---

### The Canonical Event Address

```text
AEA:399:TAI:1246190426.0:ITRF2020:3916909.5232774816,8057.749278632174,5016918.648210711:0,0,0:0,0,0,1:9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e
```

> **Stephen Hawking Time-Traveler Reception** · Cambridge, UK · `2009-06-28 12:00 UTC`  
> Input: WGS 84 ellipsoid ECEF, tagged `ITRF2020` for test vector. Not an official IERS station coordinate.

#### In Plain English: What This String Actually Tells You

Reading this canonical address from left to right translates directly into physical reality:

> *"This event took place on Earth (`399`), timed on continuous atomic time (`TAI`) at timestamp `1,246,190,426.0` (corresponding to 2009-06-28 12:00:00 UTC under the ICD conversion rule). Position is measured in an Earth-fixed grid (`ITRF2020`) at Cartesian coordinates `[3916909.52m, 8057.75m, 5016918.65m]`: which converts on the WGS 84 ellipsoid to `52.205878° N, 0.117867° E` (Stephen Hawking's reception room in Cambridge, UK). Velocity is stationary relative to the chart (`0, 0, 0 m/s`), attitude is an identity quaternion (`0, 0, 0, 1`, unmeasured for this KAT), and the entire state is sealed with a cryptographic fingerprint (`9f6cc109...`) that fails verification if any bit of the prefix is corrupted."*

#### Field-by-Field Breakdown

| Element in String | Value | What It Means in Plain English |
|---|---|---|
| **Protocol Tag** | `AEA` | Declares this as an **Abrams Event Address** record. |
| **Central Body** | `399` | **NASA/NAIF Body ID 399 = Earth's center of mass.** (If this event occurred on Mars, it would be `499`; Moon is `301`; `0` = None / no central body). |
| **Time Scale** | `TAI` | **International Atomic Time.** Unlike civil clocks (UTC), atomic time is monotonic and continuous: it never pauses or skips backwards for leap seconds. |
| **Atomic Timestamp** | `1246190426.0` | **Integer TAI seconds elapsed** for 2009-06-28 12:00:00 UTC under the ICD conversion rule (POSIX + (TAI−UTC) − 8). |
| **Reference Chart** | `ITRF2020` | **The coordinate map used.** Declares that the $(X,Y,Z)$ numbers below belong to the International Terrestrial Reference Frame 2020 (an Earth-fixed grid rotating with the planet). |
| **3D Position** | `3916909.52..., 8057.75..., 5016918.65...` | **Cartesian $X, Y, Z$ coordinates in meters** from the center of the Earth. Converting this to surface coordinates gives **$52.205878^\circ\text{ N}, 0.117867^\circ\text{ E}$** at $56.0\text{ m}$ elevation (the University of Cambridge reception room). |
| **3D Velocity** | `0, 0, 0` | **$V_x, V_y, V_z$ in meters per second** relative to the chart. `0, 0, 0` means resting stationary on the floor. |
| **Attitude Orientation** | `0, 0, 0, 1` | **Unit quaternion $[q_x, q_y, q_z, q_w]$.** Identity quaternion (attitude unmeasured for this KAT; aligned with ITRF chart axes). |
| **Integrity Seal** | `9f6cc109...` | **SHA-256 cryptographic digest** computed over the raw binary bytes of all preceding fields. If a cosmic ray bit-flip occurs in flash memory, or a coordinate is tampered by $1\text{ mm}$, this seal instantly fails verification. |

```bash
# Verify the seal yourself in 5 seconds (Python standard library only):
python tools/aea.py verify tests/kat/cambridge.bin
# Output: OK: SHA-256 integrity seal verified (9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e).
```

> **No Python installed?** Test this exact record live in your browser using the **[AEA Record Inspector](https://unlimitedinfinit.github.io/Abrams-Event-Address-AEA/tools/inspector.html)**: decodes the kinematics, computes the Bowring WGS 84 geodesy inverse, and recalculates the SHA-256 seal client-side using the W3C WebCrypto API.

---

## 1. Spacetime Addressing: Four Foundational Principles

1. **Explicit Coordinate Charts (No Frame Ambiguity):** A Cartesian coordinate $(X, Y, Z)$ without a reference chart tag cannot distinguish between an Earth-fixed rotating crust (ITRF2020), an inertial geocentric sphere (GCRS), or a solar system barycentric origin (BCRS). AEA requires declaring the reference chart.
2. **Monotonic Atomic Time (No Leap-Second Arithmetic):** Civil time scales (UTC) insert discontinuous leap seconds, introducing arithmetic errors across epoch differences. High-integrity mission kinematics require continuous atomic time (TAI) or coordinate time (TT, TDB) measured from 1970-01-01T00:00:00 on the declared time scale.
3. **Event-Epoch Geodesy (Plate Motion):** Terrestrial coordinates in ITRF are valid strictly at the event epoch. Comparing coordinates across multi-year baselines requires the caller to apply a Plate Motion Model (PMM).
4. **Cryptographic Integrity Sealing:** Flight logs stored in non-volatile memory are vulnerable to single-event upsets (SEUs) and bit-flips from cosmic radiation. The SHA-256 trailer computed across the canonical Little-Endian prefix immediately detects bit-rot, memory corruption, or packet truncation without requiring a network connection or external PKI.

<p align="center">
  <img src="docs/assets/4D.jfif" alt="4D Spacetime Event Addressing" width="100%">
</p>

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

### Option B: In-Browser Record Inspector
**[▶ Open Live Record Inspector](https://unlimitedinfinit.github.io/Abrams-Event-Address-AEA/tools/inspector.html)**: Zero-dependency client-side tool that decodes 4D coordinates, performs Bowring ellipsoid geodesy, visualizes the 136-byte memory wire map, and simulates cosmic-ray bit flips live via WebCrypto.

Auto-loads the canonical Cambridge 2009 record on start, provides one-click presets (Cambridge, Apollo 11, ISS), and allows inspecting custom `.bin` records with zero install. Runs 100% client-side via the W3C WebCrypto API with zero network calls.

### Option C: Build and Test the Rust Flight Core
```bash
# Run unit tests and known-answer test suite
cargo test

# Build bare-metal static library for ARM Cortex-M4/M7
cargo build --target thumbv7em-none-eabihf --release
```

### Option D: Standalone C Integration
Existing C/C++ flight software can consume `include/aea.h` directly. To link against the host static library built on your machine (`target/release/libaea.a`):
```bash
cd examples/c_caller
make run
# Compiles main.c, seals the record, asserts byte-for-byte SHA-256 KAT match,
# and verifies tamper detection (+1mm perturbation fails hash).
```
> *Note: For target silicon, the prebuilt bare-metal ARM Cortex-M archive is provided as a GitHub Release asset.*

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
* **MIT License** ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
* **Apache License, Version 2.0** ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
