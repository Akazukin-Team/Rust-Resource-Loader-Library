use crate::exception::ResourceError;
use crate::identifiers::ResourceIdentifier;
use crate::identifiers::resource::ResourceResourceIdentifier;
use crate::resources::Resource;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct ResourceResource {
    identifier: String,
}

impl ResourceResource {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}

impl Resource for ResourceResource {
    fn get_type(&self) -> &str {
        "input-stream"
    }

    fn get_identifier(&self) -> Box<dyn ResourceIdentifier> {
        Box::new(ResourceResourceIdentifier::new(self.identifier.clone()))
    }

    fn get_reader(&self) -> Result<Box<dyn Read>, ResourceError> {
        let path = PathBuf::from(&self.identifier);
        if !path.exists() {
            return Err(ResourceError::NotFound {
                identifier: self.identifier.clone(),
            });
        }
        File::open(path)
            .map(|f| Box::new(f) as Box<dyn Read>)
            .map_err(|e| ResourceError::Fetch {
                identifier: self.identifier.clone(),
                cause: e.to_string(),
            })
    }

    fn get_writer(&self) -> Result<Box<dyn Write>, ResourceError> {
        Err(ResourceError::UnsupportedOperation(
            "Resource implementation does not support output streams.".to_string(),
        ))
    }
}
