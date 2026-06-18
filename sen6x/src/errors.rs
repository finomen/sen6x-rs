/// SEN6x errors
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
pub enum Error<E> {
    #[cfg_attr(feature = "thiserror", error("I2C: {0}"))]
    /// I²C bus error
    I2c(E),
    #[cfg_attr(feature = "thiserror", error("CRC"))]
    /// CRC checksum validation failed
    Crc,
    #[cfg_attr(feature = "thiserror", error("Not Allowed"))]
    /// Not allowed in current state
    NotAllowed,
    #[cfg_attr(feature = "thiserror", error("Invalid value"))]
    /// Invalid value
    InvalidValue,
}
