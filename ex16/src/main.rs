fn main() {
    println!("To run tests for the ex. 16 use: cargo test");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum Nomenclature {
    #[serde(rename = "outer_variant_name_1")]
    InsideName1,
    #[serde(rename(serialize = "to_outer_variant_name_2", deserialize = "from_outer_variant_name_2"))]
    InsideName2,
    InsideName3,
}

#[cfg(test)]
mod tests {
    use super::Nomenclature;

    #[test]
    fn serialize_and_check() {
        let all = vec![Nomenclature::InsideName1, Nomenclature::InsideName2, Nomenclature::InsideName3];
        let serialized = serde_json::to_string(&all).unwrap();
        assert_eq!(r#"["outer_variant_name_1","to_outer_variant_name_2","InsideName3"]"#, serialized);
    }

    #[test]
    fn deserialize_and_check() {
        let json = r#"["outer_variant_name_1","from_outer_variant_name_2","InsideName3"]"#;
        let deserialized: Vec<Nomenclature> = serde_json::from_str(json).unwrap();
        let correct = vec![Nomenclature::InsideName1, Nomenclature::InsideName2, Nomenclature::InsideName3];
        assert_eq!(correct, deserialized);
    }
}
