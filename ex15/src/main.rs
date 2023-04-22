fn main() {
    println!("To run tests for the ex. 15 use: cargo test -p ex15");
}

/* Imagine we have unified form of response from main server. */
#[derive(serde::Serialize)]
struct ResponseFromMainServer {
    main_server_key: String,
    referrer: String,
    content: String,
}

impl ResponseFromMainServer {
    fn default_main_server_key() -> &'static str {
        "El Psy Kongroo"
    }
}

/* And we want to automatically convert every response from subservers 
 * into the unified form of ResponseFromMainServer and then serialize it. */

// Add all necessary derives here
#[derive(serde::Serialize)]
/* Place here serde attribute that will tell serde to firstly convert 
 * an instance into ResponseFromMainServer and then serialize the result */
struct ResponseFromSubServer {
    sub_server_key: String,
    content: String,
}

// One of requirements of the 'into' serde attribute
impl Into<ResponseFromMainServer> for ResponseFromSubServer {
    fn into(self) -> ResponseFromMainServer {
        ResponseFromMainServer {
            main_server_key: String::from(ResponseFromMainServer::default_main_server_key()),
            referrer: String::from("subserver"),
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ResponseFromSubServer, ResponseFromMainServer};

    #[test]
    fn serialize_subserver_response_and_get_main_server() {
        let subserver_resp = ResponseFromSubServer {
            sub_server_key: String::from("tut tu ru"),
            content: String::from("Okabe Rintaro"),
        };
        let serialized = serde_json::to_string(&subserver_resp).unwrap();
        let correct_main_resp = ResponseFromMainServer {
            main_server_key: String::from(ResponseFromMainServer::default_main_server_key()),
            referrer: String::from("subserver"),
            content: String::from("Okabe Rintaro"),
        };
        let correct_serialized = serde_json::to_string(&correct_main_resp).unwrap();
        assert_eq!(correct_serialized, serialized);
    }
}
