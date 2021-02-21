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
    radius: f64,
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

#[test]
fn test_plain_type() {
    let p = Point2D { x: 5.0, y: 4.0 };
    let c = Disk {
        center: Point2D { x: 1.0, y: -2.0 },
        radius: 3.0,
    };
    assert_eq!(p.distance(&c), c.distance(&p));
}