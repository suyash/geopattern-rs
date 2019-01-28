use std::convert;
use std::error;
use std::fmt;
use std::num;

/// ColorError is a wrapper for all errors that
/// can occur on operations on an Color object
#[derive(Debug)]
pub enum ColorError {
    HexParse,
    ParseInt(num::ParseIntError),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorError::HexParse => write!(f, "ColorError: Invalid hex"),
            ColorError::ParseInt(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for ColorError {
    fn description(&self) -> &str {
        match *self {
            ColorError::HexParse => "ColorError: Invalid hex",
            ColorError::ParseInt(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ColorError::HexParse => None,
            ColorError::ParseInt(ref e) => Some(e),
        }
    }
}

impl convert::From<num::ParseIntError> for ColorError {
    fn from(e: num::ParseIntError) -> ColorError {
        ColorError::ParseInt(e)
    }
}

/// Smallest distinguishable color unit
pub const DELTA: f64 = 1.0 / 255.0;

/// This is the default reference white point.
/// TODO: find out what that means
///
/// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L52
pub const D65: (f64, f64, f64) = (0.95047, 1.00000, 1.08883);

/// Color is a single color object
#[derive(Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    /// create new colors
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color{
            r: r,
            g: g,
            b: b,
        }
    }

    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L356
    fn delinearize(v: f64) -> f64 {
        if v <= 0.0031308 {
            12.92 * v
        } else {
            1.055 * v.powf(1.0 / 2.4) - 0.055
        }
    }

    /// create new colors from hcl color space
    pub fn new_hcl(h: f64, c: f64, l: f64) -> Color {
        let (l, a, b) = Self::hcl_to_lab(h, c, l);
        let (x, y, z) = Self::lab_to_xyz(l, a, b);
        let (r, g, b) = Self::xyz_to_rgb(x, y, z);
        Self::new(r, g, b)
    }

    /// convert a hex string ("#abc", "#fedcba") into a `Color` object
    pub fn hex(s: &str) -> Result<Color, ColorError> {
        if s.as_bytes()[0] != b'#' {
            Err(ColorError::HexParse)
        } else if s.len() == 4 {
            let r = u8::from_str_radix(&s[1..2], 16)?;
            let g = u8::from_str_radix(&s[2..3], 16)?;
            let b = u8::from_str_radix(&s[3..4], 16)?;
            Ok(Color::new(((r * 16 + r) as f64) * DELTA, ((g * 16 + g) as f64) * DELTA, ((b * 16 + b) as f64) * DELTA))
        } else if s.len() == 7 {
            let r = u8::from_str_radix(&s[1..3], 16)?;
            let g = u8::from_str_radix(&s[3..5], 16)?;
            let b = u8::from_str_radix(&s[5..7], 16)?;
            Ok(Color::new((r as f64) * DELTA, (g as f64) * DELTA, (b as f64) * DELTA))
        } else {
            Err(ColorError::HexParse)
        }
    }

    /// get r, g, b for this object
    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }

    /// check if two colors are almost same
    ///
    /// this means that the total difference is less than 3 * DELTA
    pub fn almost_equal(&self, other: &Self) -> bool {
        let (or, og, ob) = other.rgb();
        ((self.r - or).abs() + (self.g - og).abs() + (self.b - ob).abs()) < 3.0 * DELTA
    }

    /// linear RGB representation for the color
    pub fn linear_rgb(&self) -> (f64, f64, f64) {
        (Self::linearize(self.r), Self::linearize(self.g), Self::linearize(self.b))
    }

    fn linearize(v: f64) -> f64 {
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }

    /// XYZ representation for the color
    ///
    /// http://www.sjbrown.co.uk/2004/05/14/gamma-correct-rendering/
    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L420
    pub fn xyz(&self) -> (f64, f64, f64) {
        let (r, g, b) = self.linear_rgb();

        (
            0.4124564*r + 0.3575761*g + 0.1804375*b,
            0.2126729*r + 0.7151522*g + 0.0721750*b,
            0.0193339*r + 0.1191920*g + 0.9503041*b,
        )
    }

    /// LAB representation for the color
    ///
    /// http://en.wikipedia.org/wiki/Lab_color_space#CIELAB-CIEXYZ_conversions
    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L501
    pub fn lab(&self) -> (f64, f64, f64) {
        let (x, y, z) = self.xyz();

        let fy = Self::lab_f(y / D65.1);

        (
            1.16 * fy - 0.16,
            5.0 * (Self::lab_f(x / D65.0) - fy),
            2.0 * (fy - Self::lab_f(z / D65.2)),
        )
    }

    fn lab_f(t: f64) -> f64 {
        if t > 6.0/29.0 * 6.0/29.0 * 6.0/29.0 {
            t.cbrt()
        } else {
            t/3.0 * 29.0/6.0 * 29.0/6.0 + 4.0/29.0
        }
    }

    /// HCL space values
    ///
    /// http://www.hunterlab.com/appnotes/an09_96a.pdf
    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L745
    pub fn hcl(&self) -> (f64, f64, f64) {
        let (l, a, b) = self.lab();

        let (mut h, c, l) = (0.0, (a * a + b * b).sqrt(), l);

        if (b - a).abs() > 1e-4 && a.abs() > 1e-4 {
            h = (360.0 + 57.29577951308232087721 * b.atan2(a)) % 360.0;
        }

        (h, c, l)
    }

    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L773
    pub fn hcl_to_lab(h: f64, c: f64, l: f64) -> (f64, f64, f64) {
        let h = 0.01745329251994329576 * h; // Deg2Rad
        (l, c * h.cos(), c * h.sin())
    }

    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L528
    pub fn lab_to_xyz(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
        let l2 = (l + 0.16) / 1.16;
        (
            D65.0 * Self::lab_finv(l2 + a / 5.0),
            D65.1 * Self::lab_finv(l2),
            D65.2 * Self::lab_finv(l2 - b / 2.0),
        )
    }

    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L516
    fn lab_finv(f: f64) -> f64 {
        if f > 6.0 / 29.0 {
            f * f * f
        } else {
            3.0 * 6.0 / 29.0 * 6.0 / 29.0 * (f - 4.0 / 29.0)
        }
    }

    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L402
    pub fn xyz_to_linear_rgb(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        (
            3.2404542*x - 1.5371385*y - 0.4985314*z,
            -0.9692660*x + 1.8760108*y + 0.0415560*z,
            0.0556434*x - 0.2040259*y + 1.0572252*z,
        )
    }

    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors.go#L402
    pub fn xyz_to_rgb(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        (
            Self::delinearize(3.2404542*x - 1.5371385*y - 0.4985314*z),
            Self::delinearize(-0.9692660*x + 1.8760108*y + 0.0415560*z),
            Self::delinearize(0.0556434*x - 0.2040259*y + 1.0572252*z),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Color};

    struct Case<'a> {
        hex: &'a str,
        rgb: (f64, f64, f64),
        xyz: (f64, f64, f64),
        lab: (f64, f64, f64),
        hcl: (f64, f64, f64),
    }

    /// Test cases from
    /// https://github.com/lucasb-eyer/go-colorful/blob/master/colors_test.go#L27-L62
    const CASES: [&Case; 15] = [
        &Case{hex: "#ffffff", rgb: (1.0, 1.0, 1.0), xyz: (0.950470, 1.000000, 1.088830), lab: (1.000000, 0.000000, 0.000000), hcl: (  0.0000, 0.000000, 1.000000)},
        &Case{hex: "#80ffff", rgb: (0.5, 1.0, 1.0), xyz: (0.626296, 0.832848, 1.073634), lab: (0.931390,-0.353319,-0.108946), hcl: (197.1371, 0.369735, 0.931390)},
        &Case{hex: "#ff80ff", rgb: (1.0, 0.5, 1.0), xyz: (0.669430, 0.437920, 0.995150), lab: (0.720892, 0.651673,-0.422133), hcl: (327.0661, 0.776450, 0.720892)},
        &Case{hex: "#ffff80", rgb: (1.0, 1.0, 0.5), xyz: (0.808654, 0.943273, 0.341930), lab: (0.977637,-0.165795, 0.602017), hcl: (105.3975, 0.624430, 0.977637)},
        &Case{hex: "#8080ff", rgb: (0.5, 0.5, 1.0), xyz: (0.345256, 0.270768, 0.979954), lab: (0.590453, 0.332846,-0.637099), hcl: (297.5843, 0.718805, 0.590453)},
        &Case{hex: "#ff8080", rgb: (1.0, 0.5, 0.5), xyz: (0.527613, 0.381193, 0.248250), lab: (0.681085, 0.483884, 0.228328), hcl: ( 25.2610, 0.535049, 0.681085)},
        &Case{hex: "#80ff80", rgb: (0.5, 1.0, 0.5), xyz: (0.484480, 0.776121, 0.326734), lab: (0.906026,-0.600870, 0.498993), hcl: (140.2920, 0.781050, 0.906026)},
        &Case{hex: "#808080", rgb: (0.5, 0.5, 0.5), xyz: (0.203440, 0.214041, 0.233054), lab: (0.533890, 0.000000, 0.000000), hcl: (  0.0000, 0.000000, 0.533890)},
        &Case{hex: "#00ffff", rgb: (0.0, 1.0, 1.0), xyz: (0.538014, 0.787327, 1.069496), lab: (0.911132,-0.480875,-0.141312), hcl: (196.3762, 0.501209, 0.911132)},
        &Case{hex: "#ff00ff", rgb: (1.0, 0.0, 1.0), xyz: (0.592894, 0.284848, 0.969638), lab: (0.603242, 0.982343,-0.608249), hcl: (328.2350, 1.155407, 0.603242)},
        &Case{hex: "#ffff00", rgb: (1.0, 1.0, 0.0), xyz: (0.770033, 0.927825, 0.138526), lab: (0.971393,-0.215537, 0.944780), hcl: (102.8512, 0.969054, 0.971393)},
        &Case{hex: "#0000ff", rgb: (0.0, 0.0, 1.0), xyz: (0.180437, 0.072175, 0.950304), lab: (0.322970, 0.791875,-1.078602), hcl: (306.2849, 1.338076, 0.322970)},
        &Case{hex: "#00ff00", rgb: (0.0, 1.0, 0.0), xyz: (0.357576, 0.715152, 0.119192), lab: (0.877347,-0.861827, 0.831793), hcl: (136.0160, 1.197759, 0.877347)},
        &Case{hex: "#ff0000", rgb: (1.0, 0.0, 0.0), xyz: (0.412456, 0.212673, 0.019334), lab: (0.532408, 0.800925, 0.672032), hcl: ( 39.9990, 1.045518, 0.532408)},
        &Case{hex: "#000000", rgb: (0.0, 0.0, 0.0), xyz: (0.000000, 0.000000, 0.000000), lab: (0.000000, 0.000000, 0.000000), hcl: (  0.0000, 0.000000, 0.000000)},
    ];

    fn almosteq(a: f64, b: f64) -> bool {
        if a.abs() > 1.0 / 256.0 {
            ((a - b) / a).abs() < 1.0 / 256.0
        } else {
            true
        }
    }

    #[test]
    fn rgb_conversion() {
        for case in &CASES {
            let c = Color::hex(case.hex).unwrap();
            assert!(c.almost_equal(&Color::new(case.rgb.0, case.rgb.1, case.rgb.2)));
        }
    }

    #[test]
    fn xyz_conversion() {
        for case in &CASES {
            let c = Color::new(case.rgb.0, case.rgb.1, case.rgb.2);
            let (x, y, z) = c.xyz();
            assert!(almosteq(x, case.xyz.0), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.xyz, c.xyz()));
            assert!(almosteq(y, case.xyz.1), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.xyz, c.xyz()));
            assert!(almosteq(z, case.xyz.2), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.xyz, c.xyz()));
        }
    }

    #[test]
    fn lab_conversion() {
        for case in &CASES {
            let c = Color::new(case.rgb.0, case.rgb.1, case.rgb.2);
            let (l, a, b) = c.lab();
            assert!(almosteq(l, case.lab.0), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.lab, c.lab()));
            assert!(almosteq(a, case.lab.1), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.lab, c.lab()));
            assert!(almosteq(b, case.lab.2), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.lab, c.lab()));
        }
    }

    #[test]
    fn hcl_conversion() {
        for case in &CASES {
            let col = Color::new(case.rgb.0, case.rgb.1, case.rgb.2);
            let (h, c, l) = col.hcl();
            assert!(almosteq(h, case.hcl.0), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.hcl, col.hcl()));
            assert!(almosteq(c, case.hcl.1), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.hcl, col.hcl()));
            assert!(almosteq(l, case.hcl.2), format!("hex: {}, expected: {:?}, got: {:?}", case.hex, case.hcl, col.hcl()));
        }
    }

    #[test]
    fn hcl_to_lab_conversion() {
        for case in &CASES {
            let (l, a, b) = Color::hcl_to_lab(case.hcl.0, case.hcl.1, case.hcl.2);
            assert!(almosteq(l, case.lab.0), format!("hex: {}, expected: {}, got: {}", case.hex, case.lab.0, l));
            assert!(almosteq(a, case.lab.1), format!("hex: {}, expected: {}, got: {}", case.hex, case.lab.1, a));
            assert!(almosteq(b, case.lab.2), format!("hex: {}, expected: {}, got: {}", case.hex, case.lab.2, b));
        }
    }

    #[test]
    fn lab_to_xyz_conversion() {
        for case in &CASES {
            let (x, y, z) = Color::lab_to_xyz(case.lab.0, case.lab.1, case.lab.2);
            assert!(almosteq(x, case.xyz.0), format!("hex: {}, expected: {}, got: {}", case.hex, case.xyz.0, x));
            assert!(almosteq(y, case.xyz.1), format!("hex: {}, expected: {}, got: {}", case.hex, case.xyz.1, y));
            assert!(almosteq(z, case.xyz.2), format!("hex: {}, expected: {}, got: {}", case.hex, case.xyz.2, z));
        }
    }

    #[test]
    fn xyz_to_rgb_conversion() {
        for case in &CASES {
            let (r, g, b) = Color::xyz_to_rgb(case.xyz.0, case.xyz.1, case.xyz.2);
            assert!(almosteq(r, case.rgb.0), format!("hex: {}, expected: {}, got: {}", case.hex, case.rgb.0, r));
            assert!(almosteq(g, case.rgb.1), format!("hex: {}, expected: {}, got: {}", case.hex, case.rgb.1, g));
            assert!(almosteq(b, case.rgb.2), format!("hex: {}, expected: {}, got: {}", case.hex, case.rgb.2, b));
        }
    }
}
