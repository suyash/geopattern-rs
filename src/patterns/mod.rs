use std::convert;
use std::error;
use std::fmt;
use std::num;

use svg::Document;

const STROKE_COLOR: &'static str = "#000";
const STROKE_OPACITY: f64 = 0.02;
const DARK_FILL_COLOR: &'static str = "#222";
const LIGHT_FILL_COLOR: &'static str = "#ddd";

/// Patterns is a list of available patterns to choose from
#[derive(Debug, Copy, Clone)]
pub enum Patterns {
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
}

impl fmt::Display for Patterns {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Patterns::Chevrons => write!(f, "Chevrons"),
            Patterns::ConcentricCircles => write!(f, "ConcentricCircles"),
            Patterns::Diamonds => write!(f, "Diamonds"),
            Patterns::Hexagons => write!(f, "Hexagons"),
            Patterns::MosaicSquares => write!(f, "MosaicSquares"),
            Patterns::NestedSquares => write!(f, "NestedSquares"),
            Patterns::Octagons => write!(f, "Octagons"),
            Patterns::OverlappingCircles => write!(f, "OverlappingCircles"),
            Patterns::OverlappingRings => write!(f, "OverlappingRings"),
            Patterns::Plaid => write!(f, "Plaid"),
            Patterns::PlusSigns => write!(f, "PlusSigns"),
            Patterns::SineWaves => write!(f, "SineWaves"),
            Patterns::Squares => write!(f, "Squares"),
            Patterns::Tessellation => write!(f, "Tessellation"),
            Patterns::Triangles => write!(f, "Triangles"),
            Patterns::Xes => write!(f, "Xes"),
        }
    }
}

/// Pattern is a single pattern that can be used as the structure.
pub trait Pattern {
    /// draw can take a svg document and implement the current patterns structure
    /// on it, and returns the result of that operation.
    fn draw(&self, svg: Document) -> Result<Document, PatternError>;
}

/// PatternError is a wrapped error for all errors
/// originating from the 'Pattern' trait and its implementations
///
/// TODO: improve this to be more useful.
#[derive(Debug)]
pub enum PatternError {
    IntError(num::ParseIntError),
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PatternError::IntError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for PatternError {
    fn description(&self) -> &str {
        match *self {
            PatternError::IntError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PatternError::IntError(ref e) => Some(e),
        }
    }
}

impl convert::From<num::ParseIntError> for PatternError {
    fn from(e: num::ParseIntError) -> PatternError {
        PatternError::IntError(e)
    }
}

fn rescale(v: f64, amin: f64, amax: f64, bmin: f64, bmax: f64) -> f64 {
    bmax - (amax - v) * ((bmax - bmin) / (amax - amin))
}

fn opacity(v: f64) -> f64 {
    rescale(v, 0.0, 15.0, 0.02, 0.15)
}

fn fill_color(v: u8) -> &'static str {
    if v & 1 == 1 {
        DARK_FILL_COLOR
    } else {
        LIGHT_FILL_COLOR
    }
}

pub mod chevrons;
pub mod concentric_circles;
pub mod diamonds;
pub mod hexagons;
pub mod mosaic_squares;
pub mod nested_squares;
pub mod octagons;
pub mod overlapping_circles;
pub mod overlapping_rings;
pub mod plaid;
pub mod plus_signs;
pub mod sine_waves;
pub mod squares;
pub mod tessellation;
pub mod triangles;
pub mod xes;

pub use self::chevrons::Chevrons;
pub use self::concentric_circles::ConcentricCircles;
pub use self::diamonds::Diamonds;
pub use self::hexagons::Hexagons;
pub use self::mosaic_squares::MosaicSquares;
pub use self::nested_squares::NestedSquares;
pub use self::octagons::Octagons;
pub use self::overlapping_circles::OverlappingCircles;
pub use self::overlapping_rings::OverlappingRings;
pub use self::plaid::Plaid;
pub use self::plus_signs::PlusSigns;
pub use self::sine_waves::SineWaves;
pub use self::squares::Squares;
pub use self::tessellation::Tessellation;
pub use self::triangles::Triangles;
pub use self::xes::Xes;
