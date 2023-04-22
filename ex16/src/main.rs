fn main() {
    println!("To run tests for the ex. 16 use: cargo test -p ex16");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum Nomenclature {
    /* Place serde attribute, so this variant will always be "outer_variant_name_1" */
    InsideName1,
    /* Place serde attribute, so this variant will be "to_outer_variant_name_2" in serialization
     * and "from_outer_variant_name_2" in deserialization */
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
