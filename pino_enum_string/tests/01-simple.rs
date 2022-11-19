
use pino_enum_string::enum_string;

#[enum_string]
enum Weapon {
    Gun,
    Sword,
    Knife,
}

fn main() {
    assert_eq!("Gun", Weapon::Gun.to_string());
    assert_eq!("Sword", Weapon::Sword.to_string());
    assert_eq!("Knife", Weapon::Knife.to_string());
}
