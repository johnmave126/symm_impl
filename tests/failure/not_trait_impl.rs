use symm_impl::symmetric;

struct A {
    a: i32,
}

#[symmetric]
impl A {
    fn new(a: i32) -> Self {
        Self { a }
    }
}

fn main() {}
