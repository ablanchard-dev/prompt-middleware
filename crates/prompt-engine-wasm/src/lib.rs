//! WebAssembly bindings for `prompt-engine`.
//!
//! Exposes a single [`optimize`] function callable from JavaScript: it takes
//! the request as a JSON string and returns the response as a JSON string, so
//! the browser extension can optimize prompts entirely in-browser — no server,
//! nothing leaves the machine.

use prompt_engine::config::EngineConfig;
use prompt_engine::optimize_prompt;
use shared_types::api::OptimizeRequest;
use wasm_bindgen::prelude::*;

/// Optimize a prompt in WebAssembly.
///
/// `request_json` is a serialized `OptimizeRequest`; the return value is a
/// serialized `OptimizeResponse`. Returns a JS error for invalid JSON or for
/// empty/oversized input.
#[wasm_bindgen]
pub fn optimize(request_json: &str) -> Result<String, JsError> {
    let request: OptimizeRequest = serde_json::from_str(request_json)
        .map_err(|error| JsError::new(&format!("invalid request JSON: {error}")))?;

    let response = optimize_prompt(request, &EngineConfig::default())
        .map_err(|error| JsError::new(&error.to_string()))?;

    serde_json::to_string(&response)
        .map_err(|error| JsError::new(&format!("failed to serialize response: {error}")))
}
