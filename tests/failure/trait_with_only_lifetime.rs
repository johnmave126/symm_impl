use symm_impl::symmetric;

trait T<'a> {
    fn op(&self) -> i32;
}

struct A {
    a: i32,
}

#[symmetric]
impl<'a> T<'a> for A {
    fn op(&self) -> i32 {
        self.a
    }
}

fn main() {}
