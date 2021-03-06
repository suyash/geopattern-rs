# geopattern

![Continuous Integration](https://github.com/suyash/geopattern-rs/workflows/Continuous%20Integration/badge.svg) [![crates.io badge](https://img.shields.io/crates/v/geopattern.svg)](https://crates.io/crates/geopattern) [![docs.rs badge](https://docs.rs/geopattern/badge.svg)](https://docs.rs/geopattern)

This has been rewritten to be leaner and actually usable.

The sha1 stuff is out, now essentially a bunch of functions that return `svg::Document`.

In the cleanup, removed error handling and now the code has a bunch of `assert_eq!` calls. Maybe reconsider that.

## Examples

Generated using

```
cargo run --example create_readme_examples
```

### Chevrons

<img src="examples/readme/chevrons.svg">

### Circle Packing

<img src="examples/readme/circle_packing.svg">

### Concentric Circles

<img src="examples/readme/concentric_circles.svg">

### Cubic Disarray

<img src="examples/readme/cubic_disarray.svg">

### Diamonds

<img src="examples/readme/diamonds.svg">

### Hexagons

<img src="examples/readme/hexagons.svg">

### Hypnotic Squares

<img src="examples/readme/hypnotic_squares.svg">

### Joy Division

<img src="examples/readme/joy_division.svg">

### Mosaic Squares

<img src="examples/readme/mosaic_squares.svg">

### Nested Squares

<img src="examples/readme/nested_squares.svg">

### Octagons

<img src="examples/readme/octagons.svg">

### Overlapping Circles

<img src="examples/readme/overlapping_circles.svg">

### Overlapping Rings

<img src="examples/readme/overlapping_rings.svg">

### Piet Mondrian

<img src="examples/readme/piet_mondrian.svg">

### Plaid

<img src="examples/readme/plaid.svg">

### Plus Signs

<img src="examples/readme/plus_signs.svg">

### Sine Waves

<img src="examples/readme/sine_waves.svg">

### Squares

<img src="examples/readme/squares.svg">

### Tesselation

<img src="examples/readme/tesselation.svg">

### Tiled Lines

<img src="examples/readme/tiled_lines.svg">

### Triangles

<img src="examples/readme/triangles.svg">

### Triangular Mesh

<img src="examples/readme/triangular_mesh.svg">

### Un Deus Trois

<img src="examples/readme/un_deus_trois.svg">

### Xes

<img src="examples/readme/xes.svg">

## License

geopattern-rs is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
