fn main() {
    println!("To run tests for the ex. 2 use: cargo test -p ex2");
}

/* Place your
 * code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    height: f32,
    social_nicknames: Vec<String>,
}

/* Place your
 * code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
struct Cryptocurrency {
    full_name: String,
    ticker: String,
    is_coin: bool,
}

#[cfg(test)]
mod tests {
    use super::{Cryptocurrency, Person};

    #[test]
    fn serialize_person_and_check_whether_fields_camel_cased() {
        let person = Person {
            first_name: String::from("Walter"),
            last_name: String::from("White"),
            age: 52,
            height: 177.8,
            social_nicknames: vec![
                String::from("Mr. White"),
                String::from("Walt"),
                String::from("Heisenberg"),
                String::from("Mr. Mayhew"),
                String::from("Mr. Lambert"),
                String::from("David Lynn"),
            ],
        };
        let serialized = serde_json::to_string(&person).unwrap();
        assert_eq!(
            "{\
                \"firstName\":\"Walter\",\
                \"lastName\":\"White\",\
                \"age\":52,\
                \"height\":177.8,\
                \"socialNicknames\":[\
                    \"Mr. White\",\
                    \"Walt\",\
                    \"Heisenberg\",\
                    \"Mr. Mayhew\",\
                    \"Mr. Lambert\",\
                    \"David Lynn\"\
                ]\
            }",
            serialized
        );
    }

    #[test]
    fn deserialize_person_from_screaming_kebab_case() {
        let skc_person = "{\
                \"FIRST-NAME\":\"Walter\",\
                \"LAST-NAME\":\"White\",\
                \"AGE\":52,\
                \"HEIGHT\":177.8,\
                \"SOCIAL-NICKNAMES\":[\
                    \"Mr. White\",\
                    \"Walt\",\
                    \"Heisenberg\",\
                    \"Mr. Mayhew\",\
                    \"Mr. Lambert\",\
                    \"David Lynn\"\
                ]\
            }";
        let person: Person = serde_json::from_str(skc_person).unwrap();
        let actual_person = Person {
            first_name: String::from("Walter"),
            last_name: String::from("White"),
            age: 52,
            height: 177.8,
            social_nicknames: vec![
                String::from("Mr. White"),
                String::from("Walt"),
                String::from("Heisenberg"),
                String::from("Mr. Mayhew"),
                String::from("Mr. Lambert"),
                String::from("David Lynn"),
            ],
        };
        assert_eq!(actual_person, person);
    }

    #[test]
    fn serialize_cryptocurrency_and_check_whether_fields_pascal_cased() {
        let crypto = Cryptocurrency {
            full_name: String::from("Polkadot"),
            ticker: String::from("DOT"),
            is_coin: true,
        };
        let serialized = serde_json::to_string(&crypto).unwrap();
        assert_eq!(
            "{\
                \"FullName\":\"Polkadot\",\
                \"Ticker\":\"DOT\",\
                \"IsCoin\":true\
            }",
            serialized
        );
    }

    #[test]
    fn serialize_and_deserialize_crypto() {
        let crypto = Cryptocurrency {
            full_name: String::from("Solana"),
            ticker: String::from("SOL"),
            is_coin: true,
        };
        let serialized = serde_json::to_string(&crypto).unwrap();
        let deserialized: Cryptocurrency = serde_json::from_str(&serialized).unwrap();
        assert_eq!(crypto, deserialized);
    }
}
