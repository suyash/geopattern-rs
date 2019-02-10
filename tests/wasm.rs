#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use geopattern::generate_minified_svg_string;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn check_generated_string() {
    assert_eq!(generate_minified_svg_string("a") + "\n", include_str!("a.svg"));
}
