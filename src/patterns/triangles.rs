use svg::Document;
use svg::node::element::{Polyline};

use super::{
    STROKE_COLOR,
    STROKE_OPACITY,
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct Triangles<'a> {
    hash: &'a str,
}

impl<'a> Triangles<'a> {
    pub fn new(hash: &'a str) -> Self {
        Triangles{
            hash: hash,
        }
    }
}

impl<'a> Pattern for Triangles<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 15.0, 80.0);

        let th = (3.0 as f64).sqrt() * s / 2.0;
        let t = format!("{},0,{},{},0,{},{},0", s / 2.0, s, th, th, s / 2.0);
        let points = t.as_str();

        let mut svg = svg
            .set("width", s * 3.0)
            .set("height", th * 6.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;

                let v = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(v as f64), fill_color(v));

                let rot = match (x, y) {
                    (x, y) if y % 2 == 0 && x % 2 == 0 => 180,
                    (x, y) if y % 2 != 0 && x % 2 != 0 => 180,
                    _ => 0,
                };

                let p = Polyline::new()
                    .set("points", points)
                    .set("fill", f)
                    .set("fill-opacity", o)
                    .set("stroke", STROKE_COLOR)
                    .set("stroke-opacity", STROKE_OPACITY);

                svg = svg.add(
                    p.clone()
                        .set("transform", format!("translate({}, {}) rotate({}, {}, {})", (x as f64) * s * 0.5 - s / 2.0, th * (y as f64), rot, s / 2.0, th / 2.0))
                );

                if x == 0 {
                    svg = svg.add(
                        p.set("transform", format!("translate({}, {}) rotate({}, {}, {})", 6.0 * s * 0.5 - s / 2.0, th * (y as f64), rot, s / 2.0, th / 2.0))
                    );
                }
            }
        }

        Ok(svg)
    }
}
