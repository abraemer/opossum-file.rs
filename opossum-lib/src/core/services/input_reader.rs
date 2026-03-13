use crate::core::entities::opossum::Opossum;
use crate::error::OpossumError;

pub trait InputReader {
    fn read(&self) -> Result<Opossum, OpossumError>;
}
