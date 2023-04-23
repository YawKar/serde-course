fn main() {
    println!("To run tests for the ex. 11 use: cargo test");
}

#[derive(serde::Deserialize)]
#[serde(default = "Point::negative_default")]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Point {
    legend: String,
    x: i32,
    y: i32,
}

impl Point {
    fn negative_default() -> Point {
        Point {
            legend: String::from("negative"),
            x: -1,
            y: -1,
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
        assert_eq!(Point::negative_default(), point);

        let json = r#"{"legend":"Some point"}"#;
        let point: Point = serde_json::from_str(json).unwrap();
        assert_eq!(point.legend, "Some point");
        assert_eq!(point.x, -1);
        assert_eq!(point.y, -1);
    }
}
