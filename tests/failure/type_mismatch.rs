use symm_impl::symmetric;

trait T1<Other> {
    fn op_1(&self, other: Other) -> i32;
}

trait T2<Other> {
    fn op_2(self, other: &Other) -> i32;
}

trait T3<Other> {
    fn op_3(&mut self, other: &Other) -> i32;
}

trait T4<Other> {
    fn op_4(&self, other: &mut Other) -> i32;
}

trait T5<Other> {
    fn op_5<'a, 'b>(&'a self, other: &'b Other) -> i32;
}

struct A {
    a: i32,
}

struct B {
    b: i32,
}

#[symmetric]
impl T1<B> for A {
    fn op_1(&self, other: B) -> i32 {
        1
    }
}
#[symmetric]
impl T2<B> for A {
    fn op_2(self, other: &B) -> i32 {
        2
    }
}
#[symmetric]
impl T3<B> for A {
    fn op_3(&mut self, other: &B) -> i32 {
        3
    }
}
#[symmetric]
impl T4<B> for A {
    fn op_4(&self, other: &mut B) -> i32 {
        4
    }
}
#[symmetric]
impl T5<B> for A {
    fn op_5<'a, 'b>(&'a self, other: &'b B) -> i32 {
        5
    }
}

fn main() {}
