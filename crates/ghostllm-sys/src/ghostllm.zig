const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// C-compatible types
const c = @cImport({
    @cInclude("stdint.h");
    @cInclude("stdlib.h");
    @cInclude("string.h");
});

// Core GhostLLM context structure
pub const GhostContext = struct {
    model_path: [*:0]const u8,
    max_tokens: u32,
    temperature: f32,
    allocator: Allocator,
    initialized: bool,
};

// Response structure
pub const GhostResponse = struct {
    text: [*:0]const u8,
    tokens_used: u32,
    error_code: i32,
    allocator: Allocator,
};

// Stream callback type
pub const StreamCallback = ?*const fn([*:0]const u8, usize) callconv(.c) void;

// Initialize GhostLLM context
export fn ghost_init(model_path: [*:0]const u8) ?*GhostContext {
    const allocator = std.heap.c_allocator;
    
    // Validate input - model_path is never null for [*:0]const u8 type
    const path_len = std.mem.len(model_path);
    if (path_len == 0) {
        return null;
    }
    
    const ctx = allocator.create(GhostContext) catch return null;
    
    // Duplicate the model path to ensure it stays valid
    const owned_path = allocator.allocSentinel(u8, path_len, 0) catch {
        allocator.destroy(ctx);
        return null;
    };
    
    @memcpy(owned_path, std.mem.span(model_path));
    
    ctx.* = GhostContext{
        .model_path = owned_path.ptr,
        .max_tokens = 2048,
        .temperature = 0.7,
        .allocator = allocator,
        .initialized = true,
    };
    
    return ctx;
}

// Generate response with optional streaming callback
export fn ghost_generate(
    ctx: ?*GhostContext,
    prompt: [*:0]const u8,
    callback: StreamCallback,
) ?*GhostResponse {
    if (ctx == null or std.mem.len(prompt) == 0) {
        return null;
    }
    
    const context = ctx.?;
    if (!context.initialized) {
        return null;
    }
    
    const allocator = context.allocator;
    const response = allocator.create(GhostResponse) catch return null;
    
    // For now, create a simple demo response
    // In real implementation, this would call into GhostLLM inference engine
    const demo_text = "This is a demo response from GhostLLM FFI bindings. ";
    const prompt_echo = std.mem.span(prompt);
    
    // Calculate total response length
    const total_len = demo_text.len + prompt_echo.len + 50; // Extra space for formatting
    
    const response_text = allocator.allocSentinel(u8, total_len, 0) catch {
        allocator.destroy(response);
        return null;
    };
    
    // Format response
    const formatted = std.fmt.bufPrintZ(
        response_text, 
        "Demo Response: {s} (Prompt was: {s})",
        .{ demo_text, prompt_echo }
    ) catch {
        allocator.free(response_text);
        allocator.destroy(response);
        return null;
    };
    
    // Simulate streaming if callback provided
    if (callback) |cb| {
        // Stream tokens one by one - using null-terminated strings
        const words = [_][*:0]const u8{ "Demo", " Response:", " Processing", " complete" };
        for (words) |word| {
            cb(word, std.mem.len(word));
            // In real implementation, this would be actual generation with timing
        }
    }
    
    response.* = GhostResponse{
        .text = response_text.ptr,
        .tokens_used = @intCast(formatted.len / 4), // Rough token estimate
        .error_code = 0,
        .allocator = allocator,
    };
    
    return response;
}

// Configuration functions
export fn ghost_set_max_tokens(ctx: ?*GhostContext, max_tokens: u32) i32 {
    if (ctx == null or !ctx.?.initialized) {
        return -1;
    }
    
    if (max_tokens == 0 or max_tokens > 32768) {
        return -2; // Invalid token count
    }
    
    ctx.?.max_tokens = max_tokens;
    return 0;
}

export fn ghost_set_temperature(ctx: ?*GhostContext, temperature: f32) i32 {
    if (ctx == null or !ctx.?.initialized) {
        return -1;
    }
    
    if (temperature < 0.0 or temperature > 2.0) {
        return -2; // Invalid temperature
    }
    
    ctx.?.temperature = temperature;
    return 0;
}

// Response accessor functions
export fn ghost_response_text(response: ?*const GhostResponse) [*:0]const u8 {
    if (response == null) {
        return "";
    }
    return response.?.text;
}

export fn ghost_response_tokens_used(response: ?*const GhostResponse) u32 {
    if (response == null) {
        return 0;
    }
    return response.?.tokens_used;
}

export fn ghost_response_error_code(response: ?*const GhostResponse) i32 {
    if (response == null) {
        return -1;
    }
    return response.?.error_code;
}

// Cleanup functions
export fn ghost_free_context(ctx: ?*GhostContext) void {
    if (ctx == null) {
        return;
    }
    
    const context = ctx.?;
    const allocator = context.allocator;
    
    // Free the owned model path
    const path_slice = std.mem.span(context.model_path);
    allocator.free(path_slice);
    
    allocator.destroy(context);
}

export fn ghost_free_response(response: ?*GhostResponse) void {
    if (response == null) {
        return;
    }
    
    const resp = response.?;
    const allocator = resp.allocator;
    
    // Free the response text
    const text_slice = std.mem.span(resp.text);
    allocator.free(text_slice);
    
    allocator.destroy(resp);
}

// Test functions for debugging
export fn ghost_test_basic() i32 {
    const ctx = ghost_init("test_model.gguf");
    if (ctx == null) {
        return -1;
    }
    defer ghost_free_context(ctx);
    
    const response = ghost_generate(ctx, "Hello, world!", null);
    if (response == null) {
        return -2;
    }
    defer ghost_free_response(response);
    
    return 0;
}

// Version info
export fn ghost_version() [*:0]const u8 {
    return "GhostLLM FFI 0.1.0";
}