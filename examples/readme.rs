use sha1::{Digest, Sha1};
use svg::save;

use geopattern::{
    chevrons, concentric_circles, diamonds, hexagons, mosaic_squares, nested_squares, octagons,
    overlapping_circles, Fill,
};

fn main() -> anyhow::Result<()> {
    let digest = Sha1::digest(b"geopattern");

    write_chevrons(&digest)?;
    write_concentric_circles(&digest)?;
    write_diamonds(&digest)?;
    write_hexagons(&digest)?;
    write_mosaic_squares(&digest)?;
    write_nested_squares(&digest)?;
    write_octagons(&digest)?;
    write_overlapping_circles(&digest)?;

    Ok(())
}

fn write_chevrons(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/chevrons.svg",
        &chevrons(
            60.0,
            (4, 4),
            Fill::new(
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
            Fill::new(
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.2) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            Fill::new(
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
            Fill::new(
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
            Fill::new(
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

fn write_mosaic_squares(digest: &[u8]) -> anyhow::Result<()> {
    save(
        "examples/readme/mosaic_squares.svg",
        &mosaic_squares(
            30.0,
            (4, 4),
            Fill::new(
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            Fill::new(
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
            Fill::new(
                &(0..16)
                    .map(|i| if digest[i] & 1 == 0 { "#ddd" } else { "#222" })
                    .collect::<Vec<&str>>(),
                &(0..16)
                    .map(|i| 0.02 + (digest[i] as f32 * 0.4) / 255.0)
                    .collect::<Vec<f32>>(),
            ),
            Fill::new(
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
            Fill::new(
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
            Fill::new(
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
