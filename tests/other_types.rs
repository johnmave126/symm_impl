use symm_impl::symmetric;

trait A<Other> {
    fn op(&self, other: &Other) -> i32;
}

#[symmetric]
impl<'a> A<&'a str> for [i32; 5] {
    fn op(&self, _other: &&'a str) -> i32 {
        5
    }
}

#[test]
fn test_str_array() {
    let a = [0; 5];
    let b = "1";
    assert_eq!(a.op(&b), 5);
    assert_eq!(b.op(&a), 5);
}
