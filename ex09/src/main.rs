fn main() {
    println!("To run tests for the ex. 9 use: cargo test");
}

#[derive(serde::Deserialize)]
#[serde(default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Point {
    legend: String,
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Point {
        Point {
            legend: String::from("Unnamed"),
            x: 1337,
            y: 420,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn deserialize_with_missing_fields() {
        let json = "{}";
        let point: Point = serde_json::from_str(json).unwrap();
        assert_eq!(Point::default(), point);

        let json = r#"{"legend":"Some point"}"#;
        let point: Point = serde_json::from_str(json).unwrap();
        assert_eq!(point.legend, "Some point");
        assert_eq!(point.x, 1337);
        assert_eq!(point.y, 420);
    }
}
