use crate::frame::Frame;
use std::io::Error;

pub trait Content {
    fn content(&self) -> Result<Vec<u8>, Error>;
}
