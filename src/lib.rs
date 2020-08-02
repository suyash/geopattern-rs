//! A crate for creating SVG geometric patterns

#![deny(missing_docs)]

use svg::node::element::{Circle, Group, Path, Polyline, Rectangle};
use svg::node::Value;
use svg::Document;

fn create_document<V>((width, height): (V, V), background_color: &str) -> Document
where
    V: Into<Value>,
{
    Document::new()
        .set("width", width)
        .set("height", height)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", background_color),
        )
}

/// chevrons
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/chevrons.svg)
///
/// ```
/// use geopattern::chevrons;
///
/// let c = chevrons(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#000", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn chevrons<'a>(
    chevron_width: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

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

    let mut doc = create_document(
        (
            chevron_width * width as f32,
            chevron_width * height as f32 * 0.66,
        ),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let g = Group::new()
                .set("fill", fill[ix].0)
                .set("fill-opacity", fill[ix].1)
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
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/concentric_circles.svg)
///
/// ```
/// use geopattern::concentric_circles;
///
/// let c = concentric_circles(
///     20.0,
///     4.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn concentric_circles<'a>(
    diameter: f32,
    concentric_width: f32,
    (width, height): (usize, usize),
    fill_outer: &'a [(&'a str, f32)],
    fill_inner: &'a [(&'a str, f32)],
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill_outer.len(), width * height);
    debug_assert_eq!(fill_inner.len(), width * height);

    let diameter = diameter + concentric_width;

    let mut doc = create_document(
        (diameter * width as f32, diameter * height as f32),
        background_color,
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
                    .set("stroke", fill_outer[ix].0)
                    .set(
                        "style",
                        format!(
                            "opacity:{}; stroke-width:0x{:X};",
                            fill_outer[ix].1,
                            (concentric_width as i64)
                        ),
                    ),
            );

            doc = doc.add(
                Circle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", diameter / 4.0)
                    .set("fill", fill_inner[ix].0)
                    .set("fill-opacity", fill_inner[ix].1),
            );
        }
    }

    doc
}

/// Cubic Disarray
///
/// https://generativeartistry.com/tutorials/cubic-disarray/
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/cubic_disarray.svg)
///
/// ```
/// use geopattern::cubic_disarray;
///
/// let c = cubic_disarray(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     (
///         &(0..4).map(|v| 0.02 + (v as f32) / 4.0).collect::<Vec<f32>>(),
///         &(0..4).map(|v| 0.02 + (v as f32 * std::f32::consts::PI) / 4.0).collect::<Vec<f32>>(),
///     ),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn cubic_disarray<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    (translate, rotate): (&'a [f32], &'a [f32]),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);
    debug_assert_eq!(translate.len(), width * height);
    debug_assert_eq!(rotate.len(), width * height);

    let mut doc = create_document(
        (side * width as f32, side * height as f32),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            doc = doc.add(
                Rectangle::new()
                    .set("x", (x as f32) * side)
                    .set("y", (y as f32) * side)
                    .set("width", side)
                    .set("height", side)
                    .set("fill", fill[ix].0)
                    .set("fill-opacity", fill[ix].1)
                    .set("stroke", stroke.0)
                    .set("stroke-opacity", stroke.1)
                    .set(
                        "transform",
                        format!(
                            "translate({} {}) rotate({} {} {})",
                            translate[ix],
                            0,
                            rotate[ix],
                            (x as f32) * side,
                            (y as f32) * side
                        ),
                    ),
            );
        }
    }

    doc
}

/// diamonds
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/diamonds.svg)
///
/// ```
/// use geopattern::diamonds;
///
/// let c = diamonds(
///     (20.0, 20.0),
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn diamonds<'a>(
    (diamond_width, diamond_height): (f32, f32),
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

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

    let mut doc = create_document(
        (
            diamond_width * width as f32,
            diamond_height * height as f32 / 2.0,
        ),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let polyline = Polyline::new()
                .set("points", points.as_str())
                .set("fill", fill[ix].0)
                .set("fill-opacity", fill[ix].1)
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
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/hexagons.svg)
///
/// ```
/// use geopattern::hexagons;
///
/// let c = hexagons(
///     20.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn hexagons<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

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

    let mut doc = create_document(
        (
            (hexagon_width + side) * width as f32 / 2.0,
            hexagon_height * height as f32,
        ),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let polyline = Polyline::new()
                .set("points", points.as_str())
                .set("fill", fill[ix].0)
                .set("fill-opacity", fill[ix].1)
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

/// Joy Division
///
/// https://generativeartistry.com/tutorials/joy-division/
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/joy_division.svg)
///
/// ```
/// use geopattern::joy_division;
///
/// let c = joy_division(
///     60.0,
///     (2, 2),
///     &(0..4).map(|v| v as f32).collect::<Vec<f32>>(),
///     ("#333", 0.5, 2.0),
///     2,
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn joy_division<'a>(
    step_size: f32,
    (width, height): (usize, usize),
    pulse_heights: &'a [f32],
    (stroke_color, stroke_opacity, stroke_width): (&'a str, f32, f32),
    padding_top: usize,
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(pulse_heights.len(), width * height);

    let mut doc = create_document(
        (
            width as f32 * step_size,
            (height + 1 + padding_top) as f32 * step_size,
        ),
        background_color,
    );

    for y in 0..height {
        let top = (y + 1 + padding_top) as f32 * step_size + stroke_width / 2.0;
        let mut path = format!("M {} {}", 0, top);

        let (mut pleft, mut ptop) = (0.0, top);

        for x in 0..width {
            let ix = y * width + x;

            let top = top + pulse_heights[ix];
            let left = (x + 1) as f32 * step_size;

            path = format!(
                "{} Q {} {} {} {}",
                path,
                pleft,
                ptop,
                (left + pleft) / 2.0,
                (top + ptop) / 2.0,
            );

            ptop = top;
            pleft = left;
        }

        path = format!(
            "{} Q {} {} {} {}",
            path,
            pleft,
            ptop,
            width as f32 * step_size,
            top
        );

        doc = doc.add(
            Path::new()
                .set("d", path)
                .set("fill", background_color)
                .set("stroke", stroke_color)
                .set("stroke-opacity", stroke_opacity)
                .set("stroke-width", stroke_width),
        );
    }

    doc
}

/// mosaic squares
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/mosaic_squares.svg)
///
/// ```
/// use geopattern::mosaic_squares;
///
/// let c = mosaic_squares(
///     20.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn mosaic_squares<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill_outer: &'a [(&'a str, f32)],
    fill_inner: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill_outer.len(), width * height);
    debug_assert_eq!(fill_inner.len(), width * height);

    let mut doc = create_document(
        (side * width as f32 * 2.0, side * height as f32 * 2.0),
        background_color,
    );

    let draw_outer_tile = |mut doc: Document, x: f32, y: f32, ix: usize| {
        let points = format!("0,0,{},{},0,{},0,0", side, side, side);

        let polyline = Polyline::new()
            .set("points", points)
            .set("fill", fill_outer[ix].0)
            .set("fill-opacity", fill_outer[ix].1)
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
            .set("fill", fill_outer[ix].0)
            .set("fill-opacity", fill_outer[ix].1)
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
            .set("fill", fill_inner[ix].0)
            .set("fill-opacity", fill_inner[ix].1)
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
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/nested_squares.svg)
///
/// TODO: make `outer_side` independent of `inner_side`, currently evaluates to `7 * inner_side`
///
/// ```
/// use geopattern::nested_squares;
///
/// let c = nested_squares(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn nested_squares<'a>(
    inner_side: f32,
    (width, height): (usize, usize),
    stroke_outer: &'a [(&'a str, f32)],
    fill_inner: &'a [(&'a str, f32)],
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill_inner.len(), width * height);
    debug_assert_eq!(stroke_outer.len(), width * height);

    let outer_side = inner_side * 7.0;

    let mut doc = create_document(
        (
            (inner_side * 2.0 + outer_side) * width as f32,
            (inner_side * 2.0 + outer_side) * height as f32,
        ),
        background_color,
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
                    .set("stroke", stroke_outer[ix].0)
                    .set(
                        "style",
                        format!(
                            "opacity:{};stroke-width:{};",
                            stroke_outer[ix].1, inner_side
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
                    .set("stroke", fill_inner[ix].0)
                    .set(
                        "style",
                        format!("opacity:{};stroke-width:{};", fill_inner[ix].1, inner_side),
                    ),
            );
        }
    }

    doc
}

/// octagons
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/octagons.svg)
///
/// ```
/// use geopattern::octagons;
///
/// let c = octagons(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn octagons<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

    let mut doc = create_document(
        (side * width as f32, side * height as f32),
        background_color,
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
                    .set("fill", fill[ix].0)
                    .set("fill-opacity", fill[ix].1)
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
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/overlapping_circles.svg)
///
/// ```
/// use geopattern::overlapping_circles;
///
/// let c = overlapping_circles(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn overlapping_circles<'a>(
    radius: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

    let mut doc = create_document(
        (radius * width as f32, radius * height as f32),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            doc = doc.add(
                Circle::new()
                    .set("cx", x as f32 * radius)
                    .set("cy", y as f32 * radius)
                    .set("r", radius)
                    .set("fill", fill[ix].0)
                    .set("style", format!("opacity:{};", fill[ix].1)),
            );

            if x == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", width as f32 * radius)
                        .set("cy", y as f32 * radius)
                        .set("r", radius)
                        .set("fill", fill[ix].0)
                        .set("style", format!("opacity:{};", fill[ix].1)),
                )
            }

            if y == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", x as f32 * radius)
                        .set("cy", height as f32 * radius)
                        .set("r", radius)
                        .set("fill", fill[ix].0)
                        .set("style", format!("opacity:{};", fill[ix].1)),
                )
            }

            if x == 0 && y == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", width as f32 * radius)
                        .set("cy", height as f32 * radius)
                        .set("r", radius)
                        .set("fill", fill[ix].0)
                        .set("style", format!("opacity:{};", fill[ix].1)),
                )
            }
        }
    }

    doc
}

/// overlapping rings
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/overlapping_rings.svg)
///
/// TODO: consider having an `outer_radius`?
///
/// ```
/// use geopattern::overlapping_rings;
///
/// let c = overlapping_rings(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn overlapping_rings<'a>(
    radius: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

    let mut doc = create_document(
        (radius * width as f32, radius * height as f32),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            doc = doc.add(
                Circle::new()
                    .set("cx", x as f32 * radius)
                    .set("cy", y as f32 * radius)
                    .set("r", radius - radius / 8.0)
                    .set("fill", "none")
                    .set("stroke", fill[ix].0)
                    .set(
                        "style",
                        format!("opacity:{};stroke-width:{};", fill[ix].1, radius / 4.0),
                    ),
            );

            if x == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", width as f32 * radius)
                        .set("cy", (y as f32) * radius)
                        .set("r", radius - radius / 8.0)
                        .set("fill", "none")
                        .set("stroke", fill[ix].0)
                        .set(
                            "style",
                            format!("opacity:{};stroke-width:{};", fill[ix].1, radius / 4.0),
                        ),
                )
            }

            if y == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", (x as f32) * radius)
                        .set("cy", height as f32 * radius)
                        .set("r", radius - radius / 8.0)
                        .set("fill", "none")
                        .set("stroke", fill[ix].0)
                        .set(
                            "style",
                            format!("opacity:{};stroke-width:{};", fill[ix].1, radius / 4.0),
                        ),
                )
            }

            if x == 0 && y == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", width as f32 * radius)
                        .set("cy", height as f32 * radius)
                        .set("r", radius - radius / 8.0)
                        .set("fill", "none")
                        .set("stroke", fill[ix].0)
                        .set(
                            "style",
                            format!("opacity:{};stroke-width:{};", fill[ix].1, radius / 4.0),
                        ),
                )
            }
        }
    }

    doc
}

/// plaid
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/plaid.svg)
///
/// ```
/// use geopattern::plaid;
///
/// let c = plaid(
///     &(0..4).map(|v| 5.0 + v as f32 * 4.0).collect::<Vec<f32>>(),
///     &(0..4).map(|v| 3.0 + v as f32 * 7.0).collect::<Vec<f32>>(),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn plaid<'a>(
    distances: &'a [f32],
    sizes: &'a [f32],
    fill: &'a [(&'a str, f32)],
    background_color: &'a str,
) -> Document {
    let n = distances.len();

    debug_assert_eq!(sizes.len(), n);
    debug_assert_eq!(fill.len(), n);

    let (mut w, mut h) = (0.0, 0.0);

    let mut doc = create_document((0, 0), background_color);

    for i in 0..n {
        h += distances[i];

        doc = doc.add(
            Rectangle::new()
                .set("x", 0)
                .set("y", h)
                .set("width", "100%")
                .set("height", sizes[i])
                .set("opacity", fill[i].1)
                .set("fill", fill[i].0),
        );

        h += sizes[i];
    }

    for i in 0..n {
        w += distances[i];

        doc = doc.add(
            Rectangle::new()
                .set("x", w)
                .set("y", 0)
                .set("width", sizes[i])
                .set("height", "100%")
                .set("opacity", fill[i].1)
                .set("fill", fill[i].0),
        );

        w += sizes[i];
    }

    doc.set("width", w).set("height", h)
}

fn plus(side: f32) -> (Rectangle, Rectangle) {
    (
        Rectangle::new()
            .set("x", side)
            .set("y", 0)
            .set("width", side)
            .set("height", side * 3.0),
        Rectangle::new()
            .set("x", 0)
            .set("y", side)
            .set("width", side * 3.0)
            .set("height", side),
    )
}

/// plus_signs
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/plus_signs.svg)
///
/// ```
/// use geopattern::plus_signs;
///
/// let c = plus_signs(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn plus_signs<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

    let mut doc = create_document(
        (side * 2.0 * width as f32, side * 2.0 * height as f32),
        background_color,
    );

    let length = side * 3.0;

    let rects = plus(side);

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let dx = (y % 2) as f32;

            let g = Group::new()
                .set("fill", fill[ix].0)
                .set("stroke", stroke.0)
                .set("stroke-opacity", stroke.1)
                .set("style", format!("fill-opacity:{};", fill[ix].1))
                .add(rects.0.clone())
                .add(rects.1.clone());

            doc = doc.add(g.clone().set(
                "transform",
                format!(
                    "translate({}, {})",
                    (x as f32) * (length - side) + dx * side - side,
                    (y as f32) * (length - side) - length / 2.0
                ),
            ));

            if x == 0 {
                doc = doc.add(g.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        4.0 * length - (x as f32) * side + dx * side - side,
                        (y as f32) * (length - side) - length / 2.0
                    ),
                ));
            }

            if y == 0 {
                doc = doc.add(g.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        (x as f32) * (length - side) + dx * side - side,
                        4.0 * length - (y as f32) * side - length / 2.0
                    ),
                ));
            }

            if x == 0 && y == 0 {
                doc = doc.add(g.clone().set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        4.0 * length - (x as f32) * side + dx * side - side,
                        4.0 * length - (y as f32) * side - length / 2.0
                    ),
                ));
            }
        }
    }

    doc
}

/// sine waves
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/sine_waves.svg)
///
/// ```
/// use geopattern::sine_waves;
///
/// let c = sine_waves(
///     120.0,
///     80.0,
///     20.0,
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn sine_waves<'a>(
    period: f32,
    a: f32,
    ww: f32,
    fill: &'a [(&'a str, f32)],
    background_color: &'a str,
) -> Document {
    let n = fill.len();

    let mut doc = create_document((period, ww * n as f32), background_color);

    for i in 0..n {
        let xoff = (period / 4.0) * 0.7;

        let path = Path::new()
            .set(
                "d",
                format!(
                    "M0 {} C {} 0, {} 0, {} {} S {} {}, {} {} S {} 0, {}, {}",
                    a,
                    xoff,
                    period / 2.0 - xoff,
                    period / 2.0,
                    a,
                    period - xoff,
                    a * 2.0,
                    period,
                    a,
                    period * 1.5 - xoff,
                    period * 1.5,
                    a
                ),
            )
            .set("fill", "none")
            .set("stroke", fill[i].0)
            .set(
                "style",
                format!("opacity:{};stroke-width:{};", fill[i].1, ww),
            );

        doc = doc.add(path.clone().set(
            "transform",
            format!(
                "translate(-{}, {})",
                period / 4.0,
                (ww * i as f32) - (a * 1.5)
            ),
        ));

        doc = doc.add(path.clone().set(
            "transform",
            format!(
                "translate(-{}, {})",
                period / 4.0,
                (ww * i as f32) - (a * 1.5) + ww * n as f32
            ),
        ));
    }

    doc
}

/// squares
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/squares.svg)
///
/// ```
/// use geopattern::squares;
///
/// let c = squares(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn squares<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

    let mut doc = create_document(
        (side * width as f32, side * height as f32),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            doc = doc.add(
                Rectangle::new()
                    .set("x", (x as f32) * side)
                    .set("y", (y as f32) * side)
                    .set("width", side)
                    .set("height", side)
                    .set("fill", fill[ix].0)
                    .set("fill-opacity", fill[ix].1)
                    .set("stroke", stroke.0)
                    .set("stroke-opacity", stroke.1),
            );
        }
    }

    doc
}

/// tesselation
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/tesselation.svg)
///
/// ```
/// use geopattern::tesselation;
///
/// let c = tesselation(
///     60.0,
///     &(0..20)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn tesselation<'a>(
    length: f32,
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), 20);

    let hex_width = length * 2.0;
    let hex_height = length * (3.0 as f32).sqrt();

    let tess_height = length / 2.0 * (3.0 as f32).sqrt();
    let points = format!("0,0,{},{},0,{},0,0", tess_height, length / 2.0, length);

    let tile_width = length * 3.0 + tess_height * 2.0;
    let tile_height = hex_height * 2.0 + length * 2.0;

    let mut doc = create_document((tile_width, tile_height), background_color);

    let polyline = |ix: usize| {
        Polyline::new()
            .set("points", points.as_str())
            .set("fill", fill[ix].0)
            .set("fill-opacity", fill[ix].1)
            .set("stroke", stroke.0)
            .set("stroke-opacity", stroke.1)
            .set("stroke-width", 1)
    };

    let rect = |ix: usize| {
        Rectangle::new()
            .set("fill", fill[ix].0)
            .set("fill-opacity", fill[ix].1)
            .set("stroke", stroke.0)
            .set("stroke-opacity", stroke.1)
            .set("stroke-width", 1)
    };

    // 0
    doc = doc.add(
        rect(0)
            .set("x", -length / 2.0)
            .set("y", -length / 2.0)
            .set("width", length)
            .set("height", length),
    );

    doc = doc.add(
        rect(0)
            .set("x", tile_width - length / 2.0)
            .set("y", -length / 2.0)
            .set("width", length)
            .set("height", length),
    );

    doc = doc.add(
        rect(0)
            .set("x", -length / 2.0)
            .set("y", tile_height - length / 2.0)
            .set("width", length)
            .set("height", length),
    );

    doc = doc.add(
        rect(0)
            .set("x", tile_width - length / 2.0)
            .set("y", tile_height - length / 2.0)
            .set("width", length)
            .set("height", length),
    );

    // 1
    doc = doc.add(
        rect(1)
            .set("x", hex_width / 2.0 + tess_height)
            .set("y", hex_height / 2.0)
            .set("width", length)
            .set("height", length),
    );

    // 2
    doc = doc.add(
        rect(2)
            .set("x", -length / 2.0)
            .set("y", tile_height / 2.0 - length / 2.0)
            .set("width", length)
            .set("height", length),
    );

    doc = doc.add(
        rect(2)
            .set("x", tile_width - length / 2.0)
            .set("y", tile_height / 2.0 - length / 2.0)
            .set("width", length)
            .set("height", length),
    );

    // 3
    doc = doc.add(
        rect(3)
            .set("x", hex_width / 2.0 + tess_height)
            .set("y", hex_height * 1.5 + length)
            .set("width", length)
            .set("height", length),
    );

    // 4
    doc = doc.add(polyline(4).set(
        "transform",
        format!(
            "translate({}, {}) rotate(0, {}, {})",
            length / 2.0,
            -length / 2.0,
            length / 2.0,
            tess_height / 2.0
        ),
    ));

    doc = doc.add(polyline(4).set(
        "transform",
        format!(
            "translate({}, {}) rotate(0, {}, {}) scale(1, -1)",
            length / 2.0,
            tile_height + length / 2.0,
            length / 2.0,
            tess_height / 2.0
        ),
    ));

    // 5
    doc = doc.add(polyline(5).set(
        "transform",
        format!(
            "translate({}, {}) rotate(0, {}, {}) scale(-1, 1)",
            tile_width - length / 2.0,
            -length / 2.0,
            length / 2.0,
            tess_height / 2.0
        ),
    ));

    doc = doc.add(polyline(5).set(
        "transform",
        format!(
            "translate({}, {}) rotate(0, {}, {}) scale(-1, -1)",
            tile_width - length / 2.0,
            tile_height + length / 2.0,
            length / 2.0,
            tess_height / 2.0
        ),
    ));

    // 6
    doc = doc.add(polyline(6).set(
        "transform",
        format!(
            "translate({}, {})",
            tile_width / 2.0 + length / 2.0,
            hex_height / 2.0
        ),
    ));

    // 7
    doc = doc.add(polyline(7).set(
        "transform",
        format!(
            "translate({}, {}) scale(-1, 1)",
            tile_width / 2.0 - length / 2.0,
            hex_height / 2.0
        ),
    ));

    // 8
    doc = doc.add(polyline(8).set(
        "transform",
        format!(
            "translate({}, {}) scale(1, -1)",
            tile_width / 2.0 + length / 2.0,
            tile_height - hex_height / 2.0
        ),
    ));

    // 9
    doc = doc.add(polyline(9).set(
        "transform",
        format!(
            "translate({}, {}) scale(-1, -1)",
            tile_width / 2.0 - length / 2.0,
            tile_height - hex_height / 2.0
        ),
    ));

    // 10
    doc = doc.add(polyline(10).set(
        "transform",
        format!(
            "translate({}, {})",
            length / 2.0,
            tile_height / 2.0 - length / 2.0
        ),
    ));

    // 11
    doc = doc.add(polyline(11).set(
        "transform",
        format!(
            "translate({}, {}) scale(-1, 1)",
            tile_width - length / 2.0,
            tile_height / 2.0 - length / 2.0
        ),
    ));

    // 12
    doc = doc.add(
        rect(12)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "translate({}, {}) rotate(-30, 0, 0)",
                    length / 2.0,
                    length / 2.0
                ),
            ),
    );

    // 13
    doc = doc.add(
        rect(13)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "scale(-1, 1) translate({}, {}) rotate(-30, 0, 0)",
                    -tile_width + length / 2.0,
                    length / 2.0
                ),
            ),
    );

    // 14
    doc = doc.add(
        rect(14)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "translate({}, {}) rotate(30, 0, {})",
                    length / 2.0,
                    tile_height / 2.0 - length / 2.0 - length,
                    length
                ),
            ),
    );

    // 15
    doc = doc.add(
        rect(15)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "scale(-1, 1) translate({}, {}) rotate(30, 0, {})",
                    -tile_width + length / 2.0,
                    tile_height / 2.0 - length / 2.0 - length,
                    length
                ),
            ),
    );

    // 16
    doc = doc.add(
        rect(16)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "scale(1, -1) translate({}, {}) rotate(30, 0, {})",
                    length / 2.0,
                    -tile_height / 2.0 - length / 2.0 - length,
                    length
                ),
            ),
    );

    // 17
    doc = doc.add(
        rect(17)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "scale(-1, -1) translate({}, {}) rotate(30, 0, {})",
                    -tile_width + length / 2.0,
                    -tile_height / 2.0 - length / 2.0 - length,
                    length
                ),
            ),
    );

    // 18
    doc = doc.add(
        rect(18)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "scale(1, -1) translate({}, {}) rotate(-30, 0, 0)",
                    length / 2.0,
                    -tile_height + length / 2.0
                ),
            ),
    );

    // 19
    doc = doc.add(
        rect(19)
            .set("x", 0)
            .set("y", 0)
            .set("width", length)
            .set("height", length)
            .set(
                "transform",
                format!(
                    "scale(-1, -1) translate({}, {}) rotate(-30, 0, 0)",
                    -tile_width + length / 2.0,
                    -tile_height + length / 2.0
                ),
            ),
    );

    doc
}

/// tiled lines
///
/// https://generativeartistry.com/tutorials/tiled-lines/
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/tiled_lines.svg)
///
/// ```
/// use geopattern::tiled_lines;
///
/// let c = tiled_lines(
///     80,
///     (2, 2),
///     &(0..4).map(|i| i & 1 == 0).collect::<Vec<bool>>(),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     5.0,
///     "#FFFFFF",
/// );
///
/// println!("{}", c);
/// ```
pub fn tiled_lines<'a>(
    step_size: usize,
    (width, height): (usize, usize),
    ltr: &'a [bool],
    stroke: &'a [(&'a str, f32)],
    stroke_width: f32,
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(ltr.len(), width * height);
    debug_assert_eq!(ltr.len(), stroke.len());

    let mut doc = create_document((step_size * width, step_size * height), background_color);

    for x in 0..width {
        for y in 0..height {
            let ix = x * height + y;

            let (x, y) = (x * step_size, y * step_size);

            let path = if ltr[ix] {
                format!("M {} {} L {} {}", x, y, x + step_size, y + step_size)
            } else {
                format!("M {} {} L {} {}", x + step_size, y, x, y + step_size)
            };

            doc = doc.add(
                Path::new()
                    .set("d", path)
                    .set("fill", "none")
                    .set("stroke", stroke[ix].0)
                    .set("stroke-opacity", stroke[ix].1)
                    .set("stroke-width", stroke_width)
                    .set("stroke-linecap", "square"),
            );
        }
    }

    doc
}

/// triangles
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/triangles.svg)
///
/// ```
/// use geopattern::triangles;
///
/// let c = triangles(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn triangles<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    stroke: (&'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

    let triangle_height = (3.0 as f32).sqrt() * side / 2.0;
    let points = format!(
        "{},0,{},{},0,{},{},0",
        side / 2.0,
        side,
        triangle_height,
        triangle_height,
        side / 2.0
    );

    let mut doc = create_document(
        (side / 2.0 * width as f32, triangle_height * height as f32),
        background_color,
    );

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let rot = match (x, y) {
                (x, y) if y % 2 == 0 && x % 2 == 0 => 180,
                (x, y) if y % 2 != 0 && x % 2 != 0 => 180,
                _ => 0,
            };

            let p = Polyline::new()
                .set("points", points.as_str())
                .set("fill", fill[ix].0)
                .set("fill-opacity", fill[ix].1)
                .set("stroke", stroke.0)
                .set("stroke-opacity", stroke.1);

            doc = doc.add(p.clone().set(
                "transform",
                format!(
                    "translate({}, {}) rotate({}, {}, {})",
                    (x as f32) * side * 0.5 - side / 2.0,
                    triangle_height * (y as f32),
                    rot,
                    side / 2.0,
                    triangle_height / 2.0
                ),
            ));

            if x == 0 {
                doc = doc.add(p.set(
                    "transform",
                    format!(
                        "translate({}, {}) rotate({}, {}, {})",
                        width as f32 * side * 0.5 - side / 2.0,
                        triangle_height * (y as f32),
                        rot,
                        side / 2.0,
                        triangle_height / 2.0
                    ),
                ));
            }
        }
    }

    doc
}

/// Triangular Mesh
///
/// https://generativeartistry.com/tutorials/triangular-mesh/
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/triangular_mesh.svg)
///
/// `fill` has required size `(4 * width - 2) * height`
///
/// `entropy` has required size `width * (height + 1)`
///
/// ```
/// use geopattern::triangular_mesh;
///
/// let c = triangular_mesh(
///     60.0,
///     (2, 2),
///     &(0..6).map(|v| (0.02 + (v as f32) / 4.0, 0.02 + (v as f32) / 4.0)).collect::<Vec<(f32, f32)>>(),
///     &(0..12).map(|v| ("#ddd", 0.2)).collect::<Vec<(&str, f32)>>(),
///     (1.0, "#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn triangular_mesh<'a>(
    side: f32,
    (width, height): (usize, usize),
    entropy: &'a [(f32, f32)],
    fill: &'a [(&'a str, f32)],
    stroke: (f32, &'a str, f32),
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), (4 * width - 2) * height);
    debug_assert_eq!(entropy.len(), width * (height + 1));

    let mut doc = create_document(
        (side * width as f32, side * (height + 1) as f32),
        background_color,
    );

    let mut lines = Vec::new();

    for y in 0..height {
        let mut line = Vec::new();

        for x in 0..2 * width {
            let xx = x / 2;

            let e1 = entropy[y * width + xx];
            let e2 = entropy[(y + 1) * width + xx];

            if y & 1 == 0 {
                line.push((
                    side / 4.0 + xx as f32 * side + e2.0,
                    side / 2.0 + (y + 1) as f32 * side + e2.1,
                ));

                line.push((
                    side / 4.0 + side / 2.0 + xx as f32 * side + e1.0,
                    side / 2.0 + y as f32 * side + e1.1,
                ));
            } else {
                line.push((
                    side / 4.0 + xx as f32 * side + e1.0,
                    side / 2.0 + y as f32 * side + e1.1,
                ));

                line.push((
                    side / 4.0 + side / 2.0 + xx as f32 * side + e2.0,
                    side / 2.0 + (y + 1) as f32 * side + e2.1,
                ));
            }
        }

        lines.push(line);
    }

    for (j, line) in lines.iter().enumerate() {
        for i in 0..line.len() - 2 {
            doc = doc.add(
                Path::new()
                    .set(
                        "d",
                        format!(
                            "M {} {} L {} {} L {} {} Z",
                            line[i].0,
                            line[i].1,
                            line[i + 1].0,
                            line[i + 1].1,
                            line[i + 2].0,
                            line[i + 2].1
                        ),
                    )
                    .set("fill", fill[j * (line.len() - 2) + i].0)
                    .set("fill-opacity", fill[j * (line.len() - 2) + i].1)
                    .set("stroke", stroke.1)
                    .set("stroke-width", stroke.0)
                    .set("stroke-opacity", stroke.2)
                    .set("stroke-linejoin", "bevel"),
            );
        }
    }

    doc
}

/// Un Deus Trois
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/un_deus_trois.svg)
///
/// ```
/// use geopattern::un_deus_trois;
///
/// let c = un_deus_trois(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     ("#ddd", 0.2),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn un_deus_trois<'a>(
    step_size: f32,
    (width, height): (usize, usize),
    stroke: &'a [(&'a str, f32, f32)],
    rotation: &'a [f32],
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(rotation.len(), 3 * width * height);
    debug_assert_eq!(stroke.len(), 3 * width * height);

    let mut doc = create_document(
        (step_size * width as f32, step_size * height as f32 * 3.0),
        background_color,
    );

    // Un
    for y in 0..height {
        for x in 0..width {
            let (stroke, stroke_width, stroke_opacity) = stroke[y * width + x];

            doc = doc.add(
                Path::new()
                    .set(
                        "d",
                        format!(
                            "M {} {} L {} {}",
                            (x as f32 + 0.5) * step_size,
                            y as f32 * step_size,
                            (x as f32 + 0.5) * step_size,
                            (y + 1) as f32 * step_size
                        ),
                    )
                    .set("stroke", stroke)
                    .set("stroke-width", stroke_width)
                    .set("stroke-opacity", stroke_opacity)
                    .set(
                        "transform",
                        format!(
                            "rotate({} {} {})",
                            rotation[y * width + x],
                            (x as f32 + 0.5) * step_size,
                            (y as f32 + 0.5) * step_size
                        ),
                    ),
            );
        }
    }

    // Deux
    for y in 0..height {
        for x in 0..width {
            let (stroke, stroke_width, stroke_opacity) = stroke[(height + y) * width + x];

            doc = doc.add(
                Path::new()
                    .set(
                        "d",
                        format!(
                            "M {} {} L {} {} M {} {} L {} {}",
                            (x as f32 + 0.2) * step_size,
                            (height + y) as f32 * step_size,
                            (x as f32 + 0.2) * step_size,
                            (height + y + 1) as f32 * step_size,
                            (x as f32 + 0.8) * step_size,
                            (height + y) as f32 * step_size,
                            (x as f32 + 0.8) * step_size,
                            (height + y + 1) as f32 * step_size
                        ),
                    )
                    .set("stroke", stroke)
                    .set("stroke-width", stroke_width)
                    .set("stroke-opacity", stroke_opacity)
                    .set(
                        "transform",
                        format!(
                            "rotate({} {} {})",
                            rotation[(height + y) * width + x],
                            (x as f32 + 0.5) * step_size,
                            ((height + y) as f32 + 0.5) * step_size
                        ),
                    ),
            );
        }
    }

    // Trois
    for y in 0..height {
        for x in 0..width {
            let (stroke, stroke_width, stroke_opacity) = stroke[(2 * height + y) * width + x];

            doc = doc.add(
                Path::new()
                    .set(
                        "d",
                        format!(
                            "M {} {} L {} {} M {} {} L {} {} M {} {} L {} {}",
                            (x as f32 + 0.1) * step_size,
                            (2 * height + y) as f32 * step_size,
                            (x as f32 + 0.1) * step_size,
                            (2 * height + y + 1) as f32 * step_size,
                            (x as f32 + 0.5) * step_size,
                            (2 * height + y) as f32 * step_size,
                            (x as f32 + 0.5) * step_size,
                            (2 * height + y + 1) as f32 * step_size,
                            (x as f32 + 0.9) * step_size,
                            (2 * height + y) as f32 * step_size,
                            (x as f32 + 0.9) * step_size,
                            (2 * height + y + 1) as f32 * step_size
                        ),
                    )
                    .set("stroke", stroke)
                    .set("stroke-width", stroke_width)
                    .set("stroke-opacity", stroke_opacity)
                    .set(
                        "transform",
                        format!(
                            "rotate({} {} {})",
                            rotation[(2 * height + y) * width + x],
                            (x as f32 + 0.5) * step_size,
                            ((2 * height + y) as f32 + 0.5) * step_size
                        ),
                    ),
            );
        }
    }

    doc
}

/// xes
///
/// ![](https://raw.githubusercontent.com/suyash/geopattern-rs/master/examples/readme/xes.svg)
///
/// ```
/// use geopattern::xes;
///
/// let c = xes(
///     60.0,
///     (2, 2),
///     &(0..4)
///         .map(|v| {
///             (
///                 if v & 1 == 0 { "#222" } else { "#ddd" },
///                 0.02 + (v as f32) / 4.0,
///             )
///         })
///         .collect::<Vec<(&str, f32)>>(),
///     "#987987",
/// );
///
/// println!("{}", c);
/// ```
pub fn xes<'a>(
    side: f32,
    (width, height): (usize, usize),
    fill: &'a [(&'a str, f32)],
    background_color: &'a str,
) -> Document {
    debug_assert_eq!(fill.len(), width * height);

    let x_side = side * 3.0 * 0.943;

    let mut doc = create_document(
        (x_side / 2.0 * width as f32, x_side / 2.0 * height as f32),
        background_color,
    );

    let rects = plus(side);

    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;

            let dy = match x % 2 {
                1 => (y as f32) * x_side - x_side * 0.5 + x_side / 4.0,
                _ => (y as f32) * x_side - x_side * 0.5,
            };

            let group = Group::new()
                .set("fill", fill[ix].0)
                .set("style", format!("opacity:{};", fill[ix].1))
                .add(rects.0.clone())
                .add(rects.1.clone());

            doc = doc.add(group.clone().set(
                "transform",
                format!(
                    "translate({}, {}) rotate(45, {}, {})",
                    (x as f32) * x_side / 2.0 - x_side / 2.0,
                    dy - (y as f32) * x_side / 2.0,
                    x_side / 2.0,
                    x_side / 2.0
                ),
            ));

            if x == 0 {
                doc = doc.add(group.clone().set(
                    "transform",
                    format!(
                        "translate({}, {}) rotate(45, {}, {})",
                        width as f32 * x_side / 2.0 - x_side / 2.0,
                        dy - (y as f32) * x_side / 2.0,
                        x_side / 2.0,
                        x_side / 2.0
                    ),
                ));
            }

            if y == 0 {
                let dy = match x % 2 {
                    1 => width as f32 * x_side - x_side * 0.5,
                    _ => height as f32 * x_side - x_side * 0.5 + x_side * 0.25,
                };

                doc = doc.add(group.clone().set(
                    "transform",
                    format!(
                        "translate({}, {}) rotate(45, {}, {})",
                        (x as f32) * x_side / 2.0 - x_side / 2.0,
                        dy - (y as f32) * x_side / 2.0,
                        x_side / 2.0,
                        x_side / 2.0
                    ),
                ));
            }

            if y == height - 1 {
                doc = doc.add(group.clone().set(
                    "transform",
                    format!(
                        "translate({}, {}) rotate(45, {}, {})",
                        (x as f32) * x_side / 2.0 - x_side / 2.0,
                        dy - (2 * height - 1) as f32 * x_side / 2.0,
                        x_side / 2.0,
                        x_side / 2.0
                    ),
                ));
            }

            if x == 0 && y == 0 {
                doc = doc.add(group.clone().set(
                    "transform",
                    format!(
                        "translate({}, {}) rotate(45, {}, {})",
                        width as f32 * x_side / 2.0 - x_side / 2.0,
                        dy - height as f32 * x_side / 2.0,
                        x_side / 2.0,
                        x_side / 2.0
                    ),
                ));
            }
        }
    }

    doc
}
