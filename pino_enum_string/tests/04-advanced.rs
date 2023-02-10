use pino_enum_string::enum_string;

#[enum_string]
enum CoolEnum {
    Single,
    // Tuple(String),
    // Struct { a: String, b: u32 },
}

fn main() {
    assert_eq!("Single", CoolEnum::Single.to_string());
    // assert_eq!("Tuple", CoolEnum::Tuple.to_string());
    // assert_eq!("Struct", CoolEnum::Struct.to_string());
}
