use std::io::Read;

use resource_loader::identifiers::ResourceIdentifier;
use resource_loader::identifiers::uri::UriResourceIdentifier;
use resource_loader::resources::Resource;
use resource_loader::resources::uri::UriResource;

#[test]
fn uri_reader() {
    let data = b"test-bytes".to_vec();
    let res = UriResource::new("urn:test".to_string(), data.clone());

    assert_eq!(res.get_type(), "uri");
    let id = res.get_identifier();
    assert!(id.get_identifier().contains("urn:test"));

    let mut reader = res.get_reader().expect("get reader");
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("read");
    assert_eq!(buf, data);
}

#[test]
fn uri_fetch() {
    let id = UriResourceIdentifier::new("https://example.com".to_string(), true);
    let res = id.get_resource().expect("get resource");
    let mut reader = res.get_reader().expect("get reader");
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("read");
    assert!(!buf.is_empty());
}
