# Test plan (future)

This folder will house CPU vs GPU equivalence tests once the Mojo Metal backend is available in CI. Suggested cases:
- Empty file, single-chunk file, and multi-chunk file with uneven tail.
- Cross-check GPU hashes against the official BLAKE3 test vectors.
- Performance smoke tests that ensure deterministic timing by pinning grid/block sizes and running multiple iterations.
