use std::f64;

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

pub struct Hexagons<'a> {
    hash: &'a str,
}

impl<'a> Hexagons<'a> {
    pub fn new(hash: &'a str) -> Self {
        Hexagons{
            hash: hash,
        }
    }
}

impl<'a> Pattern for Hexagons<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let l = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 8.0, 60.0);

        let hw = l * 2.0;
        let hh = l * (3.0 as f64).sqrt();

        let a = l / 2.0;
        let b = (f64::consts::PI / 3.0).sin() * l;
        let h = format!("0,{},{},0,{},0,{},{},{},{},{},{},0,{}", b, a, a + l, 2.0 * l, b, a + l, 2.0 * b, a, 2.0 * b, b);
        let points = h.as_str();

        let mut svg = svg
            .set("width", hw * 3.0 + l * 3.0)
            .set("height", hh * 3.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;
                let val = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(val as f64), fill_color(val));

                let dy = match x % 2 {
                    1 => (y as f64) * hh + hh / 2.0,
                    _ => (y as f64) * hh,
                };

                let p = Polyline::new()
                    .set("points", points)
                    .set("fill", f)
                    .set("fill-opacity", o)
                    .set("stroke", STROKE_COLOR)
                    .set("stroke-opacity", STROKE_OPACITY);

                svg = svg.add(
                    p.clone()
                        .set("transform", format!("translate({}, {})", (x as f64) * l * 1.5 - hw / 2.0, dy - hh / 2.0))
                );

                if x == 0 {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", 6.0 * l * 1.5 - hw / 2.0, dy - hh / 2.0))
                    );
                }

                if y == 0 {
                    let dy = match x % 2 {
                        1 => 6.0 * hh + hh / 2.0,
                        _ => 6.0 * hh,
                    };

                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", (x as f64) * l * 1.5 - hw / 2.0, dy - hh / 2.0))
                    );
                }

                if x == 0 && y == 0 {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", 6.0 * l * 1.5 - hw / 2.0, 5.0 * hh + hh / 2.0))
                    );
                }
            }
        }

        Ok(svg)
    }
}
