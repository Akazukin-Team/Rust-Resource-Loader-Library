use crate::exception::ResourceError;
use crate::identifiers::ResourceIdentifier;
use crate::resources::Resource;
use crate::resources::path::PathResource;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PathResourceIdentifier {
    path: PathBuf,
}

impl PathResourceIdentifier {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl Display for PathResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "PathResourceIdentifier{{type='{}', identifier='{}'}}",
            self.get_type(),
            self.path.display()
        )
    }
}

impl ResourceIdentifier for PathResourceIdentifier {
    fn get_type(&self) -> &str {
        "path"
    }

    fn get_identifier(&self) -> &str {
        self.path.to_str().unwrap_or("")
    }

    fn get_resource(&self) -> Result<Box<dyn Resource>, ResourceError> {
        if !self.path.exists() {
            return Err(ResourceError::NotFound {
                identifier: self.get_identifier().to_string(),
            });
        }
        Ok(Box::new(PathResource::new(self.clone())))
    }

    fn to_relative_identifier(&self, relative_path: &str) -> Box<dyn ResourceIdentifier> {
        if relative_path.is_empty() {
            return Box::new(self.clone());
        }
        Box::new(Self::new(self.path.join(relative_path)))
    }
}
