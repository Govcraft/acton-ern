use anyhow::Error;
use acton_eid::*;
use acton_eid::prelude::EidError;

/// Tests for the Acton Eid implementation

#[test]
fn test() -> anyhow::Result<()> {
    // Create an ERN (Entity Resource Name) using the ArnBuilder with specified components
    let eid: Result<Eid<UnixTime>, EidError> = ArnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<Root>("root")?
        .with::<Part>("departmentA")?
        .with::<Part>("team1")?
        .build();

    // Verify the constructed ERN (Entity Resource Name) matches the expected value
    assert!(
        eid.is_ok(),
        "eid:acton-internal:hr:company123:root/departmentA/team1"
    );
    Ok(())
}

#[test]
fn test_parser() -> anyhow::Result<()> {
    // Create an ArnParser with a specific ERN (Entity Resource Name) string
    let parser: ArnParser<UnixTime> = ArnParser::new("ein:acton-internal:hr:company123:root/departmentA/team1");

    // Parse the ERN (Entity Resource Name) string into its components
    let result = parser.parse();

    // Verify the parser returns a successful result
    assert!(
        result.is_ok(),
        "Parser should return Ok, but returned Err with message: {:?}",
        result.err()
    );

    // Extract the components from the result
    let eid = result.unwrap();

    // Verify each component matches the expected value
    assert_eq!(
        eid.domain.to_string(),
        "acton-internal",
        "Domain should be 'acton-internal'"
    );
    assert_eq!(eid.category.to_string(), "hr", "Category should be 'hr'");
    assert_eq!(
        eid.account.to_string(),
        "company123",
        "Account should be 'company123'"
    );
    assert_eq!(
        eid.parts.to_string(),
        "departmentA/team1",
        "Parts should match expected values"
    );
    Ok(())
}
