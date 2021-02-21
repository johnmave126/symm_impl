use std::ops::{Add, Mul, Sub};
use symm_impl::symmetric;

trait SquareRoot {
    type Output;
    fn square_root(&self) -> Self::Output;
}

impl SquareRoot for f32 {
    type Output = f32;
    fn square_root(&self) -> Self::Output {
        self.sqrt()
    }
}

impl SquareRoot for f64 {
    type Output = f64;
    fn square_root(&self) -> Self::Output {
        self.sqrt()
    }
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0_f32
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0_f64
    }
}

trait Distance<Other, T> {
    fn distance(&self, other: &Other) -> T;
}

struct Point2D<T> {
    x: T,
    y: T,
}

struct Disk<T> {
    center: Point2D<T>,
    radius: T,
}

impl<T> Distance<Point2D<T>, T> for Point2D<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Add<Output = T> + SquareRoot<Output = T> + Copy,
{
    fn distance(&self, other: &Point2D<T>) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).square_root()
    }
}

#[symmetric]
impl<T> Distance<Disk<T>, T> for Point2D<T>
where
    T: Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + SquareRoot<Output = T>
        + Copy
        + PartialOrd
        + Zero,
{
    fn distance(&self, other: &Disk<T>) -> T {
        let p_diff = self.distance(&other.center);
        if p_diff.le(&other.radius) {
            T::zero()
        } else {
            p_diff - other.radius
        }
    }
}

#[test]
fn test_trait_with_more_generics() {
    let p = Point2D { x: 5.0_f32, y: 4.0 };
    let c = Disk {
        center: Point2D { x: 1.0, y: 4.0 },
        radius: 3.0,
    };
    assert_eq!(p.distance(&c), c.distance(&p));
}
