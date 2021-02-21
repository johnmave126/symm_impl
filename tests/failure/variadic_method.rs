// Should be enabled after
// https://github.com/rust-lang/rust/issues/44930 is resolved

/*
use symm_impl::symmetric;

trait T<Other> {
    unsafe extern "C" fn op(&self, args: ...) -> i32;
}

struct A {
    a: i32,
}

struct B {
    b: i32,
}

#[symmetric]
impl T<B> for A {
    unsafe extern "C" fn op(&self, args: ...) -> i32 {
        3
    }
}
*/
compile_error!("https://github.com/rust-lang/rust/issues/44930");

fn main() {}
