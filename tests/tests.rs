use anyhow::Error;
use acton_ern::*;
use acton_ern::prelude::ErnError;

/// Tests for the Acton Ern implementation

#[test]
fn test() -> anyhow::Result<()> {
    // Create an ERN (Entity Resource Name) using the ArnBuilder with specified components
    let ern: Result<Ern<UnixTime>, ErnError> = ArnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<Root>("root")?
        .with::<Part>("departmentA")?
        .with::<Part>("team1")?
        .build();

    // Verify the constructed ERN (Entity Resource Name) matches the expected value
    assert!(
        ern.is_ok(),
        "ern:acton-internal:hr:company123:root/departmentA/team1"
    );
    Ok(())
}

#[test]
fn test_v5() -> anyhow::Result<()> {
    // Create an ERN (Entity Resource Name) using the ArnBuilder with specified components
    let ern_left: Result<Ern<SHA1Name>, ErnError> = ArnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<Root>("root")?
        .with::<Part>("departmentA")?
        .with::<Part>("team1")?
        .build();

    let ern_right: Result<Ern<SHA1Name>, ErnError> = ArnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<Root>("root")?
        .with::<Part>("departmentA")?
        .with::<Part>("team1")?
        .build();

    // Verify the constructed ERN (Entity Resource Name) matches the expected value
    assert!(
        ern_left.is_ok(),
        "ern:acton-internal:hr:company123:root/departmentA/team1"
    );
    assert!(
        ern_right.is_ok(),
        "ern:acton-internal:hr:company123:root/departmentA/team1"
    );
    assert_eq!(ern_left?, ern_right?);
    Ok(())
}

#[test]
fn test_parser() -> anyhow::Result<()> {
    // Create an ArnParser with a specific ERN (Entity Resource Name) string
    let parser: ArnParser<UnixTime> = ArnParser::new("ern:acton-internal:hr:company123:root/departmentA/team1");

    // Parse the ERN (Entity Resource Name) string into its components
    let result = parser.parse();

    // Verify the parser returns a successful result
    assert!(
        result.is_ok(),
        "Parser should return Ok, but returned Err with message: {:?}",
        result.err()
    );

    // Extract the components from the result
    let ern = result.unwrap();

    // Verify each component matches the expected value
    assert_eq!(
        ern.domain.to_string(),
        "acton-internal",
        "Domain should be 'acton-internal'"
    );
    assert_eq!(ern.category.to_string(), "hr", "Category should be 'hr'");
    assert_eq!(
        ern.account.to_string(),
        "company123",
        "Account should be 'company123'"
    );
    assert_eq!(
        ern.parts.to_string(),
        "departmentA/team1",
        "Parts should match expected values"
    );
    Ok(())
}
