use svg::Document;
use svg::node::element::{Circle};

use super::{
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct OverlappingRings<'a> {
    hash: &'a str,
}

impl<'a> OverlappingRings<'a> {
    pub fn new(hash: &'a str) -> Self {
        OverlappingRings{
            hash: hash,
        }
    }
}

impl<'a> Pattern for OverlappingRings<'a> {
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

                let c = Circle::new()
                    .set("cx", (x as f64) * s)
                    .set("cy", (y as f64) * s)
                    .set("r", s - s / 8.0)
                    .set("fill", "none")
                    .set("stroke", f)
                    .set("style", format!("opacity:{};stroke-width:0x{:X};", o, (s / 4.0) as u64));

                svg = svg.add(c);

                if x == 0 {
                    let c = Circle::new()
                        .set("cx", 6.0 * s)
                        .set("cy", (y as f64) * s)
                        .set("r", s - s / 8.0)
                        .set("fill", "none")
                        .set("stroke", f)
                        .set("style", format!("opacity:{};stroke-width:0x{:X};", o, (s / 4.0) as u64));

                    svg = svg.add(c);
                }

                if y == 0 {
                    let c = Circle::new()
                        .set("cx", (x as f64) * s)
                        .set("cy", 6.0 * s)
                        .set("r", s - s / 8.0)
                        .set("fill", "none")
                        .set("stroke", f)
                        .set("style", format!("opacity:{};stroke-width:0x{:X};", o, (s / 4.0) as u64));

                    svg = svg.add(c);
                }

                if x == 0 && y == 0 {
                    let c = Circle::new()
                        .set("cx", 6.0 * s)
                        .set("cy", 6.0 * s)
                        .set("r", s - s / 8.0)
                        .set("fill", "none")
                        .set("stroke", f)
                        .set("style", format!("opacity:{};stroke-width:0x{:X};", o, (s / 4.0) as u64));

                    svg = svg.add(c);
                }
            }
        }

        Ok(svg)
    }
}
