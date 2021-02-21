use symm_impl::symmetric;

trait Distance<Other> {
    fn distance(self, other: Other) -> f64;
    fn distance_2(&mut self, other: &mut Other) -> f64;
    fn distance_3(self, other: Other) -> f64;
}

#[derive(Clone, Copy)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Disk {
    center: Point2D,
    radius: f64,
}

impl Distance<Point2D> for Point2D {
    fn distance(self, other: Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    fn distance_2(&mut self, other: &mut Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    fn distance_3(mut self, mut other: Point2D) -> f64 {
        self.x += 1.0;
        other.x += 1.0;
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[symmetric]
impl Distance<Disk> for Point2D {
    fn distance(self, other: Disk) -> f64 {
        let p_diff = self.distance(other.center);
        if p_diff.le(&other.radius) {
            0.0_f64
        } else {
            p_diff - other.radius
        }
    }
    fn distance_2(&mut self, other: &mut Disk) -> f64 {
        let p_diff = self.distance(other.center);
        if p_diff.le(&other.radius) {
            0.0_f64
        } else {
            p_diff - other.radius
        }
    }
    fn distance_3(mut self, mut other: Disk) -> f64 {
        self.x += 1.0;
        self.x -= 1.0;
        other.radius += 1.0;
        other.radius -= 1.0;
        let p_diff = self.distance(other.center);
        if p_diff.le(&other.radius) {
            0.0_f64
        } else {
            p_diff - other.radius
        }
    }
}

#[test]
fn test_move() {
    let p = Point2D { x: 5.0, y: 4.0 };
    let c = Disk {
        center: Point2D { x: 1.0, y: -2.0 },
        radius: 3.0,
    };
    assert_eq!(p.distance(c), c.distance(p));
}

#[test]
fn test_mut_ref() {
    let mut p = Point2D { x: 5.0, y: 4.0 };
    let mut c = Disk {
        center: Point2D { x: 1.0, y: -2.0 },
        radius: 3.0,
    };
    assert_eq!(p.distance_2(&mut c), c.distance_2(&mut p));
}

#[test]
fn test_mut_move() {
    let p = Point2D { x: 5.0, y: 4.0 };
    let c = Disk {
        center: Point2D { x: 1.0, y: -2.0 },
        radius: 3.0,
    };
    assert_eq!(p.distance_3(c), c.distance_3(p));
}
