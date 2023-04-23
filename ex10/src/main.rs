fn main() {
    println!("To run tests for the ex. 10 use: cargo test");
}

#[derive(serde::Deserialize)]
#[serde(default = "method_that_returns_default_instance")]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Point {
    legend: String,
    x: i32,
    y: i32,
}

fn method_that_returns_default_instance() -> Point {
    Point {
        legend: String::from("Default legend"),
        x: -1,
        y: -1,
    }
}

#[cfg(test)]
mod tests {
    use super::{method_that_returns_default_instance, Point};

    #[test]
    fn deserialize_with_missing_fields() {
        let json = "{}";
        let point: Point = serde_json::from_str(json).unwrap();
        assert_eq!(method_that_returns_default_instance(), point);

        let json = r#"{"legend":"Some point"}"#;
        let point: Point = serde_json::from_str(json).unwrap();
        assert_eq!(point.legend, "Some point");
        assert_eq!(point.x, -1);
        assert_eq!(point.y, -1);
    }
}
