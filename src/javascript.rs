use wasm_bindgen::prelude::*;
use crate::parse;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn convert_ilcd(json: String) -> String {
    let epd = parse::parse_ilcd(json);
    serde_json::to_string(&epd).unwrap()
}
