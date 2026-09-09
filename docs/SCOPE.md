# Architectural Scope & Boundaries: AEA-STATE/1

## 1. Executive Mission
The primary objective of `aea-core` is to provide an uncompromising, zero-heap, bare-metal C-ABI static library (`libaea.a`) that autonomous platforms use to freeze their estimated state into an integrity-checked, byte-stable 136-byte record when communication links, GNSS, or mesh networks drop.

---

## 2. What AEA Is

* **A Denied-Link State Labeler:** A real-time data formatter and cryptographic checksum generator. Flight computers call `aea_seal()` to freeze an EKF state estimate into flash memory before a crash, link drop, or power cycle.
* **A Standardized Content Schema:** An unambiguous single-epoch binary schema ensuring that a UAV, an underwater AUV, a LEO satellite, and a planetary probe serialize telemetry under the exact same memory layout and coordinate chart tags.
* **A FAIR Archival Primitive:** A deterministic format enabling future researchers (the "2040 Student Scenario") to parse multi-agency, multi-domain telemetry logs with a trivial 15-line parser without deciphering legacy CSV formats.
* **A Binary Companion to CCSDS OEM:** While space agencies exchange large, text-heavy XML/KVN ephemeris files (ISO 26900:2024) between ground stations, AEA serves as the compact, zero-heap binary state snapshot sealed directly on microcontroller silicon.

---

## 3. What AEA Is NOT

Aerospace and defense contractors require strict, bounded definitions. The following are explicitly out of scope for `libaea.a`:

* **NOT a Navigation Engine:** It does not observe GNSS satellite constellations, read IMU registers, track stars, or filter sensor noise. It is an output formatter, not an estimator.
* **NOT an Orbital Propagator:** It does not integrate Keplerian elements, compute gravitational perturbations ($J_2$), or propagate trajectories through time.
* **NOT a Fluid Dynamics (CFD) Solver:** It does not model Navier-Stokes equations, atmospheric turbulence, shock layers, or aerodynamic drag.
* **NOT a Propulsion or Warping Engine:** In theoretical discussions, it represents the target destination coordinate on a manifold, never the mechanism of travel.
* **NOT a Blockchain or Authenticated Ledger:** The 32-byte SHA-256 digest is an integrity trailer against bit-rot and memory corruption. It provides no digital signature, no identity proof, and no distributed consensus. Digital signatures (e.g. Ed25519) must wrap the record externally.
* **NOT a Floating-Point Math Engine:** It performs no coordinate frame transformations internally. Converting local NED to ITRF2020 or UTC to TAI is the explicit duty of the platform's host adapter stack prior to calling `aea_seal()`.
