#ifndef GHOSTLLM_H
#define GHOSTLLM_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
typedef struct ghost_context_t ghost_context_t;
typedef struct ghost_response_t ghost_response_t;

// Callback function type for streaming
typedef void (*ghost_stream_callback_t)(const char* token, size_t len);

// Core functions
ghost_context_t* ghost_init(const char* model_path);
ghost_response_t* ghost_generate(ghost_context_t* ctx, const char* prompt, ghost_stream_callback_t callback);
void ghost_free_context(ghost_context_t* ctx);
void ghost_free_response(ghost_response_t* response);

// Configuration functions
int ghost_set_max_tokens(ghost_context_t* ctx, uint32_t max_tokens);
int ghost_set_temperature(ghost_context_t* ctx, float temperature);

// Response accessors
const char* ghost_response_text(const ghost_response_t* response);
uint32_t ghost_response_tokens_used(const ghost_response_t* response);
int32_t ghost_response_error_code(const ghost_response_t* response);

#ifdef __cplusplus
}
#endif

#endif // GHOSTLLM_H