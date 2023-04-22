fn main() {
    println!("To run tests for the ex. 14 use: cargo test -p ex14");
}

/* Imagine we have this big struct with different fields */
#[derive(serde::Serialize, serde::Deserialize)]
struct VeryBigStruct {
    name: String,
    age: u8,
    id: String,
    profession: String,
}

/* And we also have this tiny struct that may be
 * in some sense derived from the VeryBigStruct.
 * We may want somehow deserialize TinyStruct through such a pipeline:
 * (json with VeryBigStruct) -> (VeryBigStruct) -> (TinyStruct) */
#[derive(serde::Serialize, serde::Deserialize)]
// Don't remove it! IT will check whether you really do the conversion.
#[serde(deny_unknown_fields)]
/* Place here the serde attribute that tells serde to deserialize TinyStruct through
 * deserializing firstly into VeryBigStruct and then converting into TinyStruct */
struct TinyStruct {
    name: String,
}

/* Place here you impl of From<VeryBigStruct> for TinyStruct. */

#[cfg(test)]
mod tests {
    use super::TinyStruct;

    #[test]
    fn deserialize_tiny_from_big() {
        let json = r#"{"name":"YawKar","age":20,"id":"diru321","profession":"fashion"}"#;
        let deserialized: TinyStruct = serde_json::from_str(json).unwrap();
        assert_eq!(String::from("YawKar"), deserialized.name);
    }
}
