// Metal-cpp scaffolding for deterministic BLAKE3 hashing on Apple Silicon GPUs.
// This example builds a tiny compute pipeline entirely in C++ to mirror the Mojo GPU sketch.
// Replace the placeholder kernels with the real BLAKE3 compression once available.

#include <Metal/Metal.hpp>
#include <Foundation/Foundation.hpp>
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

namespace blake3_metal {

constexpr const char *kAppleMetalDataLayout =
    "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-"
    "f32:32:32-f64:64:64-v16:16:16-v24:32:32-v32:32:32-v48:64:64-"
    "v64:64:64-v96:128:128-v128:128:128-v192:256:256-v256:256:256-"
    "v512:512:512-v1024:1024:1024-n8:16:32";

// Template stub that mirrors the provided signature so it can be wired to Mojo FFI later.
template <typename C, typename A, typename B>
bool use_apple_accelerate_lib() {
    // For now always prefer Apple GPU/Accelerate path when available.
    return true;
}

struct ChunkInput {
    std::array<uint8_t, 1024> data{};
    uint64_t counter{0};
};

struct MetalContext {
    NS::SharedPtr<MTL::Device> device;
    NS::SharedPtr<MTL::CommandQueue> queue;
    NS::SharedPtr<MTL::ComputePipelineState> chunk_pipeline;
    NS::SharedPtr<MTL::ComputePipelineState> reduce_pipeline;
};

static NS::SharedPtr<MTL::Library> buildLibrary(const NS::SharedPtr<MTL::Device> &device,
                                               const std::string &source) {
    NS::Error *error = nullptr;
    auto ns_source = NS::String::string(source.c_str(), NS::UTF8StringEncoding);
    auto options = NS::TransferPtr(MTL::CompileOptions::alloc()->init());
    auto library = NS::TransferPtr(device->newLibrary(ns_source, options.get(), &error));
    if (error) {
        std::cerr << "Metal library build failed: "
                  << NS::String(error->localizedDescription())->utf8String() << std::endl;
        return nullptr;
    }
    return library;
}

static MetalContext createContext() {
    MetalContext ctx{};
    ctx.device = NS::TransferPtr(MTL::CreateSystemDefaultDevice());
    ctx.queue = NS::TransferPtr(ctx.device->newCommandQueue());

    const std::string metal_source = R"METAL(
    #include <metal_stdlib>
    using namespace metal;

    struct ChunkInput { device uchar data[1024]; ulong counter; };

    kernel void blake3_hash_chunks(const device ChunkInput *chunks,
                                   constant uint &chunk_count,
                                   device uchar *out_hashes,
                                   uint gid [[thread_position_in_grid]]) {
        if (gid >= chunk_count) return;
        // TODO: load 1024-byte chunk and run real BLAKE3 compression.
        // Stub writes zero hash (32 bytes) for determinism.
        const uint base = gid * 32;
        for (uint i = 0; i < 32; ++i) {
            out_hashes[base + i] = 0;
        }
    }

    kernel void blake3_reduce_level(const device uchar *child_hashes,
                                    constant uint &child_count,
                                    device uchar *parent_hashes,
                                    uint gid [[thread_position_in_grid]]) {
        uint parent_idx = gid * 2;
        if (parent_idx + 1 >= child_count) return; // odd child carried by host
        uint in_base = parent_idx * 32;
        uint out_base = gid * 32;
        // TODO: run compress(left||right). For now copy left deterministically.
        for (uint i = 0; i < 32; ++i) {
            parent_hashes[out_base + i] = child_hashes[in_base + i];
        }
    }
    )METAL";

    auto library = buildLibrary(ctx.device, metal_source);
    auto chunk_fn = NS::String::string("blake3_hash_chunks", NS::UTF8StringEncoding);
    auto reduce_fn = NS::String::string("blake3_reduce_level", NS::UTF8StringEncoding);

    NS::Error *error = nullptr;
    auto chunk_function = NS::TransferPtr(library->newFunction(chunk_fn));
    ctx.chunk_pipeline = NS::TransferPtr(ctx.device->newComputePipelineState(chunk_function.get(), &error));
    if (error) {
        std::cerr << "Chunk pipeline build failed: "
                  << NS::String(error->localizedDescription())->utf8String() << std::endl;
    }

    auto reduce_function = NS::TransferPtr(library->newFunction(reduce_fn));
    ctx.reduce_pipeline = NS::TransferPtr(ctx.device->newComputePipelineState(reduce_function.get(), &error));
    if (error) {
        std::cerr << "Reduce pipeline build failed: "
                  << NS::String(error->localizedDescription())->utf8String() << std::endl;
    }
    return ctx;
}

static std::vector<std::array<uint8_t, 32>> hashChunks(MetalContext &ctx,
                                                      const std::vector<ChunkInput> &chunks) {
    if (!ctx.chunk_pipeline) return {};
    const uint32_t chunk_count = static_cast<uint32_t>(chunks.size());
    auto command_buffer = NS::TransferPtr(ctx.queue->commandBuffer());
    auto encoder = NS::TransferPtr(command_buffer->computeCommandEncoder());
    encoder->setComputePipelineState(ctx.chunk_pipeline.get());

    auto chunk_buffer = NS::TransferPtr(ctx.device->newBuffer(chunks.data(),
                                                              sizeof(ChunkInput) * chunks.size(),
                                                              MTL::ResourceStorageModeShared));
    auto out_buffer = NS::TransferPtr(ctx.device->newBuffer(32 * chunks.size(),
                                                            MTL::ResourceStorageModeShared));

    encoder->setBuffer(chunk_buffer.get(), 0, 0);
    encoder->setBytes(&chunk_count, sizeof(chunk_count), 1);
    encoder->setBuffer(out_buffer.get(), 0, 2);

    MTL::Size grid(chunk_count, 1, 1);
    auto max_threads = ctx.chunk_pipeline->maxTotalThreadsPerThreadgroup();
    MTL::Size tg(std::min<uint32_t>(max_threads, 64), 1, 1);
    encoder->dispatchThreads(grid, tg);
    encoder->endEncoding();
    command_buffer->commit();
    command_buffer->waitUntilCompleted();

    std::vector<std::array<uint8_t, 32>> hashes(chunks.size());
    std::memcpy(hashes.data(), out_buffer->contents(), 32 * chunks.size());
    return hashes;
}

static std::vector<std::array<uint8_t, 32>> reduceLevel(MetalContext &ctx,
                                                       const std::vector<std::array<uint8_t, 32>> &children) {
    if (!ctx.reduce_pipeline) return {};
    if (children.size() < 2) return children;

    const uint32_t child_count = static_cast<uint32_t>(children.size());
    const uint32_t parent_count = child_count / 2;

    auto command_buffer = NS::TransferPtr(ctx.queue->commandBuffer());
    auto encoder = NS::TransferPtr(command_buffer->computeCommandEncoder());
    encoder->setComputePipelineState(ctx.reduce_pipeline.get());

    auto child_buffer = NS::TransferPtr(ctx.device->newBuffer(children.data(),
                                                              32 * children.size(),
                                                              MTL::ResourceStorageModeShared));
    auto parent_buffer = NS::TransferPtr(ctx.device->newBuffer(32 * parent_count,
                                                               MTL::ResourceStorageModeShared));

    encoder->setBuffer(child_buffer.get(), 0, 0);
    encoder->setBytes(&child_count, sizeof(child_count), 1);
    encoder->setBuffer(parent_buffer.get(), 0, 2);

    MTL::Size grid(parent_count, 1, 1);
    auto max_threads = ctx.reduce_pipeline->maxTotalThreadsPerThreadgroup();
    MTL::Size tg(std::min<uint32_t>(max_threads, 64), 1, 1);
    encoder->dispatchThreads(grid, tg);
    encoder->endEncoding();
    command_buffer->commit();
    command_buffer->waitUntilCompleted();

    std::vector<std::array<uint8_t, 32>> parents(parent_count);
    std::memcpy(parents.data(), parent_buffer->contents(), 32 * parent_count);

    // Deterministic carry of odd child to preserve Merkle ordering.
    if (child_count % 2 == 1) {
        parents.push_back(children.back());
    }
    return parents;
}

static std::array<uint8_t, 32> hashFile(const std::vector<ChunkInput> &chunks) {
    MetalContext ctx = createContext();
    auto level = hashChunks(ctx, chunks);
    while (level.size() > 1) {
        level = reduceLevel(ctx, level);
    }
    if (level.empty()) return {};
    return level.front();
}

}  // namespace blake3_metal

int main() {
    using namespace blake3_metal;
    std::vector<ChunkInput> inputs(2);
    inputs[0].counter = 0;
    inputs[1].counter = 1;

    auto root_hash = hashFile(inputs);
    std::cout << "Got stub hash bytes: ";
    for (auto b : root_hash) {
        std::cout << std::hex << static_cast<int>(b) << " ";
    }
    std::cout << std::dec << "\n";
    std::cout << "Apple Metal data layout: " << kAppleMetalDataLayout << "\n";
    return 0;
}
