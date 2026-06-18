use crate::{Sen6x,  SensorConnectionAsync};
use crate::types::Milliseconds;

impl<'a, M, I2C,D, E> SensorConnectionAsync for Sen6x<'a, embassy_sync::mutex::Mutex<M, I2C>, D>
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