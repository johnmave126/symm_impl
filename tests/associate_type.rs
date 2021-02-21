use symm_impl::symmetric;

trait Distance<Other> {
    type Output;
    fn distance(&self, other: &Other) -> Self::Output;
}

struct Point2D {
    x: f64,
    y: f64,
}

struct Disk {
    center: Point2D,
    radius: f64,
}

impl Distance<Point2D> for Point2D {
    type Output = f64;
    fn distance(&self, other: &Point2D) -> Self::Output {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[symmetric]
impl Distance<Disk> for Point2D {
    type Output = f64;
    fn distance(&self, other: &Disk) -> Self::Output {
        let p_diff = self.distance(&other.center);
        if p_diff.le(&other.radius) {
            0.0_f64
        } else {
            p_diff - other.radius
        }
    }
}

#[test]
fn test_trait_with_associated_type() {
    let p = Point2D { x: 5.0, y: 4.0 };
    let c = Disk {
        center: Point2D { x: 1.0, y: -2.0 },
        radius: 3.0,
    };
    assert_eq!(p.distance(&c), c.distance(&p));
}
