mod common;

use common::{artifact_dir, path_string};
use compression::{ArchiveFlags, PathList};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aa-entry-stream");
    fs::create_dir_all(artifact_dir.join("nested"))?;
    fs::write(
        artifact_dir.join("nested").join("example.txt"),
        b"entry stream",
    )?;

    let path_list = PathList::from_directory_contents(
        &path_string(&artifact_dir),
        None,
        ArchiveFlags::empty(),
        0,
    )?;
    let paths = path_list.paths()?;
    assert!(paths.iter().any(|path| path.ends_with("example.txt")));
    println!("paths={paths:?}");
    println!("✅ AppleArchive entry/path-list APIs OK");
    Ok(())
}
