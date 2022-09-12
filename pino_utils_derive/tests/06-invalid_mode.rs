
use pino_utils_derive::enum_string;

#[enum_string(invalid)]
enum Weapon {
    Gun,
    Sword,
    Knife,
}

fn main() {}
