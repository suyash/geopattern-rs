use sha1::{Digest, Sha1};
use svg::save;

use geopattern::{
    chevrons, concentric_circles, diamonds, hexagons, joy_division, mosaic_squares, nested_squares,
    octagons, overlapping_circles, overlapping_rings, plaid, plus_signs, sine_waves, squares,
    tesselation, tiled_lines, triangles, xes,
};

fn main() -> anyhow::Result<()> {
    let digest = Sha1::digest(b"geopattern");

    write_chevrons(&digest)?;
    write_concentric_circles(&digest)?;
    write_diamonds(&digest)?;
    write_hexagons(&digest)?;
    write_joy_division(&digest)?;
    write_mosaic_squares(&digest)?;
    write_nested_squares(&digest)?;
    write_octagons(&digest)?;
    write_overlapping_circles(&digest)?;
    write_overlapping_rings(&digest)?;
    write_plaid(&digest)?;
    write_plus_signs(&digest)?;
    write_sine_waves(&digest)?;
    write_squares(&digest)?;
    write_tesselation(&digest)?;
    write_tiled_lines(&digest)?;
    write_triangles(&digest)?;
    write_xes(&digest)?;

    Ok(())
}

fn write_chevrons(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/chevrons.svg",
        &chevrons(
            60.0,
            (4, 4),
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            ("#000", 0.02),
            "#998877",
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            (
                &(4..20)
                    .rev()
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(4..20)
                    .rev()
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            &format!("rgb({},{},{})", digest[4], digest[5], digest[6]),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            ("#000", 0.02),
            &format!("rgb({},{},{})", digest[6], digest[7], digest[8]),
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
                (digest[ix % 20] as f32 / 255.0) * (variance as f32 * step_size / 5.0) * -1.0;
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            (
                &(4..20)
                    .rev()
                    .map(|i| if digest[i] & 1 == 0 { "#222" } else { "#ddd" })
                    .collect::<Vec<&str>>(),
                &(4..20)
                    .rev()
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            (
                &(4..20)
                    .rev()
                    .map(|i| if digest[i] & 1 == 0 { "#222" } else { "#ddd" })
                    .collect::<Vec<&str>>(),
                &(4..20)
                    .rev()
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            &format!("rgb({},{},{})", digest[15], digest[16], digest[17]),
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
            (
                &(0..19)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..19)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(2..18)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(2..18)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &(0..20)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..20)
                    .map(|i| 0.1 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
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
            (
                &colors.iter().map(|s| s as &str).collect::<Vec<&str>>(),
                &(0..64).map(|_| 0.75).collect::<Vec<f32>>(),
            ),
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
            (
                &(2..18)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(2..18)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            ("#ddd", 0.02),
            &format!("rgb({},{},{})", digest[2], digest[8], digest[16]),
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
            (
                &(2..18)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(2..18)
                    .map(|i| 0.1 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            &format!("rgb({},{},{})", digest[8], digest[12], digest[16]),
        ),
    )?;

    Ok(())
}
