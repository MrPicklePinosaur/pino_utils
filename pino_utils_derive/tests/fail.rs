
use pino_utils_derive::stringify;

#[test]
#[should_panic]
fn stringify_struct() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail.rs");

    #[stringify(invalid)]
    struct Player {
        name: String,
        health: u32
    }

}
