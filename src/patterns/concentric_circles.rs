use svg::Document;
use svg::node::element::{Circle};

use super::{
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct ConcentricCircles<'a> {
    hash: &'a str,
}

impl<'a> ConcentricCircles<'a> {
    pub fn new(hash: &'a str) -> Self {
        ConcentricCircles{
            hash: hash,
        }
    }
}

impl<'a> Pattern for ConcentricCircles<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = u8::from_str_radix(&self.hash[0..1], 16)?;
        let rs = rescale(s as f64, 0.0, 15.0, 10.0, 60.0);
        let sw = rs / 5.0;

        let mut svg = svg
            .set("width", (rs + sw) * 6.0)
            .set("height", (rs + sw) * 6.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;
                let v = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(v as f64), fill_color(v));

                let (cx, cy) = ((x as f64) * rs + (x as f64) * sw + (rs + sw) / 2.0, (y as f64) * rs + (y as f64) * sw + (rs + sw) / 2.0);

                let c = Circle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", rs / 2.0)
                    .set("stroke", f)
                    .set("style", format!("opacity:{}; stroke-width:0x{:X};", o, (sw as i64)));

                svg = svg.add(c);

                let v = u8::from_str_radix(&self.hash[39-i..40-i], 16)?;
                let (o, f) = (opacity(v as f64), fill_color(v));

                let c = Circle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", rs / 4.0)
                    .set("fill", f)
                    .set("fill-opacity", o);

                svg = svg.add(c);
            }
        }

        Ok(svg)
    }
}
