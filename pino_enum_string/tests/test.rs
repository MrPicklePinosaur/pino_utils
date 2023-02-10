#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-simple.rs");
    t.pass("tests/02-lowercase.rs");
    t.pass("tests/03-uppercase.rs");
    // t.pass("tests/04-advanced.rs");
    t.compile_fail("tests/05-struct.rs");
    t.compile_fail("tests/06-invalid_mode.rs");
}
