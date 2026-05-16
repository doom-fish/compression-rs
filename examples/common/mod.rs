#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn pseudo_random_bytes(len: usize) -> Vec<u8> {
    let mut state = 0x0123_4567_89ab_cdef_u64;
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        state ^= state << 7;
        state ^= state >> 9;
        state ^= state << 8;
        bytes.push(state.to_le_bytes()[0]);
    }
    bytes
}

pub fn artifact_dir(name: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let pid = std::process::id();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("example-artifacts")
        .join(format!("{name}-{pid}-{suffix}"));
    fs::create_dir_all(&dir).expect("create example artifact dir");
    dir
}

pub fn path_string(path: &Path) -> String {
    path.to_str().expect("utf-8 path").to_string()
}
