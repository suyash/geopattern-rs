use svg::Document;
use svg::node::element::{Path};

use super::{
    fill_color,
    opacity,
    rescale,
    Pattern,
    PatternError,
};

pub struct SineWaves<'a> {
    hash: &'a str,
}

impl<'a> SineWaves<'a> {
    pub fn new(hash: &'a str) -> Self {
        SineWaves{
            hash: hash,
        }
    }
}

impl<'a> Pattern for SineWaves<'a> {
    fn draw(&self, svg: Document) -> Result<Document, PatternError> {
        let p = rescale(u8::from_str_radix(&self.hash[0..1], 16)? as f64, 0.0, 15.0, 100.0, 400.0);
        let a = rescale(u8::from_str_radix(&self.hash[1..2], 16)? as f64, 0.0, 15.0, 30.0, 100.0);
        let ww = rescale(u8::from_str_radix(&self.hash[2..3], 16)? as f64, 0.0, 15.0, 3.0, 30.0);

        let mut svg = svg
            .set("width", p)
            .set("height", ww * 36.0);

        for i in 0..36 {
            let v = u8::from_str_radix(&self.hash[i..i+1], 16)?;
            let (o, f) = (opacity(v as f64), fill_color(v));

            let xoff = (p / 4.0) * 0.7;

            let path = Path::new()
                .set("d", format!("M0 {} C {} 0, {} 0, {} {} S {} {}, {} {} S {} 0, {}, {}", a, xoff, p / 2.0 - xoff, p / 2.0, a, p - xoff, a * 2.0, p, a, p * 1.5 - xoff, p * 1.5, a))
                .set("fill", "none")
                .set("stroke", f)
                .set("style", format!("opacity:{};stroke-width:0x{:X};", o, ww as u64));

            svg = svg.add(
                path.clone()
                    .set("transform", format!("translate(-{}, {})", p / 4.0, (ww * (i as f64)) - (a * 1.5)))
            );

            svg = svg.add(
                path.clone()
                    .set("transform", format!("translate(-{}, {})", p / 4.0, (ww * (i as f64)) - (a * 1.5) + ww * 36.0))
            );
        }

        Ok(svg)
    }
}
