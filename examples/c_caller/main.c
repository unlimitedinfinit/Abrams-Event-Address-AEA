/**
 * AEA-STATE/1 — Standalone C Caller Verification
 *
 * Demonstrates linking and consuming libaea.a from pure C code
 * with zero Cargo or Rust toolchain dependencies on the consumer box.
 */

#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include "../../include/aea.h"

static void print_hex(const uint8_t *bytes, size_t len) {
    for (size_t i = 0; i < len; ++i) {
        printf("%02x", bytes[i]);
    }
    printf("\n");
}

int main(void) {
    printf("=== AEA-STATE/1 C Caller Verification ===\n");

    /* 1. Verify Library Version */
    uint32_t ver = aea_version();
    printf("AEA Library ABI Version: 0x%08X (Major %u, Minor %u)\n",
           ver, (ver >> 16) & 0xFFFF, ver & 0xFFFF);
    assert(((ver >> 16) & 0xFFFF) == 1);

    /* 2. Verify Compile-Time and Runtime Struct Size */
    printf("sizeof(AeaRecord): %zu bytes\n", sizeof(AeaRecord));
    assert(sizeof(AeaRecord) == 136);

    /* 3. Initialize Stephen Hawking Cambridge Test Vector */
    AeaRecord rec;
    rec.version = 1;
    rec.body_naif = 399;           /* Earth geocenter */
    rec.time_sec = 1246190426;     /* TAI corresponding to UTC 2009-06-28 12:00:00 (TAI = UTC + 34s) */
    rec.time_nsec = 0;
    rec.frame = AEA_FRAME_ITRF2020;/* Earth-fixed Cartesian */
    rec.time_scale = AEA_TIME_TAI; /* International Atomic Time */
    rec.pos_m[0] = 3916909.5232774816;
    rec.pos_m[1] = 8057.749278632174;
    rec.pos_m[2] = 5016918.648210711;
    rec.vel_mps[0] = 0.0;
    rec.vel_mps[1] = 0.0;
    rec.vel_mps[2] = 0.0;
    rec.quat_xyzw[0] = 0.0;
    rec.quat_xyzw[1] = 0.0;
    rec.quat_xyzw[2] = 0.0;
    rec.quat_xyzw[3] = 1.0;        /* Identity null attitude */

    /* 4. Seal Record */
    int32_t rc = aea_seal(&rec);
    printf("aea_seal() return code: %d\n", rc);
    assert(rc == AEA_OK);

    printf("Sealed SHA-256 Digest: ");
    print_hex(rec.hash, 32);

    /* Assert Known-Answer Test matches published Cambridge SHA-256 digest */
    static const uint8_t EXPECTED_CAMBRIDGE_HASH[32] = {
        0x9f, 0x6c, 0xc1, 0x09, 0x65, 0x62, 0x72, 0x9a,
        0x90, 0x21, 0x67, 0xfd, 0x62, 0x2e, 0x66, 0xa2,
        0x64, 0xa2, 0x39, 0xf3, 0xe9, 0xa9, 0xd8, 0x04,
        0x41, 0x46, 0x59, 0xe3, 0xab, 0x97, 0x80, 0x0e
    };
    assert(memcmp(rec.hash, EXPECTED_CAMBRIDGE_HASH, 32) == 0);
    printf("KAT Verification: Sealed digest matches published Cambridge vector 9f6cc109... byte-for-byte!\n");

    /* 5. Verify Record */
    rc = aea_verify(&rec);
    printf("aea_verify() return code (untampered): %d\n", rc);
    assert(rc == AEA_OK);

    /* 6. Tamper Detection Test */
    rec.pos_m[0] += 0.001; /* Alter X coordinate by 1 mm */
    rc = aea_verify(&rec);
    printf("aea_verify() return code (tampered +1mm): %d\n", rc);
    assert(rc == AEA_ERR_MISMATCH);

    printf("=== All AEA C-ABI checks passed successfully! ===\n");
    return 0;
}
