use rand::{rngs::SmallRng, Rng, SeedableRng};
use sha1::{Digest, Sha1};
use svg::save;

use geopattern::{
    chevrons, circle_packing, concentric_circles, cubic_disarray, diamonds, hexagons,
    hypnotic_squares, joy_division, mosaic_squares, nested_squares, octagons, overlapping_circles,
    overlapping_rings, piet_mondrian, plaid, plus_signs, sine_waves, squares, tesselation,
    tiled_lines, triangles, triangular_mesh, un_deus_trois, xes, PietMondrianSplitType,
};

fn main() -> anyhow::Result<()> {
    let digest = Sha1::digest(b"geopattern");

    write_chevrons(&digest)?;
    write_circle_packing(&digest)?;
    write_concentric_circles(&digest)?;
    write_cubic_disarray(&digest)?;
    write_diamonds(&digest)?;
    write_hexagons(&digest)?;
    write_hypnotic_squares(&digest)?;
    write_joy_division(&digest)?;
    write_mosaic_squares(&digest)?;
    write_nested_squares(&digest)?;
    write_octagons(&digest)?;
    write_overlapping_circles(&digest)?;
    write_overlapping_rings(&digest)?;
    write_piet_mondrian(&digest)?;
    write_plaid(&digest)?;
    write_plus_signs(&digest)?;
    write_sine_waves(&digest)?;
    write_squares(&digest)?;
    write_tesselation(&digest)?;
    write_tiled_lines(&digest)?;
    write_triangles(&digest)?;
    write_triangular_mesh(&digest)?;
    write_un_deus_trois(&digest)?;
    write_xes(&digest)?;

    Ok(())
}

fn write_chevrons(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/chevrons.svg",
        &chevrons(
            60.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#000", 0.02),
            "#998877",
        ),
    )?;

    Ok(())
}

fn write_circle_packing(digest: &[u8]) -> anyhow::Result<()> {
    let mut rng =
        SmallRng::seed_from_u64(digest[7] as u64 * 256 + digest[8] as u64 * 16 + digest[9] as u64);
    let (w, h) = (300.0, 300.0);

    let mut points = Vec::new();
    for _ in 0..500 {
        points.push((rng.gen_range(0.0, w), rng.gen_range(0.0, h)));
    }

    let mut colors = Vec::new();
    for i in 0..500 {
        colors.push(format!(
            "rgb({},{},{})",
            digest[i % 20],
            digest[(2 * i) % 20],
            digest[(3 * i) % 20]
        ));
    }

    save(
        "examples/readme/circle_packing.svg",
        &circle_packing(
            &points,
            (2.0, 200.0),
            (w, h),
            &(0..500)
                .map(|ix| (colors[ix].as_str(), digest[ix % 20] as f32 / 255.0))
                .collect::<Vec<(&str, f32)>>(),
            ("#ddd", 1.0, 0.5),
            "#FFF",
        ),
    )?;

    Ok(())
}

fn write_concentric_circles(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/concentric_circles.svg",
        &concentric_circles(
            30.0,
            8.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &(4..20)
                .rev()
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &format!("rgb({},{},{})", digest[4], digest[5], digest[6]),
        ),
    )?;

    Ok(())
}

fn write_cubic_disarray(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/cubic_disarray.svg",
        &cubic_disarray(
            48.0,
            (8, 8),
            &(0..64).map(|_| ("#FFF", 1.0)).collect::<Vec<(&str, f32)>>(),
            ("#333", 1.0),
            (
                &(0..64)
                    .map(|i| {
                        ((i / 8) as f32
                            * digest[i % 20] as f32
                            * 1.5
                            * (((digest[i % 20] & 1) * 2) as f32 - 1.0))
                            / 255.0
                    })
                    .collect::<Vec<f32>>(),
                &(0..64)
                    .map(|i| {
                        ((i / 8) as f32
                            * digest[i % 20] as f32
                            * std::f32::consts::PI
                            * 1.5
                            * (((digest[i % 20] & 1) * 2) as f32 - 1.0))
                            / 255.0
                    })
                    .collect::<Vec<f32>>(),
            ),
            &format!("rgb({},{},{})", 200, 200, 200),
        ),
    )?;

    Ok(())
}

fn write_diamonds(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/diamonds.svg",
        &diamonds(
            (60.0, 60.0),
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#000", 0.02),
            &format!("rgb({},{},{})", digest[5], digest[6], digest[7]),
        ),
    )?;

    Ok(())
}

fn write_hexagons(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/hexagons.svg",
        &hexagons(
            24.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#000", 0.02),
            &format!("rgb({},{},{})", digest[6], digest[7], digest[8]),
        ),
    )?;

    Ok(())
}

fn write_hypnotic_squares(digest: &[u8]) -> anyhow::Result<()> {
    let colors: Vec<String> = (2..18)
        .map(|i| {
            format!(
                "rgb({},{},{})",
                128 + digest[i % 20] / 2,
                128 + digest[2 * i % 20] / 2,
                128 + digest[3 * i % 20] / 2
            )
        })
        .collect();

    save(
        "examples/readme/hypnotic_squares.svg",
        &hypnotic_squares(
            72.0,
            36.0,
            6,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        (digest[i] % 3) as isize - 1,
                        (digest[2 * i % 20] % 3) as isize - 1,
                    )
                })
                .collect::<Vec<(isize, isize)>>(),
            &(0..16)
                .map(|i| (colors[i].as_str(), 1.0, 1.0))
                .collect::<Vec<(&str, f32, f32)>>(),
            "#333",
        ),
    )?;

    Ok(())
}

fn write_joy_division(digest: &[u8]) -> anyhow::Result<()> {
    let (width, height, step_size) = (26, 13, 16.0);
    let mut pulse_heights = vec![0.0; width * height];
    for y in 0..height {
        for x in 0..width {
            let ix = y * width + x;
            let variance = if (x + 1) > width / 2 {
                width - (x + 1)
            } else {
                x + 1
            };
            let variance = if variance < width / 4 { 0 } else { variance };
            pulse_heights[ix] =
                (digest[ix % 20] as f32 / 255.0) * (variance as f32 * step_size / 4.0) * -1.0;
        }
    }

    save(
        "examples/readme/joy_division.svg",
        &joy_division(
            step_size,
            (width, height),
            &pulse_heights,
            ("#FFF", 0.75, 2.5),
            2,
            &format!("rgb({},{},{})", digest[4], digest[5], digest[6]),
        ),
    )?;

    Ok(())
}

fn write_mosaic_squares(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/mosaic_squares.svg",
        &mosaic_squares(
            30.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &(4..20)
                .rev()
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#222" } else { "#ddd" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#000", 0.02),
            &format!("rgb({},{},{})", digest[8], digest[9], digest[10]),
        ),
    )?;

    Ok(())
}

fn write_nested_squares(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/nested_squares.svg",
        &nested_squares(
            4.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &(4..20)
                .rev()
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#222" } else { "#ddd" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &format!("rgb({},{},{})", digest[14], digest[15], digest[16]),
        ),
    )?;

    Ok(())
}

fn write_octagons(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/octagons.svg",
        &octagons(
            24.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#ddd", 0.02),
            "#444",
        ),
    )?;

    Ok(())
}

fn write_overlapping_circles(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/overlapping_circles.svg",
        &overlapping_circles(
            40.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &format!("rgb({},{},{})", digest[15], digest[14], digest[13]),
        ),
    )?;

    Ok(())
}

fn write_overlapping_rings(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/overlapping_rings.svg",
        &overlapping_rings(
            40.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &format!("rgb({},{},{})", digest[15], digest[16], digest[17]),
        ),
    )?;

    Ok(())
}

fn write_piet_mondrian(_digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/piet_mondrian.svg",
        &piet_mondrian(
            &[
                (PietMondrianSplitType::X, 80.0),
                (PietMondrianSplitType::Y, 220.0),
            ],
            (300.0, 300.0),
            &[("#FFF", 1.0), ("#00F", 1.0), ("#F00", 1.0), ("#FFF", 1.0)],
            ("#222", 8.0, 1.0),
            "#FFF",
        ),
    )?;

    Ok(())
}

fn write_plaid(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/plaid.svg",
        &plaid(
            &(0..19)
                .map(|v| 5.0 + (digest[v] as f32 * 8.0) / 255.0)
                .collect::<Vec<f32>>(),
            &(1..20)
                .map(|v| 5.0 + (digest[v] as f32 * 8.0) / 255.0)
                .collect::<Vec<f32>>(),
            &(0..19)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &format!("rgb({},{},{})", digest[14], digest[16], digest[17]),
        ),
    )?;

    Ok(())
}

fn write_plus_signs(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/plus_signs.svg",
        &plus_signs(
            24.0,
            (4, 4),
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#ddd", 0.02),
            &format!("rgb({},{},{})", digest[12], digest[16], digest[17]),
        ),
    )?;

    Ok(())
}

fn write_sine_waves(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/sine_waves.svg",
        &sine_waves(
            300.0,
            80.0,
            10.0,
            &(0..16)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &format!("rgb({},{},{})", digest[10], digest[13], digest[14]),
        ),
    )?;

    Ok(())
}

fn write_squares(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/squares.svg",
        &squares(
            48.0,
            (4, 4),
            &(2..18)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#ddd", 0.02),
            &format!("rgb({},{},{})", digest[8], digest[10], digest[14]),
        ),
    )?;

    Ok(())
}

fn write_tesselation(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/tesselation.svg",
        &tesselation(
            48.0,
            &(0..20)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.1 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#ddd", 0.02),
            &format!("rgb({},{},{})", digest[6], digest[4], digest[14]),
        ),
    )?;

    Ok(())
}

fn write_tiled_lines(digest: &[u8]) -> anyhow::Result<()> {
    let colors = (0..64)
        .map(|i| {
            format!(
                "rgb({},{},{})",
                digest[i % 20],
                digest[(i + 1) % 20],
                digest[(i + 2) % 20]
            )
        })
        .collect::<Vec<String>>();

    save(
        "examples/readme/tiled_lines.svg",
        &tiled_lines(
            48,
            (8, 8),
            &(0..64)
                .map(|v| digest[v % 20] & 1 == 0)
                .collect::<Vec<bool>>(),
            &(0..64)
                .map(|v| (colors[v].as_str(), 0.75))
                .collect::<Vec<(&str, f32)>>(),
            5.0,
            "#222",
        ),
    )?;

    Ok(())
}

fn write_triangles(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/triangles.svg",
        &triangles(
            72.0,
            (4, 4),
            &(2..18)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.02 + (digest[i] as f32 * 0.4) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            ("#ddd", 0.02),
            &format!("rgb({},{},{})", digest[2], digest[8], digest[16]),
        ),
    )?;

    Ok(())
}

fn write_triangular_mesh(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/triangular_mesh.svg",
        &triangular_mesh(
            72.0,
            (4, 4),
            &(0..20)
                .map(|i| {
                    (
                        ((digest[i % 20] as f32 / 255.0) * 0.8 - 0.4) * 72.0 / 2.0,
                        ((digest[(i * 3) % 20] as f32 / 255.0) * 0.8 - 0.4) * 72.0 / 2.0,
                    )
                })
                .collect::<Vec<(f32, f32)>>(),
            &(0..56)
                .map(|i| ("#888", digest[i % 20] as f32 / 255.0))
                .collect::<Vec<(&str, f32)>>(),
            (2.0, "#222", 0.8),
            "#222",
        ),
    )?;

    Ok(())
}

fn write_un_deus_trois(digest: &[u8]) -> anyhow::Result<()> {
    let colors = (0..144)
        .map(|i| {
            format!(
                "rgb({},{},{})",
                128 + digest[i % 20] / 2,
                128 + digest[(2 * i) % 20] / 2,
                128 + digest[(3 * i) % 20] / 2
            )
        })
        .collect::<Vec<String>>();

    save(
        "examples/readme/un_deus_trois.svg",
        &un_deus_trois(
            32.0,
            (12, 4),
            &(0..144)
                .map(|i| {
                    (
                        colors[i].as_str(),
                        4.0,
                        0.25 + (digest[i % 20] as f32 * 0.75) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32, f32)>>(),
            &(0..144)
                .map(|x| (digest[x % 20] as f32 / 255.0) * 180.0 - 90.0)
                .collect::<Vec<f32>>(),
            "#141414",
        ),
    )?;

    Ok(())
}

fn write_xes(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/xes.svg",
        &xes(
            48.0,
            (4, 4),
            &(2..18)
                .map(|i| {
                    (
                        if digest[i] & 1 == 0 { "#ddd" } else { "#222" },
                        0.1 + (digest[i] as f32 * 0.2) / 255.0,
                    )
                })
                .collect::<Vec<(&str, f32)>>(),
            &format!("rgb({},{},{})", digest[8], digest[12], digest[16]),
        ),
    )?;

    Ok(())
}
