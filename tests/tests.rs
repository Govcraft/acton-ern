use acton_ern::EntityRoot;
use acton_ern::prelude::*;

//
// /// Tests for the Acton Ern implementation
#[test]
fn test() -> anyhow::Result<()> {
    // Create an ERN (Entity Resource Name) using the ErnBuilder with specified components
    let ern: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal".into())?
        .with::<Category>("hr".into())?
        .with::<Account>("company123".into())?
        .with::<EntityRoot>("root".into())?
        .with::<Part>("departmentA".into())?
        .with::<Part>("team1".into())?
        .build();

    // Verify the constructed ERN (Entity Resource Name) matches the expected value
    assert!(
        ern.is_ok(),
        "ern:acton-internal:hr:company123:root/departmentA/team1"
    );
    let ern = ern?;
    assert_eq!(ern.domain.to_string(), "acton-internal");
    assert_eq!(ern.category.to_string(), "hr");
    assert_eq!(ern.account.to_string(), "company123");
    assert_eq!(ern.parts.to_string(), "departmentA/team1");
    assert!(ern.root.to_string().starts_with("root_"));
    Ok(())
}
//
// #[test]
// fn test_v7() -> anyhow::Result<()> {
//     // Create an ERN (Entity Resource Name) using the ErnBuilder with specified components
//     let ern_left: Result<Ern, ErnError> = ErnBuilder::new()
//         .with::<Domain>("acton-internal")?
//         .with::<Category>("hr")?
//         .with::<Account>("company123")?
//         .with::<EntityRoot>("root")?
//         .with::<Part>("departmentA")?
//         .with::<Part>("team1")?
//         .build();
//
//     let ern_right: Result<Ern, ErnError> = ErnBuilder::new()
//         .with::<Domain>("acton-internal")?
//         .with::<Category>("hr")?
//         .with::<Account>("company123")?
//         .with::<EntityRoot>("root")?
//         .with::<Part>("departmentA")?
//         .with::<Part>("team1")?
//         .build();
//
//     // Verify the constructed ERN (Entity Resource Name) matches the expected value
//     assert!(ern_left.is_ok());
//     assert!(ern_right.is_ok());
//     assert_ne!(ern_left?, ern_right?);
//     Ok(())
// }
//
// #[test]
// fn test_v5() -> anyhow::Result<()> {
//     // Create an ERN (Entity Resource Name) using the ErnBuilder with specified components
//     let ern_left: Result<Ern<SHA1Name>, ErnError> = ErnBuilder::new()
//         .with::<Domain>("acton-internal")?
//         .with::<Category>("hr")?
//         .with::<Account>("company123")?
//         .with::<EntityRoot>("same")?
//         .with::<Part>("departmentA")?
//         .with::<Part>("team1")?
//         .build();
//
//     let ern_right: Result<Ern<SHA1Name>, ErnError> = ErnBuilder::new()
//         .with::<Domain>("acton-internal")?
//         .with::<Category>("hr")?
//         .with::<Account>("company123")?
//         .with::<EntityRoot>("same")?
//         .with::<Part>("departmentA")?
//         .with::<Part>("team1")?
//         .build();
//
//     // Verify the constructed ERN (Entity Resource Name) matches the expected value
//     assert!(ern_left.is_ok());
//     assert!(ern_right.is_ok());
//     assert_eq!(ern_left?, ern_right?);
//     Ok(())
// }
//
// #[test]
// fn test_parser() -> anyhow::Result<()> {
//     // Create an ErnParser with a specific ERN (Entity Resource Name) string
//     let parser: ErnParser =
//         ErnParser::new("ern:acton-internal:hr:company123:root/departmentA/team1".to_string());
//
//     // Parse the ERN (Entity Resource Name) string into its components
//     let result = parser.parse();
//
//     // Verify the parser returns a successful result
//     assert!(
//         result.is_ok(),
//         "Parser should return Ok, but returned Err with message: {:?}",
//         result.err()
//     );
//
//     // Extract the components from the result
//     let ern = result.unwrap();
//
//     // Verify each component matches the expected value
//     assert_eq!(
//         ern.domain.to_string(),
//         "acton-internal",
//         "Domain should be 'acton-internal'"
//     );
//     assert_eq!(ern.category.to_string(), "hr", "Category should be 'hr'");
//     assert_eq!(
//         ern.account.to_string(),
//         "company123",
//         "Account should be 'company123'"
//     );
//     assert_eq!(
//         ern.parts.to_string(),
//         "departmentA/team1",
//         "Parts should match expected values"
//     );
//     Ok(())
// }
