use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg::Document;

use geopattern::{
    chevrons, concentric_circles, diamonds, hexagons, mosaic_squares, nested_squares, octagons,
    overlapping_circles, Fill,
};

fn chevrons_bench(c: &mut Criterion) {
    c.bench_function("chevrons", |b| {
        b.iter(|| {
            let _document: Document = chevrons(
                black_box(60.0),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box(("#000", 0.02)),
                black_box("#998877"),
            );
        })
    });
}

fn concentric_circles_bench(c: &mut Criterion) {
    c.bench_function("concentric_circles", |b| {
        b.iter(|| {
            let _document: Document = concentric_circles(
                black_box(60.0),
                black_box(20.0),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box("#998877"),
            );
        })
    });
}

fn diamonds_bench(c: &mut Criterion) {
    c.bench_function("diamonds", |b| {
        b.iter(|| {
            let _document: Document = diamonds(
                black_box((60.0, 60.0)),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box(("#ddd", 0.2)),
                black_box("#998877"),
            );
        })
    });
}

fn hexagons_bench(c: &mut Criterion) {
    c.bench_function("hexagons", |b| {
        b.iter(|| {
            let _document: Document = hexagons(
                black_box(60.0),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box(("#ddd", 0.2)),
                black_box("#998877"),
            );
        })
    });
}

fn mosaic_squares_bench(c: &mut Criterion) {
    c.bench_function("mosaic_squares", |b| {
        b.iter(|| {
            let _document: Document = mosaic_squares(
                black_box(60.0),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box(("#ddd", 0.2)),
                black_box("#998877"),
            );
        })
    });
}

fn nested_squares_bench(c: &mut Criterion) {
    c.bench_function("nested_squares", |b| {
        b.iter(|| {
            let _document: Document = nested_squares(
                black_box(60.0),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box("#998877"),
            );
        })
    });
}

fn octagons_bench(c: &mut Criterion) {
    c.bench_function("octagons", |b| {
        b.iter(|| {
            let _document: Document = octagons(
                black_box(60.0),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                ("#ddd", 0.2),
                black_box("#998877"),
            );
        })
    });
}
fn overlapping_circles_bench(c: &mut Criterion) {
    c.bench_function("overlapping_circles", |b| {
        b.iter(|| {
            let _document: Document = overlapping_circles(
                black_box(60.0),
                black_box((4, 4)),
                Fill::new(
                    &(0..16)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..16)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box("#998877"),
            );
        })
    });
}

criterion_group!(
    benches,
    chevrons_bench,
    concentric_circles_bench,
    diamonds_bench,
    hexagons_bench,
    mosaic_squares_bench,
    nested_squares_bench,
    octagons_bench,
    overlapping_circles_bench,
);
criterion_main!(benches);
