fn main() {
    println!("To run tests for the ex. 18 use: cargo test -p ex18");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug))]
enum Token {
    /* Place serde attribute here, so any attempt to serialize AND deserialize Token::Spacebar will fail */
    Spacebar,
    /* Place serde attribute here, so any attempt to serialize Token::Character will fail */
    Character,
    /* Place serde attribute here, so any attempt to deserialize Token::Block will fail */
    Block,
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn serialize_spacebar_should_fail() {
        let token = Token::Spacebar;
        serde_json::to_string(&token).expect_err("Spacebar shouldn't be serializable!");
    }

    #[test]
    fn deserialize_spacebar_should_fail() {
        let json = r#""Spacebar""#;
        serde_json::from_str::<Token>(json).expect_err("Spacbar shouldn't be deserializable!");
    }

    #[test]
    fn serialize_character_should_fail() {
        let token = Token::Character;
        serde_json::to_string(&token).expect_err("Character shouldn't be serializable!");
    }

    #[test]
    fn deserialize_character_should_work() {
        let json = r#""Character""#;
        serde_json::from_str::<Token>(json).expect("Character should be deserializable!");
    }

    #[test]
    fn serialize_block_should_work() {
        let token = Token::Block;
        serde_json::to_string(&token).expect("Block should be serializable!");
    }

    #[test]
    fn deserialize_block_should_fail() {
        let json = r#""Block""#;
        serde_json::from_str::<Token>(json).expect_err("Block shouldn't be deserializable!");
    }
}
