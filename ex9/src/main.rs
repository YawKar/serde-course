fn main() {
    println!("To run tests for the ex. 9 use: cargo test -p ex9");
}

#[derive(serde::Deserialize)]
/* Place your serde macro here */
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
    }
}
