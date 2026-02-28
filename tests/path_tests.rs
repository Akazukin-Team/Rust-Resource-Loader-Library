use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use resource_loader::identifiers::ResourceIdentifier;
use resource_loader::identifiers::path::PathResourceIdentifier;

fn unique_temp_path(prefix: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let file_name = format!("{}_{}.tmp", prefix, now);
    env::temp_dir().join(file_name)
}

#[test]
fn path_type_and_identifier() {
    let path = unique_temp_path("path_type");
    let content = b"type check";
    fs::write(&path, content).expect("write temp file");

    let id = PathResourceIdentifier::new(path.clone());
    assert_eq!(id.get_type(), "path");
    assert_eq!(id.get_identifier(), path.to_str().unwrap());

    let _ = fs::remove_file(&path);
}

#[test]
fn path_to_relative() {
    let path = unique_temp_path("path_rel");
    fs::write(&path, b"x").expect("write temp file");

    let id = PathResourceIdentifier::new(path.clone());
    let rel = id.to_relative_identifier("child.txt");
    assert!(rel.get_identifier().ends_with("child.txt"));

    let _ = fs::remove_file(&path);
}

#[test]
fn path_read() {
    let path = unique_temp_path("path_read");
    let content = b"hello read";
    fs::write(&path, content).expect("write temp file");

    let id = PathResourceIdentifier::new(path.clone());
    let res = id.get_resource().expect("get resource");
    let mut reader = res.get_reader().expect("get reader");
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("read content");
    assert_eq!(buf, content);

    let _ = fs::remove_file(&path);
}

#[test]
fn path_write() {
    let path = unique_temp_path("path_write");
    fs::write(&path, b"").expect("create temp file");

    let id = PathResourceIdentifier::new(path.clone());
    let res = id.get_resource().expect("get resource");
    let mut writer = res.get_writer().expect("get writer");
    let new_content = b"written content";
    writer.write_all(new_content).expect("write content");

    let read_back = fs::read(&path).expect("read back");
    assert_eq!(read_back, new_content);

    let _ = fs::remove_file(&path);
}
