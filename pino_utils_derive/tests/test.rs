
#[cfg(test)]
mod tests {

    use pino_utils_derive::stringify;

    #[test]
    fn stringify_basic() {
        
        #[stringify(verbatim)]
        enum Weapon {
            Gun,
            Sword,
            Knife,
        }

        assert_eq!("Gun", Weapon::Gun.to_string());
        assert_eq!("Sword", Weapon::Sword.to_string());
        assert_eq!("Knife", Weapon::Knife.to_string());
    }

    #[test]
    fn lowercase() {
        
        #[stringify(lowercase)]
        enum Weapon {
            Gun,
            Sword,
            Knife,
        }

        assert_eq!("gun", Weapon::Gun.to_string());
        assert_eq!("sword", Weapon::Sword.to_string());
        assert_eq!("knife", Weapon::Knife.to_string());

    }

    #[test]
    fn uppercase() {
        
        #[stringify(uppercase)]
        enum Weapon {
            Gun,
            Sword,
            Knife,
        }

        assert_eq!("GUN", Weapon::Gun.to_string());
        assert_eq!("SWORD", Weapon::Sword.to_string());
        assert_eq!("KNIFE", Weapon::Knife.to_string());

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
