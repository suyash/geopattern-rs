extern crate geopattern;

fn main() {
    let p = geopattern::generate("something wicked this way comes");
    println!("{}", p.to_svg().unwrap());
}
