use svg::Document;
use svg::node::element::{Group, Rectangle};

use super::{
    STROKE_COLOR,
    STROKE_OPACITY,
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct PlusSigns<'a> {
    hash: &'a str,
}

impl<'a> PlusSigns<'a> {
    pub fn new(hash: &'a str) -> Self {
        PlusSigns{
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

impl<'a> Pattern for PlusSigns<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 10.0, 25.0);
        let ps = s * 3.0;
        let p = Self::plus(s);

        let mut svg = svg
            .set("width", s * 12.0)
            .set("height", s * 12.0);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;
                let val = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(val as f64), fill_color(val));

                let dx = (y % 2) as f64;

                let g = Group::new()
                    .set("fill", f)
                    .set("stroke", STROKE_COLOR)
                    .set("stroke-opacity", STROKE_OPACITY)
                    .set("style", format!("fill-opacity:{};", o))
                    .add(p.0.clone())
                    .add(p.1.clone());

                svg = svg.add(
                    g.clone()
                        .set("transform", format!("translate({}, {})", (x as f64) * (ps - s) + dx * s - s, (y as f64) * (ps - s) - ps / 2.0)),
                );

                if x == 0 {
                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {})", 4.0 * ps - (x as f64) * s + dx * s - s, (y as f64) * (ps - s) - ps / 2.0)),
                    );
                }

                if y == 0 {
                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {})", (x as f64) * (ps - s) + dx * s - s, 4.0 * ps - (y as f64) * s - ps / 2.0)),
                    );
                }

                if x == 0 && y == 0 {
                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {})", 4.0 * ps - (x as f64) * s + dx * s - s, 4.0 * ps - (y as f64) * s - ps / 2.0)),
                    );
                }
            }
        }

        Ok(svg)
    }
}
