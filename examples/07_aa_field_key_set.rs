use compression::{FieldKey, FieldKeySet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut key_set = FieldKeySet::from_csv("PAT,TYP,DAT")?;
    assert!(key_set.contains(FieldKey::PAT)?);
    key_set.insert(FieldKey::SIZ)?;
    key_set.remove(FieldKey::DAT)?;
    let serialized = key_set.serialize()?;
    assert!(serialized.contains("PAT"));
    assert!(serialized.contains("SIZ"));
    println!("serialized={serialized}");
    println!("✅ AppleArchive field-key APIs OK");
    Ok(())
}
