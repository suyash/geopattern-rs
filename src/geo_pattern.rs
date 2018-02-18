use std::convert;
use std::error;
use std::fmt;
use std::num;

use base64::encode;

use svg::Document;
use svg::node::element::Rectangle;
use sha1::Sha1;

use color::Color;
use patterns::{
    Pattern,
    PatternError,
    Patterns,
    Chevrons,
    ConcentricCircles,
    Diamonds,
    Hexagons,
    MosaicSquares,
    NestedSquares,
    Octagons,
    OverlappingCircles,
    OverlappingRings,
    Plaid,
    PlusSigns,
    SineWaves,
    Squares,
    Tessellation,
    Triangles,
    Xes,
};

/// This is the default background color if nothing else is specified
pub const DEFAULT_BASE_COLOR: &'static str = "#933c3c";

/// TODO: this is repeated, figure out private internal modules like in rayon/hornet
fn rescale(v: f64, amin: f64, amax: f64, bmin: f64, bmax: f64) -> f64 {
    bmax - (amax - v) * ((bmax - bmin) / (amax - amin))
}

#[derive(Debug)]
pub struct GeoPattern<'a> {
    svg: Option<Document>,
    base_color: Color,
    color: Option<Color>,
    hash: String,
    patterns: &'a [Patterns],
}

impl<'a> GeoPattern<'a> {
    /// This class uses the builder pattern for construction.
    /// A string is mandatory, hence is a parameter for construction.
    /// Rest of the options can be provided using static member functions.
    /// calling `build` finishes creating a new object.
    pub fn new(s: &str) -> GeoPattern {
        GeoPattern{
            svg: None,
            base_color: Color::hex(DEFAULT_BASE_COLOR).unwrap(),
            color: None,
            hash: Sha1::from(s).digest().to_string(),
            patterns: &[
                Patterns::Chevrons,
                Patterns::ConcentricCircles,
                Patterns::Diamonds,
                Patterns::Hexagons,
                Patterns::MosaicSquares,
                Patterns::NestedSquares,
                Patterns::Octagons,
                Patterns::OverlappingCircles,
                Patterns::OverlappingRings,
                Patterns::Plaid,
                Patterns::PlusSigns,
                Patterns::SineWaves,
                Patterns::Squares,
                Patterns::Tessellation,
                Patterns::Triangles,
                Patterns::Xes,
            ],
        }
    }

    /// sets color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// sets base color
    pub fn base_color(mut self, base_color: Color) -> Self {
        self.base_color = base_color;
        self
    }

    /// sets patterns to use
    pub fn patterns(mut self, patterns: &'a [Patterns]) -> Self {
        self.patterns = patterns;
        self
    }

    /// build internally builds the svg.
    pub fn build(mut self) -> Result<Self, GeoPatternError> {
        if self.color.is_none() {
            self.initialize_color()?;
        }

        let svg = Document::new();
        let svg = self.create_background(svg)?;
        let svg = self.create_pattern(svg)?;

        self.svg = Some(svg);
        Ok(self)
    }

    /// returns a reference to svg::Document, if one has been built.
    /// The returned reference can be used with svg crate's functions, like svg::save
    pub fn to_svg(&self) -> Result<&Document, GeoPatternError> {
        match &self.svg {
            &Some(ref d) => Ok(d),
            &None => Err(GeoPatternError::NotBuilt),
        }
    }

    /// removes all the newlines from formatted svg::Document and returns it.
    ///
    /// TODO: figure out if this is actually minified
    pub fn to_minified_svg(&self) -> Result<String, GeoPatternError> {
        Ok(
            format!("{}", self.to_svg()?)
                .chars()
                .map(|x| match x {
                    '\n' => '\0',
                    _ => x,
                })
                .collect()
        )
    }

    /// creates a data URI (data:image/svg+xml;utf8,...) from the minified svg.
    pub fn to_data_uri(&self) -> Result<String, GeoPatternError> {
        Ok(format!("data:image/svg+xml;utf8,{}", self.to_minified_svg()?))
    }

    /// creates a base64 encoding of the minified svg.
    pub fn to_base64(&self) -> Result<String, GeoPatternError> {
        Ok(encode(&self.to_minified_svg()?))
    }

    /// creates a data URI (data:image/svg+xml;base64,...) of a base64 encoded svg.
    pub fn to_base64_data_uri(&self) -> Result<String, GeoPatternError> {
        Ok(format!("data:image/svg+xml;base64,{}", self.to_base64()?))
    }

    /// if color is uninitialized at build time, this will initialize it
    ///
    /// https://github.com/jasonlong/geo_pattern/blob/master/lib/geo_pattern/color_generators/base_color_generator.rb#L29-L41
    fn initialize_color(&mut self) -> Result<(), GeoPatternError> {
        let (h, c, l) = self.base_color.hcl();
        let hue_off = rescale(u64::from_str_radix(&self.hash.as_str()[14..17], 16)? as f64, 0.0, 4095.0, 0.0, 359.0);
        let sat_off = u64::from_str_radix(&self.hash.as_str()[17..18], 16)?;
        let rs = rescale(sat_off as f64, 0.0, 15.0, 0.0, 1.0);

        let hue = (h - hue_off).abs();
        let sat = match sat_off % 2 {
            0 => c + rs as f64,
            _ => c - rs as f64,
        };

        self.color = Some(Color::new_hcl(hue, sat, l));
        Ok(())
    }

    /// builds the background
    fn create_background(&mut self, svg: Document) -> Result<Document, GeoPatternError> {
        match &self.color {
            &Some(ref c) => {
                let (r, g, b) = c.rgb();

                // TODO: shouldn't need abs and mod here. Somehow hcl -> rgb pipeline in color
                // is broken, despite all the "tests" passing.
                let (r, g, b) = (((r * 105.0).abs() as u64) % 256, ((g * 105.0).abs() as u64) % 256, ((b * 150.0).abs() as u64) % 256);

                let rect = Rectangle::new()
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", "100%")
                    .set("height", "100%")
                    .set("fill", format!("rgb({}, {}, {})", r, g, b));

                Ok(svg.add(rect))
            },
            &None => Err(GeoPatternError::UninitializedColor)
        }
    }

    /// builds the structure
    fn create_pattern(&mut self, svg: Document) -> Result<Document, GeoPatternError> {
        let val = u64::from_str_radix(&self.hash.as_str()[20..21], 16).unwrap();
        let index = rescale(val as f64, 0.0, 15.0, 0.0, (self.patterns.len() - 1) as f64).round() as usize;
        let pattern = &self.patterns[index];
        let svg = match pattern {
            &Patterns::Chevrons => Chevrons::new(&self.hash).draw(svg)?,
            &Patterns::ConcentricCircles => ConcentricCircles::new(&self.hash).draw(svg)?,
            &Patterns::Diamonds => Diamonds::new(&self.hash).draw(svg)?,
            &Patterns::Hexagons => Hexagons::new(&self.hash).draw(svg)?,
            &Patterns::MosaicSquares => MosaicSquares::new(&self.hash).draw(svg)?,
            &Patterns::NestedSquares => NestedSquares::new(&self.hash).draw(svg)?,
            &Patterns::Octagons => Octagons::new(&self.hash).draw(svg)?,
            &Patterns::OverlappingCircles => OverlappingCircles::new(&self.hash).draw(svg)?,
            &Patterns::OverlappingRings => OverlappingRings::new(&self.hash).draw(svg)?,
            &Patterns::Plaid => Plaid::new(&self.hash).draw(svg)?,
            &Patterns::PlusSigns => PlusSigns::new(&self.hash).draw(svg)?,
            &Patterns::SineWaves => SineWaves::new(&self.hash).draw(svg)?,
            &Patterns::Squares => Squares::new(&self.hash).draw(svg)?,
            &Patterns::Tessellation => Tessellation::new(&self.hash).draw(svg)?,
            &Patterns::Triangles => Triangles::new(&self.hash).draw(svg)?,
            &Patterns::Xes => Xes::new(&self.hash).draw(svg)?,
        };

        Ok(svg)
    }
}

/// GeoPatternError is a wrapper for all errors that can be created
/// by a GeoPattern Object.
///
/// TODO: improve this to be more useful.
#[derive(Debug)]
pub enum GeoPatternError {
    UninitializedColor,
    NotBuilt,
    Pattern(PatternError),
    ParseInt(num::ParseIntError),
}

impl fmt::Display for GeoPatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GeoPatternError::Pattern(ref e) => e.fmt(f),
            GeoPatternError::ParseInt(ref e) => e.fmt(f),
            GeoPatternError::UninitializedColor => write!(f, "color is uninitialized"),
            GeoPatternError::NotBuilt => write!(f, "not built yet"),
        }
    }
}

impl error::Error for GeoPatternError {
    fn description(&self) -> &str {
        match *self {
            GeoPatternError::Pattern(ref e) => e.description(),
            GeoPatternError::ParseInt(ref e) => e.description(),
            GeoPatternError::UninitializedColor => "color is uninitialized",
            GeoPatternError::NotBuilt => "not built yet",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GeoPatternError::Pattern(ref e) => Some(e),
            GeoPatternError::ParseInt(ref e) => Some(e),
            GeoPatternError::UninitializedColor => None,
            GeoPatternError::NotBuilt => None,
        }
    }
}

impl convert::From<PatternError> for GeoPatternError {
    fn from(e: PatternError) -> GeoPatternError {
        GeoPatternError::Pattern(e)
    }
}

impl convert::From<num::ParseIntError> for GeoPatternError {
    fn from(e: num::ParseIntError) -> GeoPatternError {
        GeoPatternError::ParseInt(e)
    }
}
