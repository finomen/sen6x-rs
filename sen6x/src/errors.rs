/// SEN6x errors
#[derive(Debug, Clone, PartialEq, thiserror_no_std::Error)]
pub enum Error<E> {
    #[error("I2C: {0}")]
    /// I²C bus error
    I2c(E),
    #[error("CRC")]
    /// CRC checksum validation failed
    Crc,
    #[error("Not Allowed")]
    /// Not allowed in current state
    NotAllowed,
    #[error("Invalid value")]
    /// Invalid value
    InvalidValue,
}
