use pino_enum_string::enum_string;

#[enum_string(uppercase)]
enum Weapon {
    Gun,
    Sword,
    Knife,
}

fn main() {
    assert_eq!("GUN", Weapon::Gun.to_string());
    assert_eq!("SWORD", Weapon::Sword.to_string());
    assert_eq!("KNIFE", Weapon::Knife.to_string());
}
