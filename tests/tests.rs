use akton_arn::*;

/// Tests for the Akton ARN implementation

#[test]
fn test() {
    // Create an Arn using the ArnBuilder with specified components
    let arn = ArnBuilder::new()
        .add::<Domain>("akton-internal")
        .add::<Category>("hr")
        .add::<Company>("company123")
        .add::<Part>("root")
        .add::<Part>("departmentA")
        .add::<Part>("team1")
        .build();

    // Verify the constructed Arn matches the expected value
    assert_eq!(arn.value, "arn:akton-internal:hr:company123:root/departmentA/team1");
}

#[test]
fn test_default() {
    // Create a default Arn using the Default trait implementation
    let arn: Arn = Default::default();

    // Verify the default Arn matches the expected value
    assert_eq!(arn.value, "arn:akton:system:framework:root");
}

#[test]
fn test_parser() {
    // Create an ArnParser with a specific Arn string
    let parser = ArnParser::new("arn:akton-internal:hr:company123:root/departmentA/team1");

    // Parse the Arn string into its components
    let result = parser.parse();

    // Verify the parser returns a successful result
    assert!(result.is_ok(), "Parser should return Ok, but returned Err with message: {:?}", result.err());

    // Extract the components from the result
    let (domain, category, company, parts) = result.unwrap();

    // Verify each component matches the expected value
    assert_eq!(domain.to_string(), "akton-internal", "Domain should be 'akton-internal'");
    assert_eq!(category.to_string(), "hr", "Category should be 'hr'");
    assert_eq!(company.to_string(), "company123", "Company should be 'company123'");
    assert_eq!(parts.to_string(), "root/departmentA/team1", "Parts should match expected values");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_part() {
        // Initialize the Arn object with a base Arn string
        let mut arn = ArnBuilder::new()
            .add::<Domain>("akton-internal")
            .add::<Category>("hr")
            .add::<Company>("company123")
            .add::<Part>("root")
            .add::<Part>("departmentA")
            .build();

        // Append a new part to the Arn
        arn.append_part("team1");

        // Verify the final state of the Arn matches the expected output
        assert_eq!(arn.to_string(), "arn:akton-internal:hr:company123:root/departmentA/team1", "Arn should match the expected value after appending a new part");
    }

    #[test]
    fn test_append_part_from_root() {
        // Initialize the Arn object with a base Arn string
        let mut arn = ArnBuilder::new()
            .add::<Domain>("akton-internal")
            .add::<Category>("hr")
            .add::<Company>("company123")
            .add::<Part>("root")
            .build();

        // Append a new part to the Arn
        arn.append_part("team1");

        // Verify the final state of the Arn matches the expected output
        assert_eq!(arn.to_string(), "arn:akton-internal:hr:company123:root/team1", "Arn should match the expected value after appending a new part");
    }
}

#[test]
fn test_clone() {
    // Initialize the Arn object with a base Arn string
    let arn = ArnBuilder::new()
        .add::<Domain>("akton-internal")
        .add::<Category>("hr")
        .add::<Company>("company123")
        .add::<Part>("root")
        .build();

    // Clone the Arn
    let cloned = arn.clone();

    // Verify the cloned Arn matches the original
    assert_eq!(arn, cloned, "Arn should match the expected value after clone");
}
