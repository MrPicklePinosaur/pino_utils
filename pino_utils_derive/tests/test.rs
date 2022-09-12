
#[cfg(test)]
mod tests {

    use pino_utils_derive::stringify;

    #[test]
    fn stringify_basic() {
        
        #[stringify]
        enum Weapon {
            Gun,
            Sword,
            Knife,
            Hammer
        }

        assert_eq!("Gun", Weapon::Gun.to_string());
        assert_eq!("Sword", Weapon::Sword.to_string());
        assert_eq!("Knife", Weapon::Knife.to_string());
        assert_eq!("Hammer", Weapon::Hammer.to_string());
    }

    /*
    #[test]
    fn advanced_enum() {

        #[stringify]
        enum CoolEnum {
            Single,
            Tuple(String),
            Struct { a: String, b: u32 },
        }
    }
    */

    /*
    #[test]
    fn stringify_struct() {

        #[stringify(a)]
        struct Player {
            name: String,
            health: u32
        }

    }
    */
}
