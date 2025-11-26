# Metal C++ scaffolding for BLAKE3

This folder holds a minimal, deterministic Metal C++ pipeline that mirrors the Mojo BLAKE3 GPU sketch. It uses the official [Metal-cpp](https://developer.apple.com/metal/cpp/) bindings and keeps the data layout and launch shape explicit for Apple GPUs.

## Contents
- `metal_blake3_example.cpp`: End-to-end C++ example that sets up the device, builds a tiny Metal compute library from source, encodes chunk hashing and reduction passes, and demonstrates unified-memory reads on Apple Silicon.

## Apple GPU data layout
Metal GPUs on Apple Silicon use the following layout:

```
e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v16:16:16-v24:32:32-v32:32:32-v48:64:64-v64:64:64-v96:128:128-v128:128:128-v192:256:256-v256:256:256-v512:512:512-v1024:1024:1024-n8:16:32
```

We surface this as a compile-time constant in the C++ scaffolding so it can be reused by future Mojo FFI shims when building MLIR targets for Metal.

## Building the example
The example expects macOS with Xcode toolchains and Metal-cpp headers available (see [Metal-cpp download](https://developer.apple.com/metal/cpp/)).

```bash
clang++ -std=c++20 -fobjc-arc \
  -I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Metal.framework/Headers \
  -I/path/to/metal-cpp \
  mojo-blake3/src/metal_cpp/metal_blake3_example.cpp \
  -framework Metal -framework Foundation -o metal_blake3_example
```

Run the binary on Apple Silicon to see the stub hashes returned from the GPU pass:

```bash
./metal_blake3_example
```

Once the Mojo GPU kernels are ready, replace the placeholder compute functions in the embedded Metal source string with the real BLAKE3 compression logic and wire Mojo FFI to feed buffers into this pipeline.
