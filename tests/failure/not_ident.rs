use symm_impl::symmetric;

trait T<Other> {
    fn op(&self, other: &Other) -> i32;
}

struct A {
    a: i32,
}

struct B {
    b: i32,
}

#[symmetric]
impl T<B> for A {
    fn op(&self, _: &B) -> i32 {
        self.a
    }
}

fn main() {}
