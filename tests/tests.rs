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
    assert_eq!(qrn, "qrn:quasar-internal:hr:company123:root:departmentA:team1");
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


