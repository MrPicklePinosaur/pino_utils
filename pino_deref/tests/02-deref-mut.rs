use pino_deref::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct Person {
    name: String,
}

#[derive(Deref, DerefMut)]
struct Nametag(String);

fn main() {
    let mut person = Person {
        name: String::from("rick"),
    };
    assert_eq!(*person, String::from("rick"));
    *person = "rickus".into();
    assert_eq!(*person, String::from("rickus"));

    let mut nametag = Nametag("morty".into());
    assert_eq!(*nametag, String::from("morty"));
    *nametag = "mortumus".into();
    assert_eq!(*nametag, String::from("mortumus"));
}
