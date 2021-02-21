// Should be enabled after
// https://github.com/rust-lang/rust/issues/68318 is resolved

/*
use symm_impl::symmetric;

struct A {
    a: i32,
}

#[symmetric]
impl !Sync for A {}

fn main() {}
*/
compile_error!("https://github.com/rust-lang/rust/issues/68318");
fn main() {}
