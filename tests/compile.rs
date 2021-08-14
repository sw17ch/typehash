#[test]
fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/**/*.rs");
}

#[test]
fn pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile-pass/**/*.rs");
}
