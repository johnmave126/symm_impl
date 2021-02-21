use symm_impl::symmetric;

trait T<Other> {
    fn op(other: &Other, this: &Self) -> i32;
}

struct A {
    a: i32,
}

struct B {
    b: i32,
}

#[symmetric]
impl T<B> for A {
    fn op(other: &B, this: &Self) -> i32 {
        this.a + other.b
    }
}

fn main() {}
