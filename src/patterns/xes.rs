use svg::Document;
use svg::node::element::{Rectangle, Group};

use super::{
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct Xes<'a> {
    hash: &'a str,
}

impl<'a> Xes<'a> {
    pub fn new(hash: &'a str) -> Self {
        Xes{
            hash: hash,
        }
    }

    fn plus(s: f64) -> (Rectangle, Rectangle) {
        (
            Rectangle::new().set("x", s).set("y", 0).set("width", s).set("height", s * 3.0),
            Rectangle::new().set("x", 0).set("y", s).set("width", s * 3.0).set("height", s),
        )
    }
}

impl<'a> Pattern for Xes<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 10.0, 25.0);
        let xs = s * 3.0 * 0.943;
        let shape = Self::plus(s);

        let mut svg = svg
            .set("width", xs * 3.0)
            .set("height", xs * 3.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;

                let v = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(v as f64), fill_color(v));

                let dy = match x % 2 {
                    1 => (y as f64) * xs - xs * 0.5 + xs / 4.0,
                    _ => (y as f64) * xs - xs * 0.5,
                };

                let g = Group::new()
                    .set("fill", f)
                    .set("style", format!("opacity:{};", o))
                    .add(shape.0.clone())
                    .add(shape.1.clone());

                svg = svg.add(
                    g.clone()
                        .set("transform", format!("translate({}, {}) rotate(45, {}, {})", (x as f64) * xs / 2.0 - xs / 2.0, dy - (y as f64) * xs / 2.0, xs / 2.0, xs / 2.0))
                );

                if x == 0 {
                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {}) rotate(45, {}, {})", 6.0 * xs / 2.0 - xs / 2.0, dy - (y as f64) * xs / 2.0, xs / 2.0, xs / 2.0))
                    );
                }

                if y == 0 {
                    let dy = match x % 2 {
                        1 => 6.0 * xs - xs * 0.5,
                        _ => 6.0 * xs - xs * 0.5 + xs * 0.25,
                    };

                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {}) rotate(45, {}, {})", (x as f64) * xs / 2.0 - xs / 2.0, dy - (y as f64) * xs / 2.0, xs / 2.0, xs / 2.0))
                    );
                }

                if y == 5 {
                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {}) rotate(45, {}, {})", (x as f64) * xs / 2.0 - xs / 2.0, dy - 11.0 * xs / 2.0, xs / 2.0, xs / 2.0))
                    );
                }

                if x == 0 && y == 0 {
                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {}) rotate(45, {}, {})", 6.0 * xs / 2.0 - xs / 2.0, dy - 6.0 * xs / 2.0, xs / 2.0, xs / 2.0))
                    );
                }
            }
        }

        Ok(svg)
    }
}
