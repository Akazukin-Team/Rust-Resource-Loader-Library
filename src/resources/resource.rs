use crate::exception::ResourceError;
use crate::identifiers::ResourceIdentifier;
use std::io::{Read, Write};

pub trait Resource: Send + Sync {
    fn get_type(&self) -> &str;
    fn get_identifier(&self) -> Box<dyn ResourceIdentifier>;
    fn get_reader(&self) -> Result<Box<dyn Read>, ResourceError>;
    fn get_writer(&self) -> Result<Box<dyn Write>, ResourceError>;
}
