//! This library provides an embedded `no_std` driver for the [Sensirion SEN6x series](https://sensirion.com/media/documents/FAFC548D/693FBB15/PS_DS_SEN6x.pdf).
//! This driver is compatible with `embedded-hal` v1.0.
//!
//! # Errors
//!
//! Every command method returns [`Result`]`<_, `[`Error`]`<E>>`, where `E` is the
//! I²C bus error type of the underlying `embedded-hal` implementation. The
//! [`Error`] variants are:
//!
//! - [`Error::I2c`] — the underlying I²C transfer failed; the bus error is wrapped.
//! - [`Error::Crc`] — a CRC-8 checksum in the sensor's response did not match, indicating a corrupted read.
//! - [`Error::NotAllowed`] — the command is not permitted in the sensor's current
//!   state (for example, reading measured values while idle, or applying a
//!   configuration that is only accepted during measurement).
//! - [`Error::InvalidValue`] — the sensor returned a value outside its defined range.
//!
//! Commands that read a response may return any of the four variants. Commands
//! that only send (with no response) return [`Error::NotAllowed`] or [`Error::I2c`].
//! Each command method also documents its own `# Errors` section.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), no_std)]

pub mod commands;

mod errors;

use core::cell::RefCell;
pub use errors::Error;

mod connection;
#[cfg(feature = "embedded-hal-async")]
mod connection_async;
#[cfg(feature = "embedded-hal")]
mod connection_sync;
mod io;
pub mod types;

use crate::connection::State;
#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-async"))]
use crate::types::Milliseconds;
#[cfg(feature = "embassy")]
use embassy_sync::mutex::Mutex;

/// Driver for a Sensirion SEN6x air-quality sensor over I²C.
///
/// Construct one with [`Sen6x::new`], then drive it through the model-specific
/// command trait for your sensor (e.g. [`Sen66Commands`] / [`Sen66CommandsAsync`]).
/// The driver tracks whether the sensor is idle or measuring and rejects commands
/// that are not valid in the current state (see the crate-level `# Errors` docs).
///
/// # Thread safety
/// `Sen6x` uses [`core::cell::RefCell`] for interior mutability, so it is
/// [`Send`] but not [`Sync`]. The driver is intended to be owned by a single
/// task; sharing one instance across threads is not supported. For shared-bus
/// setups, use the `embassy` feature, which guards the bus with a `Mutex`.
#[derive(Debug)]
pub struct Sen6x<'a, I2C, D> {
    i2c: I2C,
    delay: RefCell<&'a mut D>,
    state: State,
}

#[cfg(feature = "embedded-hal")]
trait SensorConnectionSync {
    type I2c: embedded_hal::i2c::I2c<Error = Self::Error>;
    type Delay: embedded_hal::delay::DelayNs;
    type Error;
    fn transaction<R>(&self, f: impl FnOnce(&mut Self::I2c) -> R) -> R;
    fn delay(&self, delay: Milliseconds);
}

#[cfg(feature = "embedded-hal-async")]
trait SensorConnectionAsync {
    type I2c: embedded_hal_async::i2c::I2c<Error = Self::Error>;
    type Delay: embedded_hal_async::delay::DelayNs;
    type Error;
    async fn transaction<R>(&self, f: impl AsyncFnOnce(&mut Self::I2c) -> R) -> R;
    async fn delay(&self, delay: Milliseconds);
}

trait SensorState<E> {
    fn check_state(&self, valid_in: &[State]) -> Result<(), crate::Error<E>>;

    fn state(&mut self) -> &mut State;
}

impl<'a, I2C, D, E> SensorState<E> for Sen6x<'a, I2C, D> {
    fn check_state(&self, valid_in: &[State]) -> Result<(), crate::Error<E>> {
        if !valid_in.contains(&self.state) {
            return Err(crate::Error::NotAllowed);
        }
        Ok(())
    }

    fn state(&mut self) -> &mut State {
        &mut self.state
    }
}

#[cfg(feature = "embassy")]
impl<'a, M, I2C, D, E> SensorConnectionAsync
    for Sen6x<'a, &'a embassy_sync::mutex::Mutex<M, I2C>, D>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
    M: embassy_sync::blocking_mutex::raw::RawMutex,
    D: embedded_hal_async::delay::DelayNs,
{
    type I2c = I2C;
    type Error = E;
    type Delay = D;

    async fn transaction<R>(&self, f: impl AsyncFnOnce(&mut I2C) -> R) -> R {
        let mut i2c = self.i2c.lock().await;
        f(&mut *i2c).await
    }

    // The delay future borrows the timer across the await. Sound because
    // `Sen6x` is a single-owner, `!Sync` driver (see "Thread safety" above),
    // so no concurrent borrow of this cell can exist.
    #[allow(clippy::await_holding_refcell_ref)]
    async fn delay(&self, delay: Milliseconds) {
        let mut d = self.delay.borrow_mut();
        d.delay_ms(delay as u32).await;
    }
}

mod sealed {
    pub trait Sealed {}
    #[cfg(any(feature = "embedded-hal", feature = "embedded-hal-async"))]
    impl<I2C> Sealed for &mut I2C {}
    #[cfg(feature = "embassy")]
    impl<M, I2C> Sealed for &embassy_sync::mutex::Mutex<M, I2C> where
        M: embassy_sync::blocking_mutex::raw::RawMutex
    {
    }
}

/// Conversion of an I²C bus handle into the connection type stored by [`Sen6x`].
/// This is an implementation detail of [`Sen6x::new`]
#[doc(hidden)]
pub trait IntoI2cConnection<'a>: sealed::Sealed {
    type Connection;
    fn into_i2c_connection(self) -> Self::Connection;
}

#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-async"))]
impl<'a, I2C: 'a> IntoI2cConnection<'a> for &'a mut I2C {
    type Connection = RefCell<&'a mut I2C>;
    fn into_i2c_connection(self) -> Self::Connection {
        RefCell::new(self)
    }
}

#[cfg(feature = "embassy")]
impl<'a, M, I2C> IntoI2cConnection<'a> for &'a Mutex<M, I2C>
where
    M: embassy_sync::blocking_mutex::raw::RawMutex,
{
    type Connection = &'a Mutex<M, I2C>;
    fn into_i2c_connection(self) -> Self::Connection {
        self
    }
}

impl<'a, C, D> Sen6x<'a, C, D> {
    /// Creates a driver from an I²C bus and a delay provider.
    ///
    /// `i2c` is either an exclusive `&mut` to an [`connection_sync::i2c::I2c`] /
    /// [`embedded_hal_async::i2c::I2c`] implementation, or — with the `embassy`
    /// feature — a shared `&embassy_sync::mutex::Mutex<_, I2C>` for buses shared
    /// with other drivers. `delay` provides the post-command wait each operation
    /// needs. The sensor starts in the idle state.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "embedded-hal", doc = "```")]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction};"
    )]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "use embedded_hal_mock::eh1::delay::NoopDelay;"
    )]
    #[cfg_attr(feature = "embedded-hal", doc = "use sen6x::{Sen6x, Sen66Commands};")]
    #[cfg_attr(feature = "embedded-hal", doc = "")]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "// The SEN6x lives at I²C address 0x6B; starting a measurement writes command 0x0021."
    )]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "let mut i2c = I2cMock::new(&[Transaction::write(0x6B, vec![0x00, 0x21])]);"
    )]
    #[cfg_attr(feature = "embedded-hal", doc = "let mut delay = NoopDelay::new();")]
    #[cfg_attr(feature = "embedded-hal", doc = "")]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "let mut sensor = Sen6x::new(&mut i2c, &mut delay);"
    )]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "sensor.start_continuous_measurement()?;"
    )]
    #[cfg_attr(feature = "embedded-hal", doc = "")]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "i2c.done(); // all expected I²C traffic happened"
    )]
    #[cfg_attr(
        feature = "embedded-hal",
        doc = "# Ok::<(), sen6x::Error<embedded_hal::i2c::ErrorKind>>(())"
    )]
    #[cfg_attr(feature = "embedded-hal", doc = "```")]
    pub fn new<I2C>(i2c: I2C, delay: &'a mut D) -> Self
    where
        I2C: IntoI2cConnection<'a, Connection = C>,
    {
        Self {
            i2c: i2c.into_i2c_connection(),
            delay: RefCell::new(delay),
            state: State::Idle,
        }
    }
}

#[cfg(feature = "embedded-hal")]
pub use commands::{
    Sen62Commands, Sen63cCommands, Sen65Commands, Sen66Commands, Sen68Commands, Sen69cCommands,
};
#[cfg(feature = "embedded-hal-async")]
pub use commands::{
    Sen62CommandsAsync, Sen63cCommandsAsync, Sen65CommandsAsync, Sen66CommandsAsync,
    Sen68CommandsAsync, Sen69cCommandsAsync,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sen6x_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Sen6x<'static, RefCell<&'static mut u8>, u8>>();
    }
}
