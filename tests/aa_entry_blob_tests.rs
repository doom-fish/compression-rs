mod support;

use compression::{AceQualifierType, AccessControlEntry, ArchiveFlags, EntryAclBlob, EntryXatBlob, NamedBlobEntry};
use std::env;
use std::fs;
use std::process::Command;
use support::{artifact_dir, path_string};

fn run_command(program: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new(program).args(args).status()?;
    assert!(status.success(), "{program} {args:?} failed with {status}");
    Ok(())
}

#[test]
fn entry_acl_and_xat_blobs_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aa-entry-blob");
    let dir = path_string(&artifact_dir);
    let source_name = "source.txt";
    let target_acl_name = "target-acl.txt";
    let target_xat_name = "target-xat.txt";
    let source_path = path_string(&artifact_dir.join(source_name));
    let target_acl_path = path_string(&artifact_dir.join(target_acl_name));
    let target_xat_path = path_string(&artifact_dir.join(target_xat_name));
    fs::write(&source_path, b"entry blob source")?;
    fs::write(&target_acl_path, b"acl target")?;
    fs::write(&target_xat_path, b"xat target")?;

    run_command("xattr", &["-w", "com.example.compression-rs", "blob-data", &source_path])?;

    let current_user = env::var("USER")?;
    let acl_entry = AccessControlEntry {
        tag: 1,
        perms: 1 << 1,
        flags: 0,
        qualifier_type: AceQualifierType::User,
        qualifier: current_user.into_bytes(),
    };
    let mut rebuilt_acl = EntryAclBlob::new()?;
    rebuilt_acl.append_entry(&acl_entry)?;
    rebuilt_acl.set_entry(0, &acl_entry)?;
    assert_eq!(rebuilt_acl.entry_count(), 1);
    let encoded_acl = rebuilt_acl.encoded_data()?;
    let mut decoded_acl = EntryAclBlob::from_encoded_data(&encoded_acl)?;
    decoded_acl.remove_entry(0)?;
    assert_eq!(decoded_acl.entry_count(), 0);
    decoded_acl.append_entry(&acl_entry)?;
    decoded_acl.apply_to_path(&dir, target_acl_name, ArchiveFlags::REPLACE_ATTRIBUTES)?;
    let source_acl = EntryAclBlob::from_path(&dir, target_acl_name, ArchiveFlags::empty())?;
    let target_acl = EntryAclBlob::from_encoded_data(&source_acl.encoded_data()?)?;
    let target_acl_entry = target_acl.entry(0)?;
    assert_eq!(target_acl_entry.tag, acl_entry.tag);
    assert_eq!(target_acl_entry.qualifier_type, acl_entry.qualifier_type);
    assert_eq!(target_acl_entry.qualifier, acl_entry.qualifier);
    decoded_acl.clear()?;
    assert!(decoded_acl.is_empty());

    let source_xat = EntryXatBlob::from_path(&dir, source_name, ArchiveFlags::empty())?;
    assert!(source_xat.entry_count() > 0);
    let xat_entry = source_xat
        .entries()?
        .into_iter()
        .find(|entry| entry.key == "com.example.compression-rs")
        .expect("expected custom xattr entry");
    let mut rebuilt_xat = EntryXatBlob::new()?;
    rebuilt_xat.append_entry(&xat_entry)?;
    rebuilt_xat.set_entry(0, &xat_entry)?;
    let encoded_xat = rebuilt_xat.encoded_data()?;
    let mut decoded_xat = EntryXatBlob::from_encoded_data(&encoded_xat)?;
    decoded_xat.remove_entry(0)?;
    assert!(decoded_xat.is_empty());
    decoded_xat.append_entry(&xat_entry)?;
    decoded_xat.apply_to_path(&dir, target_xat_name, ArchiveFlags::REPLACE_ATTRIBUTES)?;
    let target_xat = EntryXatBlob::from_path(&dir, target_xat_name, ArchiveFlags::empty())?;
    assert!(target_xat.entries()?.contains(&NamedBlobEntry {
        key: xat_entry.key.clone(),
        value: xat_entry.value.clone(),
    }));
    decoded_xat.clear()?;
    assert!(decoded_xat.is_empty());

    Ok(())
}
