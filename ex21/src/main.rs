fn main() {
    println!("To run tests for the ex. 21 use: cargo test");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Pagination {
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct CryptoEntries {
    entries: Vec<Cryptocurrency>,
    #[serde(flatten)]
    pagination: Pagination,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Cryptocurrency {
    ticker: String,
    price: f64,
    is_coin: bool,
}

#[cfg(test)]
mod tests {
    use super::{CryptoEntries, Cryptocurrency, Pagination};

    #[test]
    fn deserialize_and_check_flatten() {
        let json = serde_json::json!(
            {
                "limit": 100,       // All these
                "offset": 500,      // fields will be 'zipped'
                "total": 2300,      // into Pagination struct instance
                "entries": [
                    {
                        "ticker": "BTC",
                        "price": 27.700,
                        "is_coin": true,
                    }
                ]
            }
        )
        .to_string();
        let deserialized: CryptoEntries =
            serde_json::from_str(&json).expect("Should be able to deserialize this.");
        let actual = CryptoEntries {
            pagination: {
                Pagination {
                    limit: 100,
                    offset: 500,
                    total: 2300,
                }
            },
            entries: vec![Cryptocurrency {
                ticker: String::from("BTC"),
                price: 27.700,
                is_coin: true,
            }],
        };
        assert_eq!(actual, deserialized);
    }

    #[test]
    fn serialize_and_check_flatten() {
        let actual = CryptoEntries {
            pagination: {
                Pagination {
                    limit: 100,
                    offset: 500,
                    total: 2300,
                }
            },
            entries: vec![Cryptocurrency {
                ticker: String::from("BTC"),
                price: 27.700,
                is_coin: true,
            }],
        };
        let serialized = serde_json::to_string(&actual).expect("Should be able to serialize this!");
        assert!(
            !serialized.contains("pagination"),
            "Should not have 'pagination' field because its innards should be flattened."
        );
        let deserialized: CryptoEntries = serde_json::from_str(&serialized)
            .expect("Should be able to deserialize from serialized version.");
        assert_eq!(actual, deserialized);
    }
}
