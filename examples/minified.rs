fn main() {
    let p = geopattern::generate("something wicked this way comes");
    println!("{}", p.to_minified_svg().unwrap());
}
