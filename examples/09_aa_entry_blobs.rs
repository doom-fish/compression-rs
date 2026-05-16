mod common;

use common::{artifact_dir, path_string};
use compression::{AceQualifierType, AccessControlEntry, ArchiveFlags, EntryAclBlob, EntryXatBlob};
use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aa-entry-blobs");
    let dir = path_string(&artifact_dir);
    let source_name = "source.txt";
    let target_name = "target.txt";
    let source_path = path_string(&artifact_dir.join(source_name));
    let target_path = path_string(&artifact_dir.join(target_name));
    fs::write(&source_path, b"blob source")?;
    fs::write(&target_path, b"blob target")?;

    Command::new("xattr")
        .args(["-w", "com.example.compression-rs", "example", &source_path])
        .status()?;

    let acl_entry = AccessControlEntry {
        tag: 1,
        perms: 1 << 1,
        flags: 0,
        qualifier_type: AceQualifierType::User,
        qualifier: env::var("USER")?.into_bytes(),
    };
    let mut acl = EntryAclBlob::new()?;
    acl.append_entry(&acl_entry)?;
    let xat = EntryXatBlob::from_path(&dir, source_name, ArchiveFlags::empty())?;
    if !acl.is_empty() {
        let encoded = acl.encoded_data()?;
        let decoded = EntryAclBlob::from_encoded_data(&encoded)?;
        decoded.apply_to_path(&dir, target_name, ArchiveFlags::REPLACE_ATTRIBUTES)?;
    }
    let encoded_xat = xat.encoded_data()?;
    let decoded_xat = EntryXatBlob::from_encoded_data(&encoded_xat)?;
    decoded_xat.apply_to_path(&dir, target_name, ArchiveFlags::REPLACE_ATTRIBUTES)?;

    println!("acl_entries={} xattrs={}", acl.entry_count(), xat.entry_count());
    println!("✅ AppleArchive ACL/XAT blob helpers OK");
    Ok(())
}
