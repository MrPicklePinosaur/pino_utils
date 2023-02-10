use pino_enum_string::enum_string;

#[enum_string(invalid)]
enum Weapon {
    Gun,
    Sword,
    Knife,
}

fn main() {}
