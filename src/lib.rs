#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub mod color;
pub mod geo_pattern;
pub mod patterns;

pub use crate::geo_pattern::GeoPattern;

pub fn generate(s: &str) -> GeoPattern {
    GeoPattern::new(s).build().unwrap()
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn generate_minified_svg_string(s: &str) -> String {
    GeoPattern::new(s).build().unwrap().to_minified_svg().unwrap()
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn generate_base64_svg_string(s: &str) -> String {
    GeoPattern::new(s).build().unwrap().to_base64().unwrap()
}
