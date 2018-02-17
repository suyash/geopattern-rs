use svg::Document;
use svg::node::element::{Circle};

use super::{
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct OverlappingCircles<'a> {
    hash: &'a str,
}

impl<'a> OverlappingCircles<'a> {
    pub fn new(hash: &'a str) -> Self {
        OverlappingCircles{
            hash: hash,
        }
    }
}

impl<'a> Pattern for OverlappingCircles<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let d = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 25.0, 200.0);
        let r = d / 2.0;

        let mut svg = svg
            .set("width", r * 6.0)
            .set("height", r * 6.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;
                let val = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(val as f64), fill_color(val));

                let c = Circle::new()
                    .set("cx", (x as f64) * r)
                    .set("cy", (y as f64) * r)
                    .set("r", r)
                    .set("fill", f)
                    .set("style", format!("opacity:{};", o));

                svg = svg.add(c);

                if x == 0 {
                    let c = Circle::new()
                        .set("cx", 6.0 * r)
                        .set("cy", (y as f64) * r)
                        .set("r", r)
                        .set("fill", f)
                        .set("style", format!("opacity:{};", o));

                    svg = svg.add(c);
                }

                if y == 0 {
                    let c = Circle::new()
                        .set("cx", (x as f64) * r)
                        .set("cy", 6.0 * r)
                        .set("r", r)
                        .set("fill", f)
                        .set("style", format!("opacity:{};", o));

                    svg = svg.add(c);
                }

                if x == 0 && y == 0 {
                    let c = Circle::new()
                        .set("cx", 6.0 * r)
                        .set("cy", 6.0 * r)
                        .set("r", r)
                        .set("fill", f)
                        .set("style", format!("opacity:{};", o));

                    svg = svg.add(c);
                }
            }
        }

        Ok(svg)
    }
}
