use pino_enum_string::enum_string;

#[enum_string]
struct Player {
    name: String,
    health: u32,
}

fn main() {}
