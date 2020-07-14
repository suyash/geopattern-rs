use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg::Document;

use geopattern::{
    chevrons, concentric_circles, diamonds, hexagons, mosaic_squares, nested_squares, octagons,
    overlapping_circles, overlapping_rings, plaid, plus_signs, sine_waves, squares, tesselation,
    triangles, xes, Fill,
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
                black_box(("#ddd", 0.2)),
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

fn overlapping_rings_bench(c: &mut Criterion) {
    c.bench_function("overlapping_rings", |b| {
        b.iter(|| {
            let _document: Document = overlapping_rings(
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

fn plaid_bench(c: &mut Criterion) {
    c.bench_function("plaid", |b| {
        b.iter(|| {
            let _document: Document = plaid(
                &(0..19)
                    .map(|v| 5.0 + (v as f32 * 8.0) / 255.0)
                    .collect::<Vec<f32>>(),
                &(1..20)
                    .map(|v| 5.0 + (v as f32 * 8.0) / 255.0)
                    .collect::<Vec<f32>>(),
                Fill::new(
                    &(0..19)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..19)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box("#998877"),
            );
        })
    });
}

fn plus_signs_bench(c: &mut Criterion) {
    c.bench_function("plus_signs", |b| {
        b.iter(|| {
            let _document: Document = plus_signs(
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

fn sine_waves_bench(c: &mut Criterion) {
    c.bench_function("sine_waves", |b| {
        b.iter(|| {
            let _document: Document = sine_waves(
                black_box(60.0),
                black_box(20.0),
                black_box(20.0),
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

fn squares_bench(c: &mut Criterion) {
    c.bench_function("squares", |b| {
        b.iter(|| {
            let _document: Document = squares(
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

fn tesselation_bench(c: &mut Criterion) {
    c.bench_function("tesselation", |b| {
        b.iter(|| {
            let _document: Document = tesselation(
                black_box(60.0),
                Fill::new(
                    &(0..20)
                        .map(|i| if i & 1 == 0 { "#ddd" } else { "#222" })
                        .collect::<Vec<&str>>(),
                    &(0..20)
                        .map(|i| 0.02 + (i as f32 * 0.2) / 255.0)
                        .collect::<Vec<f32>>(),
                ),
                black_box(("#ddd", 0.2)),
                black_box("#998877"),
            );
        })
    });
}

fn triangles_bench(c: &mut Criterion) {
    c.bench_function("triangles", |b| {
        b.iter(|| {
            let _document: Document = triangles(
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

fn xes_bench(c: &mut Criterion) {
    c.bench_function("xes", |b| {
        b.iter(|| {
            let _document: Document = xes(
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
    overlapping_rings_bench,
    plaid_bench,
    plus_signs_bench,
    sine_waves_bench,
    squares_bench,
    tesselation_bench,
    triangles_bench,
    xes_bench,
);
criterion_main!(benches);
