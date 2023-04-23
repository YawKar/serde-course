fn main() {
    println!("To run tests for the ex. 3 use: cargo test");
}

/* Place your
 * code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
struct MobilePlan {
    call_minutes: u32,
    sms_quantity: u32,
    mms_quantity: u32,
    price: u32,
}

#[cfg(test)]
mod tests {
    use super::MobilePlan;

    #[test]
    fn deserialize_from_json_with_unknown_fields_should_fail() {
        let json = "\
            {\
                \"call_minutes\":1500,\
                \"sms_quantity\":200,\
                \"mms_quantity\":0,\
                \"price\":300,\
                \"is_special_offer\":false\
            }";
        serde_json::from_str::<MobilePlan>(json).expect_err(
            "Shouldn't deserialize this json because it contains unknown field 'is_special_offer'",
        );
    }
}
