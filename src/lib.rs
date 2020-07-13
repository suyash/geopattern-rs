//! A crate for creating SVG geometric patterns

#![deny(missing_docs)]

use svg::node::element::{Circle, Group, Polyline, Rectangle};
use svg::Document;

/// Fill defines properties for fill colors
pub struct Fill<'a> {
    color: &'a [&'a str],
    opacity: &'a [f32],
}

impl<'a> Fill<'a> {
    /// create new Fill
    pub fn new(color: &'a [&'a str], opacity: &'a [f32]) -> Self {
        assert_eq!(color.len(), opacity.len());

        Fill { color, opacity }
    }
}

/// chevrons
///
/// ```
/// use geopattern::{Fill, chevrons};
///
/// let c = chevrons(
///     60.0,
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     ("#000", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn chevrons<'a>(
    chevron_width: f32,
    (width, height): (usize, usize),
    fill: Fill<'a>,
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    assert_eq!(fill.opacity.len(), width * height);

    let (stroke_color, stroke_opacity) = stroke;

    let chevron = |w, h| {
        let e = h * 0.66;

        (
            Polyline::new().set(
                "points",
                format!("0,0,{},{},{},{},0,{},0,0", w / 2.0, h - e, w / 2.0, h, e),
            ),
            Polyline::new().set(
                "points",
                format!(
                    "{},{},{},0,{},{},{},{},{},{}",
                    w / 2.0,
                    h - e,
                    w,
                    w,
                    e,
                    w / 2.0,
                    h,
                    w / 2.0,
                    h - e
                ),
            ),
        )
    };
    let c = chevron(chevron_width, chevron_width);

    let mut doc = Document::new()
        .set("width", chevron_width * width as f32)
        .set("height", chevron_width * height as f32 * 0.66)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let g = Group::new()
                .set("fill", fill.color[ix])
                .set("fill-opacity", fill.opacity[ix])
                .set("stroke", stroke_color)
                .set("stroke-opacity", stroke_opacity)
                .set("stroke-width", 1)
                .add(c.0.clone())
                .add(c.1.clone());

            doc = doc.add(g.clone().set(
                "transform",
                format!(
                    "translate({}, {})",
                    (x as f32) * chevron_width,
                    (y as f32) * chevron_width * 0.66 - chevron_width / 2.0
                ),
            ));

            if y == 0 {
                doc = doc.add(g.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        (x as f32) * chevron_width,
                        height as f32 * chevron_width * 0.66 - chevron_width / 2.0
                    ),
                ));
            }
        }
    }

    doc
}

/// concentric circles
///
/// ```
/// use geopattern::{Fill, concentric_circles};
///
/// let c = concentric_circles(
///     20.0,
///     4.0,
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     Fill::new(
///         &(0..4).rev().map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).rev().map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn concentric_circles<'a>(
    diameter: f32,
    concentric_width: f32,
    (width, height): (usize, usize),
    fill_outer: Fill<'a>,
    fill_inner: Fill<'a>,
    background_color: &'a str,
) -> Document {
    assert_eq!(fill_outer.opacity.len(), width * height);
    assert_eq!(fill_inner.opacity.len(), width * height);

    let diameter = diameter + concentric_width;

    let mut doc = Document::new()
        .set("width", diameter * width as f32)
        .set("height", diameter * height as f32)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let radius = diameter / 2.0;
            let (cx, cy) = (
                (x as f32) * diameter + radius,
                (y as f32) * diameter + radius,
            );

            doc = doc.add(
                Circle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", diameter / 2.0)
                    .set("stroke", fill_outer.color[ix])
                    .set(
                        "style",
                        format!(
                            "opacity:{}; stroke-width:0x{:X};",
                            fill_outer.opacity[ix],
                            (concentric_width as i64)
                        ),
                    ),
            );

            doc = doc.add(
                Circle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", diameter / 4.0)
                    .set("fill", fill_inner.color[ix])
                    .set("fill-opacity", fill_inner.opacity[ix]),
            );
        }
    }

    doc
}

/// diamonds
///
/// ```
/// use geopattern::{Fill, diamonds};
///
/// let c = diamonds(
///     (20.0, 20.0),
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn diamonds<'a>(
    (diamond_width, diamond_height): (f32, f32),
    (width, height): (usize, usize),
    fill: Fill<'a>,
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    assert_eq!(fill.opacity.len(), width * height);

    let points = format!(
        "{},0,{},{},{},{},0,{}",
        diamond_width / 2.0,
        diamond_width,
        diamond_height / 2.0,
        diamond_width / 2.0,
        diamond_height,
        diamond_height / 2.0
    );

    let (stroke_color, stroke_opacity) = stroke;

    let mut doc = Document::new()
        .set("width", diamond_width * width as f32)
        .set("height", diamond_height * height as f32 / 2.0)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let polyline = Polyline::new()
                .set("points", points.as_str())
                .set("fill", fill.color[ix])
                .set("fill-opacity", fill.opacity[ix])
                .set("stroke", stroke_color)
                .set("stroke-opacity", stroke_opacity);

            let dx = match y % 2 {
                1 => diamond_width / 2.0,
                _ => 0.0,
            };

            doc = doc.add(polyline.clone().set(
                "transform",
                format!(
                    "translate({}, {})",
                    dx + (x as f32) * diamond_width - (diamond_width / 2.0),
                    (diamond_height / 2.0) * (y as f32) - (diamond_height / 2.0)
                ),
            ));

            if x == 0 {
                doc = doc.add(polyline.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        dx + (width as f32) * diamond_width - (diamond_width / 2.0),
                        (diamond_height / 2.0) * (y as f32) - (diamond_height / 2.0)
                    ),
                ));
            }

            if y == 0 {
                doc = doc.add(polyline.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        dx + (x as f32) * diamond_width - (diamond_width / 2.0),
                        diamond_height / 2.0 * (height as f32) - diamond_height / 2.0
                    ),
                ));
            }

            if x == 0 && y == 0 {
                doc = doc.add(polyline.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        dx + (width as f32) * diamond_width - (diamond_width / 2.0),
                        (diamond_height / 2.0) * (height as f32) - (diamond_height / 2.0)
                    ),
                ));
            }
        }
    }

    doc
}

/// hexagons
///
/// ```
/// use geopattern::{Fill, hexagons};
///
/// let c = hexagons(
///     20.0,
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn hexagons<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: Fill<'a>,
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    assert_eq!(fill.opacity.len(), width * height);

    let hexagon_width = side * 2.0;
    let hexagon_height = side * (3.0 as f32).sqrt();

    let a = side / 2.0;
    let b = (std::f32::consts::PI / 3.0).sin() * side;
    let points = format!(
        "0,{},{},0,{},0,{},{},{},{},{},{},0,{}",
        b,
        a,
        a + side,
        2.0 * side,
        b,
        a + side,
        2.0 * b,
        a,
        2.0 * b,
        b
    );

    let mut doc = Document::new()
        .set("width", (hexagon_width + side) * width as f32 / 2.0)
        .set("height", hexagon_height * height as f32)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let polyline = Polyline::new()
                .set("points", points.as_str())
                .set("fill", fill.color[ix])
                .set("fill-opacity", fill.opacity[ix])
                .set("stroke", stroke.0)
                .set("stroke-opacity", stroke.1);

            let dy = match x % 2 {
                1 => (y as f32) * hexagon_height + hexagon_height / 2.0,
                _ => (y as f32) * hexagon_height,
            };

            doc = doc.add(polyline.clone().set(
                "transform",
                format!(
                    "translate({}, {})",
                    (x as f32) * side * 1.5 - hexagon_width / 2.0,
                    dy - hexagon_height / 2.0
                ),
            ));

            if x == 0 {
                doc = doc.add(polyline.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        width as f32 * side * 1.5 - hexagon_width / 2.0,
                        dy - hexagon_height / 2.0
                    ),
                ));
            }

            if y == 0 {
                let dy = match x % 2 {
                    1 => height as f32 * hexagon_height + hexagon_height / 2.0,
                    _ => height as f32 * hexagon_height,
                };

                doc = doc.add(polyline.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        (x as f32) * side * 1.5 - hexagon_width / 2.0,
                        dy - hexagon_height / 2.0
                    ),
                ));
            }

            if x == 0 && y == 0 {
                doc = doc.add(polyline.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        width as f32 * side * 1.5 - hexagon_width / 2.0,
                        5.0 * hexagon_height + hexagon_height / 2.0
                    ),
                ));
            }
        }
    }

    doc
}

/// mosaic squares
///
/// ```
/// use geopattern::{Fill, mosaic_squares};
///
/// let c = mosaic_squares(
///     20.0,
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     Fill::new(
///         &(0..4).rev().map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).rev().map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn mosaic_squares<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill_outer: Fill<'a>,
    fill_inner: Fill<'a>,
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    assert_eq!(fill_outer.opacity.len(), width * height);
    assert_eq!(fill_inner.opacity.len(), width * height);

    let mut doc = Document::new()
        .set("width", side * width as f32 * 2.0)
        .set("height", side * height as f32 * 2.0)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    let draw_outer_tile = |mut doc: Document, x: f32, y: f32, ix: usize| {
        let points = format!("0,0,{},{},0,{},0,0", side, side, side);

        let polyline = Polyline::new()
            .set("points", points)
            .set("fill", fill_outer.color[ix])
            .set("fill-opacity", fill_outer.opacity[ix])
            .set("stroke", stroke.0)
            .set("stroke-opacity", stroke.1);

        doc = doc.add(polyline.clone().set(
            "transform",
            format!("translate({}, {}) scale(1, -1)", x, y + side),
        ));

        doc = doc.add(polyline.clone().set(
            "transform",
            format!("translate({}, {}) scale(-1, -1)", x + side * 2.0, y + side),
        ));

        doc = doc.add(polyline.clone().set(
            "transform",
            format!("translate({}, {}) scale(1, 1)", x, y + side),
        ));

        doc = doc.add(polyline.set(
            "transform",
            format!("translate({}, {}) scale(-1, 1)", x + side * 2.0, y + side),
        ));

        doc
    };

    let draw_inner_tile = |mut doc: Document, x: f32, y: f32, ix: usize| {
        let points = format!("0,0,{},{},0,{},0,0", side, side, side);

        let polyline = Polyline::new()
            .set("points", points.as_str())
            .set("fill", fill_outer.color[ix])
            .set("fill-opacity", fill_outer.opacity[ix])
            .set("stroke", stroke.0)
            .set("stroke-opacity", stroke.1);

        doc = doc.add(polyline.clone().set(
            "transform",
            format!("translate({}, {}) scale(-1, 1)", x + side, y),
        ));

        doc = doc.add(polyline.set(
            "transform",
            format!("translate({}, {}) scale(1, -1)", x + side, y + side * 2.0),
        ));

        let polyline = Polyline::new()
            .set("points", points)
            .set("fill", fill_inner.color[ix])
            .set("fill-opacity", fill_inner.opacity[ix])
            .set("stroke", stroke.0)
            .set("stroke-opacity", stroke.1);

        doc = doc.add(polyline.clone().set(
            "transform",
            format!("translate({}, {}) scale(-1, -1)", x + side, y + side * 2.0),
        ));

        doc = doc.add(polyline.set(
            "transform",
            format!("translate({}, {}) scale(1, 1)", x + side, y),
        ));

        doc
    };

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            match x % 2 {
                1 => match y % 2 {
                    1 => {
                        doc = draw_outer_tile(
                            doc,
                            (x as f32) * side * 2.0,
                            (y as f32) * side * 2.0,
                            ix,
                        );
                    }
                    _ => {
                        doc = draw_inner_tile(
                            doc,
                            (x as f32) * side * 2.0,
                            (y as f32) * side * 2.0,
                            ix,
                        );
                    }
                },
                _ => match y % 2 {
                    1 => {
                        doc = draw_inner_tile(
                            doc,
                            (x as f32) * side * 2.0,
                            (y as f32) * side * 2.0,
                            ix,
                        );
                    }
                    _ => {
                        doc = draw_outer_tile(
                            doc,
                            (x as f32) * side * 2.0,
                            (y as f32) * side * 2.0,
                            ix,
                        );
                    }
                },
            }
        }
    }

    doc
}

/// nested squares
///
/// TODO: make `outer_side` independent of `inner_side`, currently evaluates to `7 * inner_side`
///
/// ```
/// use geopattern::{Fill, nested_squares};
///
/// let c = nested_squares(
///     60.0,
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     Fill::new(
///         &(0..4).rev().map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).rev().map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn nested_squares<'a>(
    inner_side: f32,
    (width, height): (usize, usize),
    stroke_outer: Fill<'a>,
    fill_inner: Fill<'a>,
    background_color: &'a str,
) -> Document {
    assert_eq!(fill_inner.opacity.len(), width * height);
    assert_eq!(stroke_outer.color.len(), width * height);

    let outer_side = inner_side * 7.0;

    let mut doc = Document::new()
        .set("width", (inner_side * 2.0 + outer_side) * width as f32)
        .set("height", (inner_side * 2.0 + outer_side) * height as f32)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            // outer
            doc = doc.add(
                Rectangle::new()
                    .set(
                        "x",
                        (x as f32) * outer_side + (x as f32) * inner_side * 2.0 + inner_side / 2.0,
                    )
                    .set(
                        "y",
                        (y as f32) * outer_side + (y as f32) * inner_side * 2.0 + inner_side / 2.0,
                    )
                    .set("width", outer_side)
                    .set("height", outer_side)
                    .set("fill", "none")
                    .set("stroke", stroke_outer.color[ix])
                    .set(
                        "style",
                        format!(
                            "opacity:{};stroke-width:0x{:X};",
                            stroke_outer.opacity[ix], inner_side as u64
                        ),
                    ),
            );

            // inner
            doc = doc.add(
                Rectangle::new()
                    .set(
                        "x",
                        (x as f32) * outer_side
                            + (x as f32) * inner_side * 2.0
                            + inner_side / 2.0
                            + inner_side * 2.0,
                    )
                    .set(
                        "y",
                        (y as f32) * outer_side
                            + (y as f32) * inner_side * 2.0
                            + inner_side / 2.0
                            + inner_side * 2.0,
                    )
                    .set("width", inner_side * 3.0)
                    .set("height", inner_side * 3.0)
                    .set("fill", "none")
                    .set("stroke", fill_inner.color[ix])
                    .set(
                        "style",
                        format!(
                            "opacity:{};stroke-width:0x{:X};",
                            fill_inner.opacity[ix], inner_side as u64
                        ),
                    ),
            );
        }
    }

    doc
}

/// octagons
///
/// TODO: make `outer_side` independent of `inner_side`, currently evaluates to `7 * inner_side`
///
/// ```
/// use geopattern::{Fill, octagons};
///
/// let c = octagons(
///     60.0,
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn octagons<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: Fill<'a>,
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    assert_eq!(fill.opacity.len(), width * height);

    let mut doc = Document::new()
        .set("width", side * width as f32)
        .set("height", side * height as f32)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    let c = 0.33 * side;
    let points = format!(
        "{},0,{},0,{},{},{},{},{},{},{},{},0,{},0,{},{},0",
        c,
        side - c,
        side,
        c,
        side,
        side - c,
        side - c,
        side,
        c,
        side,
        side - c,
        c,
        c
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            doc = doc.add(
                Polyline::new()
                    .set("points", points.as_str())
                    .set("fill", fill.color[ix])
                    .set("fill-opacity", fill.opacity[ix])
                    .set("stroke", stroke.0)
                    .set("stroke-opacity", stroke.1)
                    .set(
                        "transform",
                        format!("translate({}, {})", (x as f32) * side, (y as f32) * side),
                    ),
            );
        }
    }

    doc
}

/// overlapping circles
///
/// TODO: make `outer_side` independent of `inner_side`, currently evaluates to `7 * inner_side`
///
/// ```
/// use geopattern::{Fill, overlapping_circles};
///
/// let c = overlapping_circles(
///     60.0,
///     (2, 2),
///     Fill::new(
///         &(0..4).map(|v| if v & 1 == 0 { "#222" } else { "#ddd" }).collect::<Vec<&str>>(),
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///     ),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn overlapping_circles<'a>(
    radius: f32,
    (width, height): (usize, usize),
    fill: Fill<'a>,
    background_color: &'a str,
) -> Document {
    assert_eq!(fill.opacity.len(), width * height);

    let mut doc = Document::new()
        .set("width", radius * width as f32)
        .set("height", radius * height as f32)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            doc = doc.add(
                Circle::new()
                    .set("cx", x as f32 * radius)
                    .set("cy", y as f32 * radius)
                    .set("r", radius)
                    .set("fill", fill.color[ix])
                    .set("style", format!("opacity:{};", fill.opacity[ix])),
            );

            if x == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", width as f32 * radius)
                        .set("cy", y as f32 * radius)
                        .set("r", radius)
                        .set("fill", fill.color[ix])
                        .set("style", format!("opacity:{};", fill.opacity[ix])),
                )
            }

            if y == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", x as f32 * radius)
                        .set("cy", height as f32 * radius)
                        .set("r", radius)
                        .set("fill", fill.color[ix])
                        .set("style", format!("opacity:{};", fill.opacity[ix])),
                )
            }

            if x == 0 && y == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", width as f32 * radius)
                        .set("cy", height as f32 * radius)
                        .set("r", radius)
                        .set("fill", fill.color[ix])
                        .set("style", format!("opacity:{};", fill.opacity[ix])),
                )
            }
        }
    }

    doc
}
