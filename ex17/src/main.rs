fn main() {
    println!("To run tests for the ex. 17 use: cargo test");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum Color {
    #[serde(alias = "Chili red", alias = "Rust")]
    Red,
    #[serde(alias = "Azure", alias = "Aero")]
    Blue,
    #[serde(alias = "Lime", alias = "Harlequin")]
    Green,
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn deserialize_different_shades_of_red() {
        let shades = vec![r#""Red""#, r#""Chili red""#, r#""Rust""#];
        for shade  in shades {
            assert_eq!(Color::Red, serde_json::from_str(shade).unwrap());
        }
    }
    
    #[test]
    fn deserialize_different_shades_of_blue() {
        let shades = vec![r#""Blue""#, r#""Azure""#, r#""Aero""#];
        for shade in shades {
            assert_eq!(Color::Blue, serde_json::from_str(shade).unwrap());
        }
    }

    #[test]
    fn deserialize_different_shades_of_green() {
        let shades = vec![r#""Green""#, r#""Lime""#, r#""Harlequin""#];
        for shade in shades {
            assert_eq!(Color::Green, serde_json::from_str(shade).unwrap());
        }
    }
}
