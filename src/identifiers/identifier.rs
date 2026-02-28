use crate::exception::ResourceError;
use crate::resources::Resource;
use std::fmt::Display;

pub trait ResourceIdentifier: Display + Send + Sync {
    fn get_type(&self) -> &str;
    fn get_identifier(&self) -> &str;
    fn get_resource(&self) -> Result<Box<dyn Resource>, ResourceError>;
    fn to_relative_identifier(&self, relative_path: &str) -> Box<dyn ResourceIdentifier>;
}
