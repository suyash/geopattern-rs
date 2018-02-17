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

pub struct Octagons<'a> {
    hash: &'a str,
}

impl<'a> Octagons<'a> {
    pub fn new(hash: &'a str) -> Self {
        Octagons{
            hash: hash,
        }
    }
}

impl<'a> Pattern for Octagons<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 10.0, 60.0);
        let c = 0.33 * s;
        let o = format!("{},0,{},0,{},{},{},{},{},{},{},{},0,{},0,{},{},0", c, s - c, s, c, s, s - c, s - c, s, c, s, s - c, c, c);
        let points = o.as_str();

        let mut svg = svg
            .set("width", s * 6.0)
            .set("height", s * 6.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;
                let val = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(val as f64), fill_color(val));

                let p = Polyline::new()
                    .set("points", points)
                    .set("fill", f)
                    .set("fill-opacity", o)
                    .set("stroke", STROKE_COLOR)
                    .set("stroke-opacity", STROKE_OPACITY)
                    .set("transform", format!("translate({}, {})", (x as f64) * s, (y as f64) * s));

                svg = svg.add(p);
            }
        }

        Ok(svg)
    }
}
