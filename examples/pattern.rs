extern crate geopattern;

fn main() {
    let p = geopattern::GeoPattern::new("Mastering Markdown")
        .patterns(&[geopattern::patterns::Patterns::Chevrons])
        .build()
        .unwrap();

    println!("{}", p.to_svg().unwrap());
}
