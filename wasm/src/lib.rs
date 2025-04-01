
use wasm_bindgen::prelude::*;

// Import web_sys for console access
use web_sys::console;

// Import submodules
mod matrix;

// Re-export our matrix implementation
pub use matrix::WasmMatrixEngine;

// Library initialization function
#[wasm_bindgen(start)]
pub fn init() {
    // Set up any panic hooks or other initialization
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Log successful initialization
    console::log_1(&JsValue::from_str("WASM Graphics Engine initialized"));
}

// Version information function
#[wasm_bindgen]
pub fn version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("WASM Graphics Engine v{}", version)
}