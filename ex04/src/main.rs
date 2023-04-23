fn main() {
    println!("To run tests for the ex. 4 use: cargo test");
}

/* Place your
 * code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
enum ContainmentClass {
    Safe, // Enum plain variant
    Euclid { reason: String }, // Enum struct variant 
    Keter,
    Thaumiel(u32), // Enum tuple struct variant
    Neutralized,
    Pending,
    Explained,
    Esoteric,
}

/* Place your
 * code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
struct Object {
    name: String,
    code: u32,
    class: ContainmentClass,
}

#[cfg(test)]
mod tests {
    use super::{ContainmentClass, Object};

    #[test]
    fn serialize_and_check_plain_case() {
        let class = ContainmentClass::Neutralized;
        let serialized = serde_json::to_string(&class).unwrap();
        assert_eq!(r#""Neutralized""#, serialized);
    }

    #[test]
    fn serialize_object_and_check_struct_variant() {
        let scp_173 = Object {
            name: String::from("The Sculpture"),
            code: 173,
            class: ContainmentClass::Euclid {
                reason: String::from("Pollutes containment room"),
            },
        };
        let serialized = serde_json::to_string(&scp_173).unwrap();
        assert_eq!(
            "\
            {\
                \"name\":\"The Sculpture\",\
                \"code\":173,\
                \"class\":{\
                    \"Euclid\":{\
                        \"reason\":\"Pollutes containment room\"\
                    }\
                }\
            }",
            serialized
        );
    }

    #[test]
    fn serialize_object_and_check_tuple_struct() {
        let scp_179 = Object {
            name: String::from("Sauelsuesor"),
            code: 179,
            class: ContainmentClass::Thaumiel(5),
        };
        let serialized = serde_json::to_string(&scp_179).unwrap();
        assert_eq!(
            "{\
                \"name\":\"Sauelsuesor\",\
                \"code\":179,\
                \"class\":{\
                    \"Thaumiel\":5\
                }\
            }",
            serialized
        );
    }
}
