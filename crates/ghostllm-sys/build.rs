use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Check if Zig compiler is available
    let zig_check = Command::new("zig")
        .arg("version")
        .output();
    
    match zig_check {
        Ok(output) => {
            if !output.status.success() {
                panic!("Zig compiler found but not working properly");
            }
            println!("cargo:warning=Using Zig version: {}", String::from_utf8_lossy(&output.stdout).trim());
        }
        Err(_) => {
            println!("cargo:warning=Zig compiler not found. Please install Zig 0.11+ from https://ziglang.org");
            println!("cargo:warning=Falling back to C implementation stub");
            
            // Create a simple C stub for now
            let stub_path = format!("{}/ghostllm_stub.c", out_dir);
            std::fs::write(&stub_path, r#"
#include "ghostllm.h"
#include <stdlib.h>
#include <string.h>

typedef struct {
    char* model_path;
    int max_tokens;
    float temperature;
} ghost_context_t;

typedef struct {
    char* text;
    int tokens_used;
    int error_code;
} ghost_response_t;

ghost_context_t* ghost_init(const char* model_path) {
    ghost_context_t* ctx = malloc(sizeof(ghost_context_t));
    if (!ctx) return NULL;
    
    ctx->model_path = strdup(model_path);
    ctx->max_tokens = 2048;
    ctx->temperature = 0.7f;
    
    return ctx;
}

ghost_response_t* ghost_generate(ghost_context_t* ctx, const char* prompt, void (*callback)(const char*, size_t)) {
    ghost_response_t* response = malloc(sizeof(ghost_response_t));
    if (!response) return NULL;
    
    // Simple stub response
    response->text = strdup("This is a stub response. Install Zig and GhostLLM for real AI inference.");
    response->tokens_used = 15;
    response->error_code = 0;
    
    if (callback) {
        callback("Stub ", 5);
        callback("response", 8);
    }
    
    return response;
}

void ghost_free_context(ghost_context_t* ctx) {
    if (ctx) {
        free(ctx->model_path);
        free(ctx);
    }
}

void ghost_free_response(ghost_response_t* response) {
    if (response) {
        free(response->text);
        free(response);
    }
}
"#).expect("Failed to write stub file");
            
            // Compile the stub
            cc::Build::new()
                .file(&stub_path)
                .include("src")
                .compile("ghostllm_stub");
                
            println!("cargo:rustc-link-lib=static=ghostllm_stub");
            
            // Generate bindings for the stub
            let bindings = bindgen::Builder::default()
                .header("src/ghostllm.h")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                .generate()
                .expect("Unable to generate bindings");

            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
                
            return;
        }
    }
    
    // Compile Zig code to static library
    let zig_src = format!("{}/src/ghostllm.zig", src_dir);
    let lib_path = format!("{}/libghostllm.a", out_dir);
    
    let zig_build = Command::new("zig")
        .args(&[
            "build-lib",
            &zig_src,
            "-O", "ReleaseFast",
            &format!("-femit-bin={}", lib_path),
            "-target", "native-native-gnu",
            "-mcpu=native",
            "-lc",
        ])
        .status()
        .expect("Failed to execute Zig compiler");

    if !zig_build.success() {
        panic!("Zig compilation failed");
    }

    // Link the compiled library
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ghostllm");
    println!("cargo:rustc-link-lib=c");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src/ghostllm.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Rebuild if source files change
    println!("cargo:rerun-if-changed=src/ghostllm.zig");
    println!("cargo:rerun-if-changed=src/ghostllm.h");
}