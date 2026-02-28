use crate::exception::ResourceError;
use crate::identifiers::ResourceIdentifier;
use crate::resources::Resource;
use crate::resources::uri::UriResource;
use reqwest::blocking;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct UriResourceIdentifier {
    identifier: String,
    ssl: bool,
}

impl UriResourceIdentifier {
    pub fn new(uri: impl Into<String>, ssl: bool) -> Self {
        Self {
            identifier: uri.into(),
            ssl,
        }
    }
}

impl Display for UriResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "UriResourceIdentifier{{type='{}', identifier='{}', ssl={}}}",
            self.get_type(),
            self.identifier,
            self.ssl
        )
    }
}

impl ResourceIdentifier for UriResourceIdentifier {
    fn get_type(&self) -> &str {
        "uri"
    }

    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_resource(&self) -> Result<Box<dyn Resource>, ResourceError> {
        let res = match blocking::get(&self.identifier) {
            Ok(r) => r,
            Err(e) => {
                return Err(ResourceError::Fetch {
                    identifier: self.identifier.clone(),
                    cause: e.to_string(),
                });
            }
        };

        if !res.status().is_success() {
            return Err(ResourceError::Fetch {
                identifier: self.identifier.clone(),
                cause: format!("HTTP status {}", res.status()),
            });
        }

        let bytes = match res.bytes() {
            Ok(b) => b.to_vec(),
            Err(e) => {
                return Err(ResourceError::Fetch {
                    identifier: self.identifier.clone(),
                    cause: e.to_string(),
                });
            }
        };

        Ok(Box::new(UriResource::new(self.identifier.clone(), bytes)))
    }

    fn to_relative_identifier(&self, relative_path: &str) -> Box<dyn ResourceIdentifier> {
        if relative_path.is_empty() {
            return Box::new(self.clone());
        }
        Box::new(Self::new(
            format!("{}/{}", self.identifier, relative_path),
            self.ssl,
        ))
    }
}
