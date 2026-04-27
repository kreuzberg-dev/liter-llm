const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");

    const batches_module = b.createModule(.{
        .root_source_file = b.path("src/batches_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const batches_tests = b.addTest(.{
        .root_module = batches_module,
    });
    const batches_run = b.addRunArtifact(batches_tests);
    test_step.dependOn(&batches_run.step);

    const cache_module = b.createModule(.{
        .root_source_file = b.path("src/cache_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const cache_tests = b.addTest(.{
        .root_module = cache_module,
    });
    const cache_run = b.addRunArtifact(cache_tests);
    test_step.dependOn(&cache_run.step);

    const chat_module = b.createModule(.{
        .root_source_file = b.path("src/chat_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const chat_tests = b.addTest(.{
        .root_module = chat_module,
    });
    const chat_run = b.addRunArtifact(chat_tests);
    test_step.dependOn(&chat_run.step);

    const configuration_module = b.createModule(.{
        .root_source_file = b.path("src/configuration_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const configuration_tests = b.addTest(.{
        .root_module = configuration_module,
    });
    const configuration_run = b.addRunArtifact(configuration_tests);
    test_step.dependOn(&configuration_run.step);

    const contract_module = b.createModule(.{
        .root_source_file = b.path("src/contract_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const contract_tests = b.addTest(.{
        .root_module = contract_module,
    });
    const contract_run = b.addRunArtifact(contract_tests);
    test_step.dependOn(&contract_run.step);

    const custom_provider_module = b.createModule(.{
        .root_source_file = b.path("src/custom_provider_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const custom_provider_tests = b.addTest(.{
        .root_module = custom_provider_module,
    });
    const custom_provider_run = b.addRunArtifact(custom_provider_tests);
    test_step.dependOn(&custom_provider_run.step);

    const embed_module = b.createModule(.{
        .root_source_file = b.path("src/embed_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const embed_tests = b.addTest(.{
        .root_module = embed_module,
    });
    const embed_run = b.addRunArtifact(embed_tests);
    test_step.dependOn(&embed_run.step);

    const error_handling_module = b.createModule(.{
        .root_source_file = b.path("src/error_handling_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const error_handling_tests = b.addTest(.{
        .root_module = error_handling_module,
    });
    const error_handling_run = b.addRunArtifact(error_handling_tests);
    test_step.dependOn(&error_handling_run.step);

    const files_module = b.createModule(.{
        .root_source_file = b.path("src/files_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const files_tests = b.addTest(.{
        .root_module = files_module,
    });
    const files_run = b.addRunArtifact(files_tests);
    test_step.dependOn(&files_run.step);

    const image_generate_module = b.createModule(.{
        .root_source_file = b.path("src/image_generate_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const image_generate_tests = b.addTest(.{
        .root_module = image_generate_module,
    });
    const image_generate_run = b.addRunArtifact(image_generate_tests);
    test_step.dependOn(&image_generate_run.step);

    const list_models_module = b.createModule(.{
        .root_source_file = b.path("src/list_models_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const list_models_tests = b.addTest(.{
        .root_module = list_models_module,
    });
    const list_models_run = b.addRunArtifact(list_models_tests);
    test_step.dependOn(&list_models_run.step);

    const moderate_module = b.createModule(.{
        .root_source_file = b.path("src/moderate_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const moderate_tests = b.addTest(.{
        .root_module = moderate_module,
    });
    const moderate_run = b.addRunArtifact(moderate_tests);
    test_step.dependOn(&moderate_run.step);

    const ocr_module = b.createModule(.{
        .root_source_file = b.path("src/ocr_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const ocr_tests = b.addTest(.{
        .root_module = ocr_module,
    });
    const ocr_run = b.addRunArtifact(ocr_tests);
    test_step.dependOn(&ocr_run.step);

    const parity_module = b.createModule(.{
        .root_source_file = b.path("src/parity_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const parity_tests = b.addTest(.{
        .root_module = parity_module,
    });
    const parity_run = b.addRunArtifact(parity_tests);
    test_step.dependOn(&parity_run.step);

    const proxy_module = b.createModule(.{
        .root_source_file = b.path("src/proxy_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const proxy_tests = b.addTest(.{
        .root_module = proxy_module,
    });
    const proxy_run = b.addRunArtifact(proxy_tests);
    test_step.dependOn(&proxy_run.step);

    const rerank_module = b.createModule(.{
        .root_source_file = b.path("src/rerank_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const rerank_tests = b.addTest(.{
        .root_module = rerank_module,
    });
    const rerank_run = b.addRunArtifact(rerank_tests);
    test_step.dependOn(&rerank_run.step);

    const responses_module = b.createModule(.{
        .root_source_file = b.path("src/responses_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const responses_tests = b.addTest(.{
        .root_module = responses_module,
    });
    const responses_run = b.addRunArtifact(responses_tests);
    test_step.dependOn(&responses_run.step);

    const search_module = b.createModule(.{
        .root_source_file = b.path("src/search_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const search_tests = b.addTest(.{
        .root_module = search_module,
    });
    const search_run = b.addRunArtifact(search_tests);
    test_step.dependOn(&search_run.step);

    const smoke_module = b.createModule(.{
        .root_source_file = b.path("src/smoke_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const smoke_tests = b.addTest(.{
        .root_module = smoke_module,
    });
    const smoke_run = b.addRunArtifact(smoke_tests);
    test_step.dependOn(&smoke_run.step);

    const speech_module = b.createModule(.{
        .root_source_file = b.path("src/speech_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const speech_tests = b.addTest(.{
        .root_module = speech_module,
    });
    const speech_run = b.addRunArtifact(speech_tests);
    test_step.dependOn(&speech_run.step);

    const streaming_module = b.createModule(.{
        .root_source_file = b.path("src/streaming_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const streaming_tests = b.addTest(.{
        .root_module = streaming_module,
    });
    const streaming_run = b.addRunArtifact(streaming_tests);
    test_step.dependOn(&streaming_run.step);

    const tool_calling_module = b.createModule(.{
        .root_source_file = b.path("src/tool_calling_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const tool_calling_tests = b.addTest(.{
        .root_module = tool_calling_module,
    });
    const tool_calling_run = b.addRunArtifact(tool_calling_tests);
    test_step.dependOn(&tool_calling_run.step);

    const transcribe_module = b.createModule(.{
        .root_source_file = b.path("src/transcribe_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const transcribe_tests = b.addTest(.{
        .root_module = transcribe_module,
    });
    const transcribe_run = b.addRunArtifact(transcribe_tests);
    test_step.dependOn(&transcribe_run.step);

    const types_module = b.createModule(.{
        .root_source_file = b.path("src/types_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const types_tests = b.addTest(.{
        .root_module = types_module,
    });
    const types_run = b.addRunArtifact(types_tests);
    test_step.dependOn(&types_run.step);

}
