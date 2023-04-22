fn main() {
    println!("To run tests for the ex. 18 use: cargo test -p ex18");
}

#[derive(serde::Serialize, serde::Deserialize)]
enum Token {
    /* Place serde attribute here, so any attempt to serialize Token::Spacebar will fail */
    Spacebar,
    Character,
    Block,
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn serialize_normal_tokens() {
        assert_eq!(
            r#""Character""#,
            serde_json::to_string(&Token::Character).unwrap()
        );
        assert_eq!(r#""Block""#, serde_json::to_string(&Token::Block).unwrap());
    }

    #[test]
    fn serialize_spacebar() {
        let token = Token::Spacebar;
        serde_json::to_string(&token).expect_err("Spacebar shouldn't be serialized!");
    }
}
