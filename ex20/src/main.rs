fn main() {
    println!("To run tests for the ex. 20 use: cargo test -p ex20");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct User {
    nickname: String,
    password_hash: String,
    /* Place here serde attribute, so status will fallback to its Default::default implementation
     * if the actual value is not present */
    status: bool,
    /* Place here serde attribute with path, so short_description will fallback to
     * User::default_description if the actual value is not present */
    short_description: String,
}

impl User {
    fn default_description() -> String {
        String::from("So far it's empty :)")
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    #[test]
    fn check_default_description() {
        let json = r#"{"nickname":"YawKar","password_hash":"c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a"}"#;
        let deserialized: User = serde_json::from_str(json).expect(
            "Status and short description should be optional and fallback to mentioned defaults!",
        );
        let correct_user = User {
            nickname: String::from("YawKar"),
            password_hash: String::from(
                "c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a",
            ),
            status: false,
            short_description: User::default_description(),
        };
        assert_eq!(correct_user, deserialized);
    }

    #[test]
    fn check_nickname_is_required() {
        let json = r#"{"password_hash":"c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a"}"#;
        serde_json::from_str::<User>(json).expect_err("Nickname should be required!");
    }

    #[test]
    fn check_password_hash_is_required() {
        let json = r#"{"nickname":"YawKar"}"#;
        serde_json::from_str::<User>(json).expect_err("Password hash should be required!");
    }
}
