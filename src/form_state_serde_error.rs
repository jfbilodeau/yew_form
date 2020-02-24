use std::string::ToString;
use std::fmt::Display;
use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum ModelSerializerError {
    Message(String),
}

impl serde::ser::Error for ModelSerializerError {
    fn custom<T: Display>(msg: T) -> Self {
        ModelSerializerError::Message(msg.to_string())
    }
}

impl serde::de::Error for ModelSerializerError {
    fn custom<T: Display>(msg: T) -> Self {
        ModelSerializerError::Message(msg.to_string())
    }
}


impl Display for ModelSerializerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for ModelSerializerError {
    fn description(&self) -> &str {
        match *self {
            ModelSerializerError::Message(ref msg) => msg,
        }
    }
}

