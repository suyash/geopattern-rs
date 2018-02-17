use svg::Document;
use svg::node::element::{Rectangle};

use super::{
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct NestedSquares<'a> {
    hash: &'a str,
}

impl<'a> NestedSquares<'a> {
    pub fn new(hash: &'a str) -> Self {
        NestedSquares{
            hash: hash,
        }
    }
}

impl<'a> Pattern for NestedSquares<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 4.0, 12.0);
        let ss = s * 7.0;

        let mut svg = svg
            .set("width", (s + ss) * 6.0 + s * 6.0)
            .set("height", (s + ss) * 6.0 + s * 6.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;

                let v = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(v as f64), fill_color(v));

                let outer = Rectangle::new()
                    .set("x", (x as f64) * ss + (x as f64) * s * 2.0 + s / 2.0)
                    .set("y", (y as f64) * ss + (y as f64) * s * 2.0 + s / 2.0)
                    .set("width", ss)
                    .set("height", ss)
                    .set("fill", "none")
                    .set("stroke", f)
                    .set("style", format!("opacity:{};stroke-width:0x{:X};", o, s as u64));

                svg = svg.add(outer);

                let v = u8::from_str_radix(&self.hash[39-i..40-i], 16)?;
                let (o, f) = (opacity(v as f64), fill_color(v));

                let inner = Rectangle::new()
                    .set("x", (x as f64) * ss + (x as f64) * s * 2.0 + s / 2.0 + s * 2.0)
                    .set("y", (y as f64) * ss + (y as f64) * s * 2.0 + s / 2.0 + s * 2.0)
                    .set("width", s * 3.0)
                    .set("height", s * 3.0)
                    .set("fill", "none")
                    .set("stroke", f)
                    .set("style", format!("opacity:{};stroke-width:0x{:X};", o, s as u64));

                svg = svg.add(inner);
            }
        }

        Ok(svg)
    }
}
