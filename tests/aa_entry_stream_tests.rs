mod support;

use compression::{ArchiveFlags, PathList};
use std::fs;
use support::{artifact_dir, path_string};

#[test]
fn path_list_discovers_nested_files() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aa-entry-stream-test");
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
    Ok(())
}
