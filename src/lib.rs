pub mod color;
pub mod geo_pattern;
pub mod patterns;

pub use crate::geo_pattern::GeoPattern;

pub fn generate(s: &str) -> GeoPattern {
    GeoPattern::new(s).build().unwrap()
}
