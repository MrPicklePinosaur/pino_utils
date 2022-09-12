
#[cfg(test)]
mod tests {

    use pino_utils_derive::stringify;

    #[test]
    fn stringify() {
        
        #[stringify(a)]
        enum Weapon {
            Gun,
            Sword,
            Knife,
            Hammer
        }

    }

    #[test]
    fn stringify_struct() {

        #[stringify(a)]
        struct Player {
            name: String,
            health: u32
        }

    }
}
