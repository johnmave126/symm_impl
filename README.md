# Attribute macro that automatically implements a symmetric trait
[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
[![Build Status][build-image]][build-link]

```
[dependencies]
symm_impl = "0.1"
```

## Example
```rust
use symm_impl::symmetric;

trait Distance<Other> {
    fn distance(&self, other: &Other) -> f64;
}
struct Point2D {
    x: f64,
    y: f64,
}
struct Disk {
    center: Point2D,
    radius: f64
}
impl Distance<Point2D> for Point2D {
    fn distance(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
#[symmetric]
impl Distance<Disk> for Point2D {
    fn distance(&self, other: &Disk) -> f64 {
        let p_diff = self.distance(&other.center);
        if p_diff.le(&other.radius) {
            0.0_f64
        } else {
            p_diff - other.radius
        }
    }
}
/* Expands to
impl Distance<Point2D> for Disk {
    #[allow(unused_mut)]
    #[inline]
    fn distance(&self, other: &Point2D) -> f64 {
        <Point2D as Distance>::distance(other, self)
    }
}
*/
```

## Details
In computational geometry (and potentially other areas), it is common to have symmetric binary operator between two different types. For example, distance between two different shapes, and intersection between two different shapes. In these cases, one would expect to only need to implement the operator in one direction and automatically derive the other.

This attribute macro automatically implements the "mirrored" version of an implementation to save a few keystroke and make the code looks slightly cleaner.

We require the trait to have the following property to be symmetric:
* Trait must be generic, with the first non-lifetime parameter being the type for the symmetry.
* All the methods in the trait must take exactly 2 arguments, where the first argument is a receiver (`self`, `&self`, `&mut self`) and the other argument is of the type for the symmetry. The two arguments must have the same family in the sense that they should both or neither be reference or mutable.

## License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.


[//]: # (badges and links)

[crate-image]: https://img.shields.io/crates/v/symm_impl.svg
[crate-link]: https://crates.io/crates/symm_impl
[docs-image]: https://docs.rs/symm_impl/badge.svg
[docs-link]: https://docs.rs/symm_impl/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[build-image]: https://github.com/johnmave126/symm_impl/actions/workflows/symm_impl.yml/badge.svg?branch=master&event=push
[build-link]: https://github.com/johnmave126/symm_impl/actions/workflows/symm_impl.yml?query=branch:master