# Abrams Event Address (AEA): AEA-STATE/1

[![CI](https://github.com/unlimitedinfinit/Abrams-Event-Address-AEA/actions/workflows/ci.yml/badge.svg)](https://github.com/unlimitedinfinit/Abrams-Event-Address-AEA/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

The CI badge above is GitHub's live workflow status, not a static image.

## Abstract

**How do you address an event in a moving four-dimensional universe?**

In physics and systems engineering, an event is a physical occurrence or state at a specific point in three-dimensional space and one-dimensional time. While modern physics treats space and time as a single four-dimensional structure, navigation, tracking, and archival systems still rely on domain-specific coordinate conventions—terrestrial datums for Earth's surface, celestial frames for deep space, and independent clocks for time. Because everyday maps move with the Earth, locations appear static; in reality, Earth is rotating, orbiting the Sun at roughly 29.78 km/s (67,000 mph), and moving through the Milky Way at roughly 220–230 km/s. Once an event record leaves its local system, defining where it occurred requires explicitly locking down when it happened, on which time scale, on which celestial body, and relative to what reference frame.

Stephen Hawking's 2009 reception for time travelers illustrates this problem: a street address, a calendar date, and a wall-clock time work on Earth's surface through shared human convention, but outside our moving terrestrial frame, those inputs are incomplete.

The Abrams Event Address (AEA) does not claim new physics. It unifies seven existing physical and contextual declarations into one explicit coordinate standard: target celestial body, epoch, atomic time scale, coordinate reference frame, 3D spatial position, frame-relative velocity, and spatial orientation, sealed with a cryptographic integrity digest. The published AEA-STATE/1 specification encodes this single-epoch state into a deterministic 136-byte binary record built on established geodetic, astrometric, and metrological standards, enabling unambiguous event addressing and cross-system state correlation across terrestrial, orbital, and planetary domains.

<p align="center">
  <a href="docs/aea-whitepaper.pdf"><img src="https://img.shields.io/badge/White_Paper-PDF-b31b1b?style=for-the-badge&logo=adobeacrobatreader&logoColor=white" alt="White Paper (PDF)"></a>
  &nbsp;&nbsp;
  <a href="docs/aea-whitepaper.docx"><img src="https://img.shields.io/badge/White_Paper-DOCX-2b579a?style=for-the-badge&logo=microsoftword&logoColor=white" alt="White Paper (DOCX)"></a>
</p>

<p align="center">
  <a href="https://doi.org/10.5281/zenodo.23970628"><img src="https://img.shields.io/badge/DOI-10.5281%2Fzenodo.23970628-informational?style=for-the-badge&logo=doi&logoColor=white" alt="DOI 10.5281/zenodo.23970628"></a>
</p>

The company repository is [Abrams-Research-Systems-LLC/Abrams-Event-Address-AEA](https://github.com/Abrams-Research-Systems-LLC/Abrams-Event-Address-AEA).

## Reference Implementation

The remainder of this README documents the AEA-STATE/1 record format and the reference implementation in this repository.

**AEA-STATE/1 is a proposed 136-byte record format.** It tags an estimated state with a frame and a time scale, and protects the record with a SHA-256 integrity digest. The reference implementation is a zero-heap `#![no_std]` Rust library with a C ABI. Wire-format version 1.0 (reference implementation release v1.0.3, ABI version 1.0 via `aea_version() = 0x00010000`).

## What it is
- A fixed, naturally aligned, 136-byte binary record. Its contents are: body (NAIF ID), time (seconds and nanoseconds on a declared time scale), reference frame, position, velocity, and an attitude quaternion. A SHA-256 digest of the first 104 bytes follows.
- A small C-callable library: exactly three functions: `aea_seal`, `aea_verify`, `aea_version`.
- **Integrity only.** It detects modification to the sealed 104-byte prefix (such as bit flips, memory or flash corruption, and truncation; 1,082 of 1,088 single-bit flips detected). It does not stop deliberate tampering if the digest is recomputed.

## What it is not
- It is **not authentication.** The digest is unkeyed, so anyone who can modify a record can recompute it. It does not protect against a deliberate attacker or spoofing. An authenticated profile is specified separately as a draft and is not implemented: [docs/AEA-AUTH-DRAFT.md](docs/AEA-AUTH-DRAFT.md).
- It is not a navigation filter, propagator, or frame/time converter. The caller supplies a state that is already expressed in the declared frame and time scale.
- It does not by itself improve GNSS-denied navigation, drift, EW resilience, or spoofing detection. Those are open research questions.
- It is not a ratified standard and it is not certified (DO-178C or otherwise).

## Wire layout (see [docs/ICD.md](docs/ICD.md))
| Offset | Size | Type | Field |
|---:|---:|---|---|
| 0 | 4 | u32 | `version` (=1) |
| 4 | 4 | u32 | `body_naif` (399 Earth geocenter, 301 Moon, 499 Mars, 0 = none, not a barycenter) |
| 8 | 8 | i64 | `time_sec`: seconds since 1970-01-01T00:00:00 on the declared scale |
| 16 | 4 | u32 | `time_nsec` (< 1e9) |
| 20 | 2 | u16 | `frame` (1 ITRF2020, 2 GCRS, 3 BCRS, 4 BODY) |
| 22 | 2 | u16 | `time_scale` (1 TAI, 2 TT, 3 TCB, 4 TDB) |
| 24 | 24 | f64[3] | `pos_m` |
| 48 | 24 | f64[3] | `vel_mps` |
| 72 | 32 | f64[4] | `quat_xyzw` (identity = unmeasured; unit norm **not** enforced by seal) |
| 104 | 32 | u8[32] | SHA-256 of the canonical little-endian encoding of bytes [0,104) |

## Verify it yourself
```bash
cargo test                                   # unit, layout, 20 KAT vectors, exhaustive bit-flip test
python3 tools/gen_kat.py --check             # independent Python generator matches committed vectors
python3 tools/aea.py verify tests/kat/cambridge.bin
cargo build --release --target thumbv7em-none-eabihf   # also thumbv6m-none-eabi, thumbv8m.main-none-eabihf
cd examples/c_caller && cargo build --release --manifest-path ../../Cargo.toml && make run
```
Known-answer vectors are listed in `tests/kat/vectors.json` (human-readable) and `tests/kat/vectors.txt` (machine-readable). There are 20 vectors covering all frames and time scales, edge cases, and every error code. The original Cambridge vector is unchanged: `9f6cc1096562729a902167fd622e66a264a239f3e9a9d804414659e3ab97800e`.

## Measured facts (with method)
- Code plus read-only data (llvm-size text column, release + LTO, excluding compiler_builtins soft-float helpers): see `test_results/embedded_sizes.txt`. Builds for Cortex-M0 (2,329 B), M4/M7 (2,315 B), and M33 (2,319 B). Not yet run on target hardware.
- Stack usage and cycle counts have **not** been measured yet. They are on the roadmap.
- Big-endian: the tests are wired to run on s390x under QEMU in CI. See CI for status.
- Bit-flip coverage: all 1,088 single-bit flips of the sealed Cambridge record are detected, with 6 documented exceptions. Those 6 are sign flips of fields equal to +0.0, which are canonicalized by design (see `tests/bitflip.rs`).

## Limitations
- Cortex-M4F has a single-precision FPU only, so double-precision handling uses software routines.
- The identity quaternion means both "unmeasured" and "aligned with frame axes". This is a known v1 ambiguity.
- ITRF coordinates are valid at the event epoch only. Comparing across years needs a plate-motion model applied by the user.
- Time is seconds since 1970-01-01 on the declared scale. Operational UTC→TAI for post-1972 instants is `POSIX + (TAI−UTC) − 8` (see ICD §6.3).
- The SHA-256 implementation is not CAVP/CMVP validated.

## Research roadmap (headline)
These are open questions, not capabilities. We plan to investigate:
- an authenticated, sequence-numbered profile;
- whether sealed records help drift detection during GNSS outages;
- whether sealed records support cross-node consistency checks;
- measured embedded cost on Cortex-M/R and LEON.

## Repository layout
`src/` library · `include/aea.h` C header · `tests/` tests and KAT data · `tools/` Python reference decoder, KAT generator, and browser inspector · `fuzz/` cargo-fuzz targets · `examples/c_caller` · `docs/` ICD, scope, auth draft, and white paper.

No prebuilt binaries are released at this time. Build from source as shown above.

## License
MIT OR Apache-2.0. The full license texts are available in [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

## Building a release archive

```sh
tools/make_release.sh v1.0.3 dist   # git archive --prefix=aea-1.0.3/ v1.0.3 | gzip -n, plus dist/SHA256SUMS
```
