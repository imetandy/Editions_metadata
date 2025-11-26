# Mojo BLAKE3 GPU Prototype (Apple Silicon)

This directory sketches how we could prototype a deterministic GPU-accelerated BLAKE3 pipeline on Apple Silicon using [Modular Mojo](https://www.modular.com/mojo). The goal is to keep the BLAKE3 output identical to the CPU reference while offloading chunk compression and the Merkle-tree reduction to the GPU cores.

## Proposed layout
- `src/`
  - `blake3_gpu.moj`: GPU-first implementation sketch with clearly separated phases (chunk hashing, tree reduction, CPU fallbacks).
  - `host_pipeline.moj`: Host-side orchestration for streaming large files and handling deterministic batching.
  - `metal_cpp/`: Metal-cpp scaffolding that mirrors the Mojo layout and demonstrates a fully wired C++ pipeline against Apple GPUs.
- `tests/`
  - Placeholder for CPU vs GPU equivalence and performance sanity checks once Mojo has stable GPU runners on macOS.

## Determinism strategy
- **Fixed chunking**: Keep the canonical 1 KiB BLAKE3 chunks; do not alter the tree topology when scheduling GPU work.
- **Stable work distribution**: Use deterministic grid/block sizing derived only from input length (e.g., one thread per chunk, fixed workgroup size). Avoid dynamic scheduling or atomics that introduce non-deterministic ordering.
- **Ordered reductions**: Perform pairwise reductions in a fixed left-to-right order; when the level is odd, carry the last hash forward unchanged to mirror the reference algorithm.
- **CPU truth source**: Retain a CPU reference implementation and compare every GPU-produced chunk hash during development to guarantee correctness.

## Build & run expectations
- Requires a Mojo toolchain with Metal backend enabled. On Apple Silicon, GPU dispatch currently needs the nightly toolchains from Modular.
- A simple driver (see `host_pipeline.moj`) would stream file chunks, copy them to GPU buffers, and read back the final root hash. Until Mojo supports async file I/O on macOS, keep the host loop single-threaded to avoid I/O jitter in benchmarks.
- For a C++-only reference of the same launch shape, see `src/metal_cpp/metal_blake3_example.cpp`. It uses Metal-cpp to build a minimal compute library in memory, dispatch chunk hashing, and fold hashes deterministically, so Mojo FFI can mirror the same contract.

## Next steps
1. Flesh out `blake3_gpu.moj` with real kernels once the Metal backend is available in the target environment.
2. Add tests comparing CPU vs GPU digests for varied file sizes and unaligned endings.
3. Integrate with the existing BLAKE3 code path by feature flag, falling back to CPU when GPU is unavailable.
