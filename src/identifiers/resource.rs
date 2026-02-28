use crate::exception::ResourceError;
use crate::identifiers::ResourceIdentifier;
use crate::resources::Resource;
use crate::resources::embedded::ResourceResource;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ResourceResourceIdentifier {
    identifier: String,
}

impl ResourceResourceIdentifier {
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            identifier: uri.into(),
        }
    }
}

impl Display for ResourceResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "ResourceResourceIdentifier{{type='{}', identifier='{}'}}",
            self.get_type(),
            self.identifier
        )
    }
}

impl ResourceIdentifier for ResourceResourceIdentifier {
    fn get_type(&self) -> &str {
        "resource"
    }

    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_resource(&self) -> Result<Box<dyn Resource>, ResourceError> {
        let path = PathBuf::from(&self.identifier);
        if !path.exists() {
            return Err(ResourceError::NotFound {
                identifier: self.identifier.clone(),
            });
        }

        Ok(Box::new(ResourceResource::new(self.identifier.clone())))
    }

    fn to_relative_identifier(&self, relative_path: &str) -> Box<dyn ResourceIdentifier> {
        if relative_path.is_empty() {
            return Box::new(self.clone());
        }
        Box::new(Self::new(format!("{}/{}", self.identifier, relative_path)))
    }
}
