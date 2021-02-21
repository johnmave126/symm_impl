use symm_impl::symmetric;

trait T<Other> {
    fn op(&self, other: &Other, more: i32) -> i32;
}

struct A {
    a: i32,
}

struct B {
    b: i32,
}

#[symmetric]
impl T<B> for A {
    fn op(&self, other: &B, more: i32) -> i32 {
        self.a + other.b
    }
}

fn main() {}
