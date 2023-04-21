fn main() {
    println!("To run tests for the ex. 2 use: cargo test -p ex2");
}

/* Place your
code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    height: f32,
    social_nicknames: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::Person;

    #[test]
    fn serialize_person_and_check_whether_fields_uppercased() {
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
}
