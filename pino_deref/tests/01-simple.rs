
use pino_deref::Deref;

#[derive(Deref)]
struct Person {
    name: String
}

#[derive(Deref)]
struct Nametag(String);

fn main() {
    let person = Person { name: String::from("bob") };
    let nametag = Nametag("bill".into());

    assert_eq!(*person, String::from("bob"));
    assert_eq!(*nametag, String::from("bill"));
}
