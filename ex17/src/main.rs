fn main() {
    println!("To run tests for the ex. 17 use: cargo test");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum Color {
    /* Place here serde attribute to add next aliases: "Chili red", "Rust" */
    Red,
    /* Place here serde attribute to add next aliases: "Azure", "Aero" */
    Blue,
    /* Place here serde attribute to add next aliases: "Lime", "Harlequin" */
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
