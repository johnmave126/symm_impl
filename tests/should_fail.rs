#[test]
fn should_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failure/*.rs");
}
