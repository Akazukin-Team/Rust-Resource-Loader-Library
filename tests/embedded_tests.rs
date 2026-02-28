use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use resource_loader::identifiers::ResourceIdentifier;
use resource_loader::identifiers::resource::ResourceResourceIdentifier;

fn unique_temp_path(prefix: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let file_name = format!("{}_{}.tmp", prefix, now);
    env::temp_dir().join(file_name)
}

#[test]
fn embedded_read() {
    let path = unique_temp_path("res_res");
    let content = b"embedded content";
    fs::write(&path, content).expect("write temp file");

    let id = ResourceResourceIdentifier::new(path.to_str().unwrap());
    assert_eq!(id.get_type(), "resource");
    assert_eq!(id.get_identifier(), path.to_str().unwrap());

    let res = id.get_resource().expect("get resource");
    let mut reader = res.get_reader().expect("get reader");
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("read content");
    assert_eq!(buf, content);

    let _ = fs::remove_file(&path);
}
