mod utils;

use wasm_bindgen::prelude::wasm_bindgen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// import JS function
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// export greet function as JS function
#[wasm_bindgen]
pub fn greet(name: &str) {
    // use alert fn here
    alert(&format!("Hello, {}!", name));
}
