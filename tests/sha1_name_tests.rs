use acton_ern::prelude::*;

#[test]
fn test_sha1name_in_ern() -> anyhow::Result<()> {
    // Create an ERN using SHA1Name
    let ern: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<SHA1Name>("document-content")?
        .with::<Part>("departmentA")?
        .with::<Part>("team1")?
        .build();

    // Verify the constructed ERN matches the expected value
    assert!(
        ern.is_ok(),
        "ern:acton-internal:hr:company123:<sha1-id>/departmentA/team1"
    );

    let ern = ern?;
    assert_eq!(ern.domain.to_string(), "acton-internal");
    assert_eq!(ern.category.to_string(), "hr");
    assert_eq!(ern.account.to_string(), "company123");
    assert_eq!(ern.parts.to_string(), "departmentA/team1");

    // The root should not be empty
    assert!(!ern.root.to_string().is_empty());

    Ok(())
}

#[test]
fn test_sha1name_creation() -> anyhow::Result<()> {
    // Create a SHA1Name directly
    let name1 = SHA1Name::new("test-content".to_string())?;
    let name2 = SHA1Name::new("test-content".to_string())?;

    // SHA1Name should be deterministic for the same input
    assert_eq!(name1.to_string(), name2.to_string());

    Ok(())
}

#[test]
fn test_sha1name_vs_entityroot() -> anyhow::Result<()> {
    // Create a SHA1Name
    let sha1_name1 = SHA1Name::new("test-content".to_string())?;
    let sha1_name2 = SHA1Name::new("test-content".to_string())?;

    // Create an EntityRoot
    let entity_root1 = EntityRoot::new("test-content".to_string())?;
    let entity_root2 = EntityRoot::new("test-content".to_string())?;

    // SHA1Name should be deterministic (same content always produces same ID)
    assert_eq!(sha1_name1.to_string(), sha1_name2.to_string());

    // EntityRoot should be non-deterministic (same content produces different IDs)
    assert_ne!(entity_root1.to_string(), entity_root2.to_string());

    Ok(())
}
