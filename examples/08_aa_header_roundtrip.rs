use compression::{BlobDescription, FieldKey, HashFunction, Header, Timespec};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut header = Header::new()?;
    header.append_field_uint(FieldKey::TYP, u64::from(b'F'))?;
    header.append_field_string(FieldKey::PAT, "notes.txt")?;
    header.append_field_hash(FieldKey::SH2, HashFunction::Sha256, &[7_u8; 32])?;
    header.append_field_timespec(
        FieldKey::MTM,
        Timespec {
            seconds: 1_234,
            nanoseconds: 56,
        },
    )?;
    header.append_field_blob(FieldKey::DAT, 512)?;

    let encoded = header.encoded_data()?;
    let decoded = Header::from_encoded_data(&encoded)?;

    assert_eq!(decoded.path()?.as_deref(), Some("notes.txt"));
    assert_eq!(
        decoded.hash_with_key(FieldKey::SH2)?.expect("hash").bytes,
        vec![7_u8; 32]
    );
    assert_eq!(
        decoded.blob_with_key(FieldKey::DAT)?.expect("blob"),
        BlobDescription {
            size: 512,
            offset: 0
        }
    );
    let field_count = decoded.field_count();
    println!("fields={field_count}");
    println!("✅ AppleArchive header encode/decode OK");
    Ok(())
}
