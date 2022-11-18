
use enum_string::enum_string;

#[enum_string(lowercase)]
enum Weapon {
    Gun,
    Sword,
    Knife,
}

fn main() {
    assert_eq!("gun", Weapon::Gun.to_string());
    assert_eq!("sword", Weapon::Sword.to_string());
    assert_eq!("knife", Weapon::Knife.to_string());
}
