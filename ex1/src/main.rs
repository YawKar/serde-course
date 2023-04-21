fn main() {
    println!("To run tests for the ex. 1 use: cargo test -p ex1");
}

/* Place your code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced with plain derive
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn serialize_point_and_check_by_str_example() {
        let point = Point { x: 1, y: 2 };
        let serialized = serde_json::to_string(&point).unwrap();
        assert_eq!(r#"{"x":1,"y":2}"#, serialized);
    }

    #[test]
    fn deserialize_point_and_check_by_instance() {
        let serialized = r#"{"x":1,"y":2}"#;
        let point: Point = serde_json::from_str(serialized).unwrap();
        assert_eq!(Point { x: 1, y: 2 }, point);
    }

    #[test]
    fn serialize_and_deserialize_point() {
        let point = Point { x: 42, y: 146 };
        let serialized = serde_json::to_string(&point).unwrap();
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        assert_eq!(point, deserialized);
    }
}
