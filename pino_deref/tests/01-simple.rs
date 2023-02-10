
use pino_deref::Deref;

#[derive(Deref)]
struct Person {
    name: String
}

fn main() {
    let person = Person { name: String::from("bob") };
    assert_eq!(*person, String::from("bob"));
}
