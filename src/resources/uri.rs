use crate::exception::ResourceError;
use crate::identifiers::ResourceIdentifier;
use crate::identifiers::uri::UriResourceIdentifier;
use crate::resources::Resource;
use std::io::{Cursor, Read, Write};

pub struct UriResource {
    identifier: String,
    data: Vec<u8>,
}

impl UriResource {
    pub fn new(identifier: String, data: Vec<u8>) -> Self {
        Self { identifier, data }
    }
}

impl Resource for UriResource {
    fn get_type(&self) -> &str {
        "uri"
    }

    fn get_identifier(&self) -> Box<dyn ResourceIdentifier> {
        Box::new(UriResourceIdentifier::new(self.identifier.clone(), true))
    }

    fn get_reader(&self) -> Result<Box<dyn Read>, ResourceError> {
        Ok(Box::new(Cursor::new(self.data.clone())))
    }

    fn get_writer(&self) -> Result<Box<dyn Write>, ResourceError> {
        Err(ResourceError::UnsupportedOperation(
            "URI resources do not support writing".to_string(),
        ))
    }
}
