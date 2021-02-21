use symm_impl::symmetric;

trait Distance<'a, Other> {
    fn distance(&'a self, other: &'a Other) -> f64;
}

struct Point2D<'a> {
    x: &'a f64,
    y: &'a f64,
}

struct Disk<'a> {
    center: Point2D<'a>,
    radius: &'a f64,
}

impl<'a> Distance<'a, Point2D<'a>> for Point2D<'a> {
    fn distance(&'a self, other: &'a Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[symmetric]
impl<'a> Distance<'a, Disk<'a>> for Point2D<'a> {
    fn distance(&'a self, other: &'a Disk<'a>) -> f64 {
        let p_diff = self.distance(&other.center);
        if p_diff.le(&other.radius) {
            0.0_f64
        } else {
            p_diff - other.radius
        }
    }
}

#[test]
fn test_trait_with_lifetime() {
    let p_x = 5.0;
    let p_y = 4.0;
    let c_x = 1.0;
    let c_y = 4.0;
    let c_r = 3.0;
    let p = Point2D { x: &p_x, y: &p_y };
    let c = Disk {
        center: Point2D { x: &c_x, y: &c_y },
        radius: &c_r,
    };
    assert_eq!(p.distance(&c), c.distance(&p));
}
