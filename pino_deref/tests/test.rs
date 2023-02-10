
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-deref.rs");
    t.pass("tests/02-deref-mut.rs");
}
