use quasar_qrn::*;

#[test]
fn test() {
    let qrn = QrnBuilder::new()
        .add::<Domain>("quasar-internal")
        .add::<Category>("hr")
        .add::<Company>("company123")
        .add::<Part>("root")
        .add::<Part>("departmentA")
        .add::<Part>("team1")
        .build();
    assert_eq!(qrn.value, "qrn:quasar-internal:hr:company123:root/departmentA/team1");
}

#[test]
fn test_default() {
    let qrn : Qrn= Default::default();
    assert_eq!(qrn.value, "qrn:quasar:system:framework:root");
}

#[test]
fn test_parser() {
    let parser = QrnParser::new("qrn:quasar-internal:hr:company123:root/departmentA/team1");
    let result = parser.parse();

    assert!(result.is_ok(), "Parser should return Ok, but returned Err with message: {:?}", result.err());

    let (domain, category, company, parts) = result.unwrap();

    // Ensure we can compare these directly if Domain, Category, Company, and Parts implement Display
    assert_eq!(domain.to_string(), "quasar-internal", "Domain should be 'quasar-internal'");
    assert_eq!(category.to_string(), "hr", "Category should be 'hr'");
    assert_eq!(company.to_string(), "company123", "Company should be 'company123'");
    assert_eq!(parts.to_string(), "root/departmentA/team1", "Parts should match expected values");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_part() {
        // Initialize the QRN object with a base QRN string
        let mut qrn = QrnBuilder::new()
            .add::<Domain>("quasar-internal")
            .add::<Category>("hr")
            .add::<Company>("company123")
            .add::<Part>("root")
            .add::<Part>("departmentA")
            .build();

        // Append a new part to the QRN
        qrn.append_part("team1");

        // Check the final state of the QRN to ensure it matches expected output
        assert_eq!(qrn.to_string(), "qrn:quasar-internal:hr:company123:root/departmentA/team1", "QRN should match the expected value after appending a new part");
    }
    #[test]
    fn test_append_part_from_root() {
        // Initialize the QRN object with a base QRN string
        let mut qrn = QrnBuilder::new()
            .add::<Domain>("quasar-internal")
            .add::<Category>("hr")
            .add::<Company>("company123")
            .add::<Part>("root")
            .build();

        // Append a new part to the QRN
        qrn.append_part("team1");

        // Check the final state of the QRN to ensure it matches expected output
        assert_eq!(qrn.to_string(), "qrn:quasar-internal:hr:company123:root/team1", "QRN should match the expected value after appending a new part");
    }
}

#[test]
fn test_clone() {
    // Initialize the QRN object with a base QRN string
    let qrn = QrnBuilder::new()
        .add::<Domain>("quasar-internal")
        .add::<Category>("hr")
        .add::<Company>("company123")
        .add::<Part>("root")
        .build();

    // Append a new part to the QRN
    let cloned = qrn.clone();

    // Check the final state of the QRN to ensure it matches expected output
    assert_eq!(qrn, cloned, "QRN should match the expected value after clone");
}
