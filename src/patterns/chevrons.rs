use svg::Document;
use svg::node::element::{Group, Polyline};

use super::{
    STROKE_COLOR,
    STROKE_OPACITY,
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct Chevrons<'a> {
    hash: &'a str,
}

impl<'a> Chevrons<'a> {
    pub fn new(hash: &'a str) -> Self {
        Chevrons{
            hash: hash,
        }
    }

    fn chevron(w: f64, h: f64) -> (Polyline, Polyline) {
        let e = h * 0.66;

        (
            Polyline::new().set("points", format!("0,0,{},{},{},{},0,{},0,0", w / 2.0, h - e, w / 2.0, h, e)),
            Polyline::new().set("points", format!("{},{},{},0,{},{},{},{},{},{}",  w / 2.0, h - e, w, w, e, w / 2.0, h, w / 2.0, h - e)),
        )
    }
}

impl<'a> Pattern for Chevrons<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let cw = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 30.0, 80.0);
        let c = Self::chevron(cw, cw);

        let mut svg = svg
            .set("width", cw * 6.0)
            .set("height", cw * 6.0 * 0.66);

        for y in 0..6 {
            for x in 0..6 {
                let i = y * 6 + x;
                let val = u8::from_str_radix(&self.hash[i..i+1], 16)?;
                let (o, f) = (opacity(val as f64), fill_color(val));

                let g = Group::new()
                    .set("fill", f)
                    .set("fill-opacity", o)
                    .set("stroke", STROKE_COLOR)
                    .set("stroke-opacity", STROKE_OPACITY)
                    .set("stroke-width", 1)
                    .add(c.0.clone())
                    .add(c.1.clone());

                svg = svg.add(
                    g.clone()
                        .set("transform", format!("translate({}, {})", (x as f64) * cw, (y as f64) * cw * 0.66 - cw / 2.0))
                );

                if y == 0 {
                    svg = svg.add(
                        g.clone()
                            .set("transform", format!("translate({}, {})", (x as f64) * cw, 6.0 * cw * 0.66 - cw / 2.0))
                    );
                }
            }
        }

        Ok(svg)
    }
}
