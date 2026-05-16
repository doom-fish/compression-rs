use compression::{FieldKey, FieldKeySet};

#[test]
fn field_key_set_supports_clone_and_set_ops() -> Result<(), Box<dyn std::error::Error>> {
    let mut key_set = FieldKeySet::from_csv("PAT,TYP,DAT")?;
    key_set.insert(FieldKey::SIZ)?;
    key_set.remove(FieldKey::DAT)?;

    let mut clone = key_set.clone();
    clone.insert(FieldKey::UID)?;
    key_set.insert_set(&clone)?;

    assert!(key_set.contains(FieldKey::PAT)?);
    assert!(key_set.contains(FieldKey::SIZ)?);
    assert!(key_set.contains(FieldKey::UID)?);
    assert!(!key_set.contains(FieldKey::DAT)?);
    assert!(key_set.serialize()?.contains("UID"));
    Ok(())
}
