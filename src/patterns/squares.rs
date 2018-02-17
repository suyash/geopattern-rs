use svg::Document;
use svg::node::element::{Rectangle};

use super::{
    STROKE_COLOR,
    STROKE_OPACITY,
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct Squares<'a> {
    hash: &'a str,
}

impl<'a> Squares<'a> {
    pub fn new(hash: &'a str) -> Self {
        Squares{
            hash: hash,
        }
    }
}

impl<'a> Pattern for Squares<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 10.0, 60.0);

        let mut svg = svg
            .set("width", s * 6.0)
            .set("height", s * 6.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;

                let v = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(v as f64), fill_color(v));

                let r = Rectangle::new()
                    .set("x", (x as f64) * s)
                    .set("y", (y as f64) * s)
                    .set("width", s)
                    .set("height", s)
                    .set("fill", f)
                    .set("fill-opacity", o)
                    .set("stroke", STROKE_COLOR)
                    .set("stroke-opacity", STROKE_OPACITY);

                svg = svg.add(r);
            }
        }

        Ok(svg)
    }
}
