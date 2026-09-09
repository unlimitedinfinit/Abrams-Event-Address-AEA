/**
 * AEA-STATE/1 — Public C ABI
 *
 * 136-byte record, Little-Endian, IEEE-754 binary64.
 * Field order keeps every 8-byte field naturally aligned on an 8-byte boundary.
 * Use as #[repr(C)] in Rust. Do NOT pack with __attribute__((packed)).
 *
 * This header defines the ONLY public C interface to libaea.a.
 */

#ifndef AEA_H
#define AEA_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define AEA_VERSION_MAJOR 1u
#define AEA_VERSION_MINOR 0u
#define AEA_RECORD_BYTES  136u
#define AEA_PREFIX_BYTES  104u
#define AEA_HASH_BYTES    32u

/* Return / Status Codes */
#define AEA_OK               0   /* Success, or verified integrity match */
#define AEA_ERR_MISMATCH     1   /* Verification failed: hash mismatch */
#define AEA_ERR_NULL_PTR   (-1)  /* Null pointer passed to API */
#define AEA_ERR_VERSION    (-2)  /* Unsupported wire version (!= 1) */
#define AEA_ERR_NSEC_BOUND (-3)  /* Nanoseconds out of bounds (>= 1,000,000,000) */
#define AEA_ERR_FRAME      (-4)  /* Unrecognized or invalid coordinate frame enum */
#define AEA_ERR_TIME_SCALE (-5)  /* Unrecognized or invalid time scale enum */
#define AEA_ERR_NON_FINITE (-6)  /* Non-finite float (NaN, +Inf, -Inf) encountered */

/**
 * Coordinate Time Scale Enums.
 * 0 is explicitly reserved as INVALID.
 */
enum AeaTimeScale {
    AEA_TIME_INVALID = 0,
    AEA_TIME_TAI     = 1,  /* International Atomic Time */
    AEA_TIME_TT      = 2,  /* Terrestrial Time */
    AEA_TIME_TCB     = 3,  /* Barycentric Coordinate Time */
    AEA_TIME_TDB     = 4   /* Barycentric Dynamical Time */
};

/**
 * Coordinate Reference Frame Enums.
 * 0 is explicitly reserved as INVALID.
 */
enum AeaFrame {
    AEA_FRAME_INVALID  = 0,
    AEA_FRAME_ITRF2020 = 1,  /* Earth-fixed Cartesian metres (IERS ITRF2020) */
    AEA_FRAME_GCRS     = 2,  /* Geocentric Celestial Reference System (IAU) */
    AEA_FRAME_BCRS     = 3,  /* Barycentric Celestial Reference System (IAU) */
    AEA_FRAME_BODY     = 4   /* Body-fixed Cartesian (centered on body_naif) */
};

/**
 * The 136-byte Canonical AEA State Record.
 * Naturally 8-byte aligned. Total struct size is exactly 136 bytes.
 */
typedef struct {
    uint32_t version;       /* Offset  0: Wire format version (must be 1) */
    uint32_t body_naif;     /* Offset  4: NAIF Body ID (399=Earth, 499=Mars, 0=none) */
    int64_t  time_sec;      /* Offset  8: Integer seconds elapsed since 1970-01-01 on declared scale */
    uint32_t time_nsec;     /* Offset 16: Integer nanoseconds (0 to 999,999,999) */
    uint16_t frame;         /* Offset 20: Coordinate frame enum (enum AeaFrame) */
    uint16_t time_scale;    /* Offset 22: Time scale enum (enum AeaTimeScale) */
    double   pos_m[3];      /* Offset 24: Cartesian position [X, Y, Z] in metres */
    double   vel_mps[3];    /* Offset 48: Cartesian velocity [Vx, Vy, Vz] in metres/second */
    double   quat_xyzw[4];  /* Offset 72: Attitude quaternion [x, y, z, w] (Null: 0,0,0,1) */
    uint8_t  hash[32];      /* Offset 104: SHA-256 digest over prefix [0, 104) */
} AeaRecord;

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
_Static_assert(sizeof(AeaRecord) == 136, "AeaRecord must be exactly 136 bytes");
#endif

/**
 * Validates record fields, encodes the 104-byte Little-Endian prefix,
 * computes the SHA-256 digest, and writes the 32-byte digest into record->hash.
 *
 * Canonicalizes -0.0 to +0.0 across all float fields prior to hashing.
 *
 * @param record Pointer to mutable AeaRecord.
 * @return AEA_OK (0) on success, or negative error code on validation failure.
 */
int32_t aea_seal(AeaRecord *record);

/**
 * Validates record fields, recomputes the SHA-256 digest over the 104-byte
 * prefix, and compares it with record->hash in a fixed 32-iteration loop.
 *
 * @param record Pointer to const AeaRecord.
 * @return AEA_OK (0) if hash matches, AEA_ERR_MISMATCH (1) if hash does not match,
 *         or negative error code on validation failure.
 */
int32_t aea_verify(const AeaRecord *record);

/**
 * Returns library ABI version as ((MAJOR << 16) | MINOR).
 * For v1.0, returns 0x00010000 (65536).
 */
uint32_t aea_version(void);

#ifdef __cplusplus
}
#endif

#endif /* AEA_H */
