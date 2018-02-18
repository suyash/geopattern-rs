extern crate base64;
extern crate sha1;
extern crate svg;

pub mod color;
pub mod geo_pattern;
pub mod patterns;

pub use geo_pattern::GeoPattern;

pub fn generate(s: &str) -> GeoPattern {
    GeoPattern::new(s).build().unwrap()
}
