use geopattern::patterns::Patterns;

fn main() {
    for &pattern in [
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
    ].iter() {
        let patname = pattern.to_string();
        let pat = &[pattern];
        let p = geopattern::GeoPattern::new(patname.as_str())
                .patterns(pat)
                .build()
                .unwrap();

        svg::save(format!("examples/samples/{}.svg", pattern.to_string()), p.to_svg().unwrap()).unwrap();
        println!("created {}", pattern);
    }
}
