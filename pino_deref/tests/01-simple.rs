
use pino_deref::Deref;

#[derive(Deref)]
struct Person {
    name: String
}

#[derive(Deref)]
struct Nametag(String);

fn main() {
    let person = Person { name: String::from("rick") };
    let nametag = Nametag("morty".into());

    assert_eq!(*person, String::from("rick"));
    assert_eq!(*nametag, String::from("morty"));
}
