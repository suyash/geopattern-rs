use svg::Document;
use svg::node::element::{Rectangle, Polyline};

use super::{
    STROKE_COLOR,
    STROKE_OPACITY,
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct Tessellation<'a> {
    hash: &'a str,
}

impl<'a> Tessellation<'a> {
    pub fn new(hash: &'a str) -> Self {
        Tessellation{
            hash: hash,
        }
    }
}

impl<'a> Pattern for Tessellation<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let l = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 5.0, 40.0);

        let hexw = l * 2.0;
        let hexh = l * (3.0 as f64).sqrt();

        let th = l / 2.0 * (3.0 as f64).sqrt();
        let t = format!("0,0,{},{},0,{},0,0", th, l / 2.0, l);
        let points = t.as_str();

        let tilew = l * 3.0 + th * 2.0;
        let tileh = hexh * 2.0 + l * 2.0;

        let mut svg = svg
            .set("width", tilew)
            .set("height", tileh);

        for i in 0..20 {
            let v = u8::from_str_radix(&self.hash[i..i+1], 16)? as f64;
            let (o, f) = (opacity(v), fill_color(v as u8));

            let p = Polyline::new()
                .set("points", points)
                .set("fill", f)
                .set("fill-opacity", o)
                .set("stroke", STROKE_COLOR)
                .set("stroke-opacity", STROKE_OPACITY)
                .set("stroke-width", 1);

            let r = Rectangle::new()
                .set("fill", f)
                .set("fill-opacity", o)
                .set("stroke", STROKE_COLOR)
                .set("stroke-opacity", STROKE_OPACITY)
                .set("stroke-width", 1);

            match i {
                0 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", -l / 2.0)
                            .set("y", -l / 2.0)
                            .set("width", l)
                            .set("height", l)
                    );

                    svg = svg.add(
                        r.clone()
                            .set("x", tilew - l / 2.0)
                            .set("y", -l / 2.0)
                            .set("width", l)
                            .set("height", l)
                    );

                    svg = svg.add(
                        r.clone()
                            .set("x", -l / 2.0)
                            .set("y", tileh - l / 2.0)
                            .set("width", l)
                            .set("height", l)
                    );

                    svg = svg.add(
                        r.clone()
                            .set("x", tilew - l / 2.0)
                            .set("y", tileh - l / 2.0)
                            .set("width", l)
                            .set("height", l)
                    );
                },
                1 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", hexw / 2.0 + th)
                            .set("y", hexh / 2.0)
                            .set("width", l)
                            .set("height", l)
                    );
                },
                2 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", -l / 2.0)
                            .set("y", tileh / 2.0 - l / 2.0)
                            .set("width", l)
                            .set("height", l)
                    );

                    svg = svg.add(
                        r.clone()
                            .set("x", tilew - l / 2.0)
                            .set("y", tileh / 2.0 - l / 2.0)
                            .set("width", l)
                            .set("height", l)
                    );
                },
                3 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", hexw / 2.0 + th)
                            .set("y", hexh * 1.5 + l)
                            .set("width", l)
                            .set("height", l)
                    );
                },
                4 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) rotate(0, {}, {})", l / 2.0, -l / 2.0, l / 2.0, th / 2.0))
                    );

                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) rotate(0, {}, {}) scale(1, -1)", l / 2.0, tileh + l / 2.0, l / 2.0, th / 2.0))
                    );
                },
                5 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) rotate(0, {}, {}) scale(-1, 1)", tilew - l / 2.0, -l / 2.0, l / 2.0, th / 2.0))
                    );

                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) rotate(0, {}, {}) scale(-1, -1)", tilew - l / 2.0, tileh + l / 2.0, l / 2.0, th / 2.0))
                    );
                },
                6 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", tilew / 2.0 + l / 2.0, hexh / 2.0))
                    );
                },
                7 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) scale(-1, 1)", tilew / 2.0 - l / 2.0, hexh / 2.0))
                    );
                },
                8 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) scale(1, -1)", tilew / 2.0 + l / 2.0, tileh - hexh / 2.0))
                    );
                },
                9 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) scale(-1, -1)", tilew / 2.0 - l / 2.0, tileh - hexh / 2.0))
                    );
                },
                10 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {})", l / 2.0, tileh / 2.0 - l / 2.0))
                    );
                },
                11 => {
                    svg = svg.add(
                        p.clone()
                            .set("transform", format!("translate({}, {}) scale(-1, 1)", tilew - l / 2.0, tileh / 2.0 - l / 2.0))
                    );
                },
                12 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("translate({}, {}) rotate(-30, 0, 0)", l / 2.0, l / 2.0))
                    );
                },
                13 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("scale(-1, 1) translate({}, {}) rotate(-30, 0, 0)", -tilew + l / 2.0, l / 2.0))
                    );
                },
                14 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("translate({}, {}) rotate(30, 0, {})", l / 2.0, tileh / 2.0 - l / 2.0 - l, l))
                    );
                },
                15 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("scale(-1, 1) translate({}, {}) rotate(30, 0, {})", -tilew + l / 2.0, tileh / 2.0 - l / 2.0 - l, l))
                    );
                },
                16 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("scale(1, -1) translate({}, {}) rotate(30, 0, {})", l / 2.0, -tileh / 2.0 - l / 2.0 - l, l))
                    );
                },
                17 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("scale(-1, -1) translate({}, {}) rotate(30, 0, {})", -tilew + l / 2.0, -tileh / 2.0 - l / 2.0 - l, l))
                    );
                },
                18 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("scale(1, -1) translate({}, {}) rotate(-30, 0, 0)", l / 2.0, -tileh + l / 2.0))
                    );
                },
                19 => {
                    svg = svg.add(
                        r.clone()
                            .set("x", 0)
                            .set("y", 0)
                            .set("width", l)
                            .set("height", l)
                            .set("transform", format!("scale(-1, -1) translate({}, {}) rotate(-30, 0, 0)", -tilew + l / 2.0, -tileh + l / 2.0))
                    );
                },
                _ => {},
            }
        }

        Ok(svg)
    }
}
