use symm_impl::symmetric;

trait T {
    fn op(&self) -> i32;
}

struct A {
    a: i32,
}

#[symmetric]
impl T for A {
    fn op(&self) -> i32 {
        self.a
    }
}

fn main() {}
