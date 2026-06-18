//! This library provides an embedded `no_std` driver for the [Sensirion SEN6x series](https://sensirion.com/media/documents/FAFC548D/693FBB15/PS_DS_SEN6x.pdf).
//! This driver is compatible with `embedded-hal` v1.0.
#![cfg_attr(not(test), no_std)]

mod sen5x;

pub mod commands;

mod errors;

use core::cell::RefCell;
pub use errors::Error;

mod connection;
#[cfg(feature = "embedded-hal")]
mod embeded_hal;
#[cfg(feature = "embedded-hal-async")]
mod embeded_hal_async;
mod io;
pub mod types;

pub use sen6x_macros::SenRead;

use crate::commands::CommandId;
use crate::connection::State;
use crate::io::{FromBytes, ToBytes};
use crate::types::Milliseconds;
#[cfg(feature = "embassy")]
use embassy_sync::mutex::{Mutex, MutexGuard};

use embedded_hal::i2c::Operation;

pub struct Sen6x<'a, I2C, D> {
    i2c: I2C,
    delay: RefCell<&'a mut D>,
    state: State,
}

const SEN6X_I2C_ADDRESS: u8 = 0x6B;

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

    async fn delay(&self, delay: Milliseconds) {
        self.delay.borrow_mut().delay_ms(delay as u32).await;
    }
}
pub trait IntoI2cConnection<'a> {
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
pub use commands::Sen6xCommands;
#[cfg(feature = "embedded-hal-async")]
pub use commands::Sen6xCommandsAsync;
