#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Error types for GhostLLM operations
#[derive(Debug, Clone)]
pub enum GhostLLMError {
    InitializationFailed,
    InvalidModelPath,
    GenerationFailed,
    InvalidConfiguration,
    NullPointer,
}

impl std::fmt::Display for GhostLLMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GhostLLMError::InitializationFailed => write!(f, "Failed to initialize GhostLLM"),
            GhostLLMError::InvalidModelPath => write!(f, "Invalid model path"),
            GhostLLMError::GenerationFailed => write!(f, "Text generation failed"),
            GhostLLMError::InvalidConfiguration => write!(f, "Invalid configuration parameter"),
            GhostLLMError::NullPointer => write!(f, "Null pointer encountered"),
        }
    }
}

impl std::error::Error for GhostLLMError {}

/// Configuration for GhostLLM
#[derive(Debug, Clone)]
pub struct GhostConfig {
    pub max_tokens: u32,
    pub temperature: f32,
}

impl Default for GhostConfig {
    fn default() -> Self {
        Self {
            max_tokens: 2048,
            temperature: 0.7,
        }
    }
}

/// Response from GhostLLM generation
#[derive(Debug, Clone)]
pub struct GhostGenerationResponse {
    pub text: String,
    pub tokens_used: u32,
}

/// Callback trait for streaming generation
pub trait StreamingCallback: Send + Sync {
    fn on_token(&mut self, token: &str);
}

/// Simple callback implementation that collects tokens
#[derive(Default)]
pub struct TokenCollector {
    pub tokens: Vec<String>,
}

impl StreamingCallback for TokenCollector {
    fn on_token(&mut self, token: &str) {
        self.tokens.push(token.to_string());
    }
}

/// Main GhostLLM interface
pub struct GhostLLM {
    context: *mut ghost_context_t,
    config: GhostConfig,
}

// Global storage for callbacks (needed for C FFI)
lazy_static::lazy_static! {
    static ref CALLBACK_STORAGE: Arc<Mutex<HashMap<usize, Box<dyn StreamingCallback>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

// C callback wrapper
extern "C" fn stream_callback_wrapper(text: *const c_char, len: usize) {
    if text.is_null() {
        return;
    }
    
    unsafe {
        let slice = std::slice::from_raw_parts(text as *const u8, len);
        if let Ok(token_str) = std::str::from_utf8(slice) {
            // For simplicity in this demo, we'll print the token
            // In a real implementation, you'd need a way to route this to the correct callback
            print!("{}", token_str);
        }
    }
}

impl GhostLLM {
    /// Create a new GhostLLM instance
    pub fn new(model_path: &str) -> Result<Self, GhostLLMError> {
        let c_path = CString::new(model_path)
            .map_err(|_| GhostLLMError::InvalidModelPath)?;
        
        unsafe {
            let context = ghost_init(c_path.as_ptr());
            if context.is_null() {
                return Err(GhostLLMError::InitializationFailed);
            }
            
            Ok(Self {
                context,
                config: GhostConfig::default(),
            })
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(model_path: &str, config: GhostConfig) -> Result<Self, GhostLLMError> {
        let mut llm = Self::new(model_path)?;
        llm.set_config(config)?;
        Ok(llm)
    }
    
    /// Update configuration
    pub fn set_config(&mut self, config: GhostConfig) -> Result<(), GhostLLMError> {
        unsafe {
            let result = ghost_set_max_tokens(self.context, config.max_tokens);
            if result != 0 {
                return Err(GhostLLMError::InvalidConfiguration);
            }
            
            let result = ghost_set_temperature(self.context, config.temperature);
            if result != 0 {
                return Err(GhostLLMError::InvalidConfiguration);
            }
        }
        
        self.config = config;
        Ok(())
    }
    
    /// Generate text without streaming
    pub fn generate(&self, prompt: &str) -> Result<GhostGenerationResponse, GhostLLMError> {
        let c_prompt = CString::new(prompt)
            .map_err(|_| GhostLLMError::GenerationFailed)?;
        
        unsafe {
            let response = ghost_generate(
                self.context,
                c_prompt.as_ptr(),
                None,
            );
            
            if response.is_null() {
                return Err(GhostLLMError::GenerationFailed);
            }
            
            let error_code = ghost_response_error_code(response);
            if error_code != 0 {
                ghost_free_response(response);
                return Err(GhostLLMError::GenerationFailed);
            }
            
            let text_ptr = ghost_response_text(response);
            let text = if text_ptr.is_null() {
                String::new()
            } else {
                CStr::from_ptr(text_ptr)
                    .to_string_lossy()
                    .into_owned()
            };
            
            let tokens_used = ghost_response_tokens_used(response);
            
            ghost_free_response(response);
            
            Ok(GhostGenerationResponse {
                text,
                tokens_used,
            })
        }
    }
    
    /// Generate text with streaming callback
    pub fn generate_stream<F>(&self, prompt: &str, mut callback: F) -> Result<GhostGenerationResponse, GhostLLMError>
    where
        F: FnMut(&str) + Send + 'static,
    {
        let c_prompt = CString::new(prompt)
            .map_err(|_| GhostLLMError::GenerationFailed)?;
        
        unsafe {
            let response = ghost_generate(
                self.context,
                c_prompt.as_ptr(),
                Some(stream_callback_wrapper),
            );
            
            if response.is_null() {
                return Err(GhostLLMError::GenerationFailed);
            }
            
            let error_code = ghost_response_error_code(response);
            if error_code != 0 {
                ghost_free_response(response);
                return Err(GhostLLMError::GenerationFailed);
            }
            
            let text_ptr = ghost_response_text(response);
            let text = if text_ptr.is_null() {
                String::new()
            } else {
                CStr::from_ptr(text_ptr)
                    .to_string_lossy()
                    .into_owned()
            };
            
            let tokens_used = ghost_response_tokens_used(response);
            
            ghost_free_response(response);
            
            Ok(GhostGenerationResponse {
                text,
                tokens_used,
            })
        }
    }
    
    /// Get current configuration
    pub fn config(&self) -> &GhostConfig {
        &self.config
    }
    
    /// Test the connection and basic functionality
    pub fn test_connection(&self) -> Result<(), GhostLLMError> {
        let response = self.generate("test")?;
        if response.text.is_empty() {
            return Err(GhostLLMError::GenerationFailed);
        }
        Ok(())
    }
}

impl Drop for GhostLLM {
    fn drop(&mut self) {
        unsafe {
            if !self.context.is_null() {
                ghost_free_context(self.context);
                self.context = std::ptr::null_mut();
            }
        }
    }
}

// Safety: GhostLLM can be safely sent between threads
unsafe impl Send for GhostLLM {}
unsafe impl Sync for GhostLLM {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ghostllm_creation() {
        let result = GhostLLM::new("test_model.gguf");
        // In stub mode, this should succeed
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_basic_generation() {
        let llm = GhostLLM::new("test_model.gguf").expect("Failed to create LLM");
        let response = llm.generate("Hello, world!");
        assert!(response.is_ok());
        
        let resp = response.unwrap();
        assert!(!resp.text.is_empty());
        assert!(resp.tokens_used > 0);
    }
    
    #[test]
    fn test_config_update() {
        let mut llm = GhostLLM::new("test_model.gguf").expect("Failed to create LLM");
        
        let config = GhostConfig {
            max_tokens: 1024,
            temperature: 0.5,
        };
        
        let result = llm.set_config(config.clone());
        assert!(result.is_ok());
        assert_eq!(llm.config().max_tokens, config.max_tokens);
        assert_eq!(llm.config().temperature, config.temperature);
    }
    
    #[test]
    fn test_invalid_config() {
        let mut llm = GhostLLM::new("test_model.gguf").expect("Failed to create LLM");
        
        let invalid_config = GhostConfig {
            max_tokens: 0, // Invalid
            temperature: 3.0, // Invalid
        };
        
        let result = llm.set_config(invalid_config);
        assert!(result.is_err());
    }
}

// Re-export for convenience
pub use self::GhostLLM as LLM;
pub use self::GhostConfig as Config;
pub use self::GhostGenerationResponse as Response;