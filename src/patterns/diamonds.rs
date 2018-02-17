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

pub struct Diamonds<'a> {
    hash: &'a str,
}

impl<'a> Diamonds<'a> {
    pub fn new(hash: &'a str) -> Self {
        Diamonds{
            hash: hash,
        }
    }
}

impl<'a> Pattern for Diamonds<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let w = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 10.0, 50.0);
        let h = rescale(u8::from_str_radix(&self.hash[1..2], 16)? as f64, 0.0, 15.0, 10.0, 50.0);
        let p = format!("{},0,{},{},{},{},0,{}", w / 2.0, w, h / 2.0, w / 2.0, h, h / 2.0);
        let points = p.as_str();

        let mut svg = svg
            .set("width", w * 6.0)
            .set("height", h * 3.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;
                let val = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(val as f64), fill_color(val));

                let dx = match y % 2 {
                    1 => w / 2.0,
                    _ => 0.0,
                };

                let p = Polyline::new()
                    .set("points", points)
                    .set("fill", f)
                    .set("fill-opacity", o)
                    .set("stroke", STROKE_COLOR)
                    .set("stroke-opacity", STROKE_OPACITY);

                svg = svg.add(
                    p.clone()
                        .set("transform", format!("translate({}, {})", dx + (x as f64) * w - (w / 2.0), (h / 2.0) * (y as f64) - (h / 2.0)))
                );

                if x == 0 {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", dx + 6.0 * w - (w / 2.0), (h / 2.0) * (y as f64) - (h / 2.0)))
                    );
                }

                if y == 0 {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", dx + (x as f64) * w - (w / 2.0), h / 2.0 * 6.0 - h / 2.0))
                    );
                }

                if x == 0 && y == 0 {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", dx + 6.0 * w - (w / 2.0), (h / 2.0) * 6.0 - (h / 2.0)))
                    );
                }
            }
        }

        Ok(svg)
    }
}
