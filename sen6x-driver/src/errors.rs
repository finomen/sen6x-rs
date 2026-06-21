/// SEN6x errors
#[derive(Debug, Clone, PartialEq, thiserror_no_std::Error)]
pub enum Error<E> {
    /// I²C bus error
    #[error("I2C: {0}")]
    I2c(E),
    /// CRC checksum validation failed
    #[error("CRC")]
    Crc,
    /// Not allowed in current state
    #[error("Not Allowed")]
    NotAllowed,
    /// Invalid value
    #[error("Invalid value")]
    InvalidValue,
}
