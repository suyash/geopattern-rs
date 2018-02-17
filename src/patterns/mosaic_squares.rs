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

pub struct MosaicSquares<'a> {
    hash: &'a str,
}

impl<'a> MosaicSquares<'a> {
    pub fn new(hash: &'a str) -> Self {
        MosaicSquares{
            hash: hash,
        }
    }

    fn draw_outer_tile(&self, mut svg: Document, x: f64, y: f64, s: f64, v: f64) -> Document {
        let t = format!("0,0,{},{},0,{},0,0", s, s, s);
        let points = t.as_str();

        let (o, f) = (opacity(v), fill_color(v as u8));

        let p = Polyline::new()
            .set("points", points)
            .set("fill", f)
            .set("fill-opacity", o)
            .set("stroke", STROKE_COLOR)
            .set("stroke-opacity", STROKE_OPACITY);

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(1, -1)", x, y + s)),
        );

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(-1, -1)", x + s * 2.0, y + s)),
        );

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(1, 1)", x, y + s)),
        );

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(-1, 1)", x + s * 2.0, y + s)),
        );

        svg
    }

    fn draw_inner_tile(&self, mut svg: Document, x: f64, y: f64, s: f64, v1: f64, v2: f64) -> Document {
        let t = format!("0,0,{},{},0,{},0,0", s, s, s);
        let points = t.as_str();

        let (o, f) = (opacity(v1), fill_color(v1 as u8));

        let p = Polyline::new()
            .set("points", points)
            .set("fill", f)
            .set("fill-opacity", o)
            .set("stroke", STROKE_COLOR)
            .set("stroke-opacity", STROKE_OPACITY);

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(-1, 1)", x + s, y)),
        );

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(1, -1)", x + s, y + s * 2.0)),
        );

        let (o, f) = (opacity(v2), fill_color(v2 as u8));

        let p = Polyline::new()
            .set("points", points)
            .set("fill", f)
            .set("fill-opacity", o)
            .set("stroke", STROKE_COLOR)
            .set("stroke-opacity", STROKE_OPACITY);

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(-1, -1)", x + s, y + s * 2.0)),
        );

        svg = svg.add(
            p.clone()
                .set("transform", format!("translate({}, {}) scale(1, 1)", x + s, y)),
        );

        svg
    }
}

impl<'a> Pattern for MosaicSquares<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let s = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 15.0, 50.0);

        let mut svg = svg
            .set("width", s * 8.0)
            .set("height", s * 8.0);

        for y in 0..4 {
            for x in 0..4 {
                let i = y * 4 + x;
                let v1 = u8::from_str_radix(&self.hash[i..i+1], 16)? as f64;
                let v2 = u8::from_str_radix(&self.hash[i+1..i+2], 16)? as f64;

                match x % 2 {
                    1 => {
                        match y % 2 {
                            1 => {
                                svg = self.draw_outer_tile(svg, (x as f64) * s * 2.0, (y as f64) * s * 2.0, s, v1);
                            },
                            _ => {
                                svg = self.draw_inner_tile(svg, (x as f64) * s * 2.0, (y as f64) * s * 2.0, s, v1, v2);
                            }
                        }
                    },
                    _ => {
                        match y % 2 {
                            1 => {
                                svg = self.draw_inner_tile(svg, (x as f64) * s * 2.0, (y as f64) * s * 2.0, s, v1, v2);
                            },
                            _ => {
                                svg = self.draw_outer_tile(svg, (x as f64) * s * 2.0, (y as f64) * s * 2.0, s, v1);
                            }
                        }
                    }
                }
            }
        }

        Ok(svg)
    }
}
