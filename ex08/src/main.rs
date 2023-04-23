fn main() {
    println!("To run tests for the ex. 8 use: cargo test");
}

// Marker trait
trait NonSensitiveData {}

#[derive(serde::Serialize, serde::Deserialize)]
/* Place your bound macro here */
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Response<T> {
    id: String,
    body: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct UserInfo {
    username: String,
    online: bool,
    profile_status: String,
}

/* Place your additional impl here */

#[cfg(test)]
mod tests {
    use super::{Response, UserInfo};

    #[test]
    fn serialize_response_with_user_info() {
        let resp = Response {
            id: String::from("sfds-1231"),
            body: UserInfo {
                username: String::from("YawKar"),
                online: true,
                profile_status: String::from(":eight_ball"),
            },
        };
        let serialized = serde_json::to_string(&resp).unwrap();
        assert_eq!(
            r#"{"id":"sfds-1231","body":{"username":"YawKar","online":true,"profile_status":":eight_ball"}}"#,
            serialized
        );
    }

    #[test]
    fn serialize_and_deserialize_user_info() {
        let resp = Response {
            id: String::from("sfds-1231"),
            body: UserInfo {
                username: String::from("YawKar"),
                online: true,
                profile_status: String::from(":eight_ball"),
            },
        };
        let serialized = serde_json::to_string(&resp).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(resp, deserialized);
    }
}
