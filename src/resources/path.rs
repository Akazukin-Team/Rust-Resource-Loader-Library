use crate::exception::ResourceError;
use crate::identifiers::ResourceIdentifier;
use crate::identifiers::path::PathResourceIdentifier;
use crate::resources::Resource;
use std::fs::File;
use std::io::{Read, Write};

pub struct PathResource {
    identifier: PathResourceIdentifier,
}

impl PathResource {
    pub fn new(identifier: PathResourceIdentifier) -> Self {
        Self { identifier }
    }
}

impl Resource for PathResource {
    fn get_type(&self) -> &str {
        self.identifier.get_type()
    }

    fn get_identifier(&self) -> Box<dyn ResourceIdentifier> {
        Box::new(self.identifier.clone())
    }

    fn get_reader(&self) -> Result<Box<dyn Read>, ResourceError> {
        let path = self.identifier.get_identifier();
        File::open(path)
            .map(|f| Box::new(f) as Box<dyn Read>)
            .map_err(|e| ResourceError::Fetch {
                identifier: path.to_string(),
                cause: e.to_string(),
            })
    }

    fn get_writer(&self) -> Result<Box<dyn Write>, ResourceError> {
        let path = self.identifier.get_identifier();
        File::create(path)
            .map(|f| Box::new(f) as Box<dyn Write>)
            .map_err(|e| ResourceError::Fetch {
                identifier: path.to_string(),
                cause: e.to_string(),
            })
    }
}
