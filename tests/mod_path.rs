use symm_impl::symmetric;

mod inner {
    pub(crate) mod inner {
        pub(crate) trait Distance<Other> {
            fn distance(&self, other: &Other) -> f64;
        }

        pub(crate) struct Point2D {
            pub(crate) x: f64,
            pub(crate) y: f64,
        }

        pub(crate) struct Disk {
            pub(crate) center: Point2D,
            pub(crate) radius: f64,
        }

        impl Distance<Point2D> for Point2D {
            fn distance(&self, other: &Point2D) -> f64 {
                let dx = self.x - other.x;
                let dy = self.y - other.y;
                (dx * dx + dy * dy).sqrt()
            }
        }
    }
}

#[symmetric]
impl inner::inner::Distance<inner::inner::Disk> for inner::inner::Point2D {
    fn distance(&self, other: &inner::inner::Disk) -> f64 {
        let p_diff = self.distance(&other.center);
        if p_diff.le(&other.radius) {
            0.0_f64
        } else {
            p_diff - other.radius
        }
    }
}

#[test]
fn test_plain_type_inside_mod() {
    use inner::inner::Distance;
    let p = inner::inner::Point2D { x: 5.0, y: 4.0 };
    let c = inner::inner::Disk {
        center: inner::inner::Point2D { x: 1.0, y: -2.0 },
        radius: 3.0,
    };
    assert_eq!(p.distance(&c), c.distance(&p));
}
