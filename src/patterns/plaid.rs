use svg::Document;
use svg::node::element::{Rectangle};

use super::{
    fill_color,
    opacity,
    Pattern,
    PatternError,
};

pub struct Plaid<'a> {
    hash: &'a str,
}

impl<'a> Plaid<'a> {
    pub fn new(hash: &'a str) -> Self {
        Plaid{
            hash: hash,
        }
    }
}

impl<'a> Pattern for Plaid<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let (mut w, mut h) = (0.0, 0.0);

        let mut svg = svg;

        for i in 1..19 {
            let j = 2 * (i - 1);

            let s = u8::from_str_radix(&self.hash[j..j+1], 16)? as f64;
            h += s + 5.0;

            let val = u8::from_str_radix(&self.hash[j+1..j+2], 16)? as f64;
            let (o, f) = (opacity(val), fill_color(val as u8));

            let sh = val + 5.0;

            let r = Rectangle::new()
                .set("x", 0)
                .set("y", h)
                .set("width", "100%")
                .set("height", sh)
                .set("opacity", o)
                .set("fill", f);

            svg = svg.add(r);

            h += sh;
        }

        for i in 1..19 {
            let j = 2 * (i - 1);

            let s = u8::from_str_radix(&self.hash[j..j+1], 16)? as f64;
            w += s + 5.0;

            let val = u8::from_str_radix(&self.hash[j+1..j+2], 16)? as f64;
            let (o, f) = (opacity(val), fill_color(val as u8));

            let sw = val + 5.0;

            let r = Rectangle::new()
                .set("x", w)
                .set("y", 0)
                .set("width", sw)
                .set("height", "100%")
                .set("opacity", o)
                .set("fill", f);

            svg = svg.add(r);

            w += sw;
        }

        svg = svg
            .set("width", w)
            .set("height", h);

        Ok(svg)
    }
}
