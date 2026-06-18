use crate::commands::CommandId;
use crate::connection::State;
use crate::io::{FromBytes, ToBytes};
use crate::types::Milliseconds;
use crate::{SEN6X_I2C_ADDRESS, Sen6x, SensorConnectionAsync, SensorState, errors};
use core::cell::RefCell;
use embedded_hal::i2c::Operation;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

impl<'a, I2C, D, E> SensorConnectionAsync for Sen6x<'a, RefCell<&'a mut I2C>, D>
where
    I2C: I2c<Error = E>,
    D: DelayNs,
{
    type I2c = I2C;
    type Error = E;
    type Delay = D;

    async fn transaction<R>(&self, f: impl AsyncFnOnce(&mut I2C) -> R) -> R {
        let mut i2c = self.i2c.borrow_mut();
        f(&mut *i2c).await
    }

    async fn delay(&self, delay: Milliseconds) {
        self.delay.borrow_mut().delay_ms(delay as u32).await;
    }
}

impl<T> crate::connection::hal_async::Sen6xConnection<T::Error> for T
where
    T: SensorConnectionAsync + SensorState<T::Error>,
{
    async fn send(
        &mut self,
        cmd: CommandId,
        execution_time: Milliseconds,
        valid_in: &[State],
    ) -> Result<(), crate::Error<T::Error>> {
        self.check_state(valid_in)?;
        let this = &*self;
        this.transaction(async move |i2c| {
            i2c.write(SEN6X_I2C_ADDRESS, &(cmd as u16).to_be_bytes())
                .await
                .map_err(|e| errors::Error::I2c(e))?;
            this.delay(execution_time).await;
            Ok(())
        })
        .await
    }

    async fn write<const N: usize, Tx: ToBytes<N>>(
        &mut self,
        cmd: CommandId,
        execution_time: Milliseconds,
        data: Tx,
        valid_in: &[State],
    ) -> Result<(), crate::Error<T::Error>> {
        self.check_state(valid_in)?;
        let this = &*self;

        this.transaction(async move |i2c| {
            i2c.transaction(
                SEN6X_I2C_ADDRESS,
                &mut [
                    Operation::Write(&(cmd as u16).to_be_bytes()),
                    Operation::Write(&data.to_bytes()),
                ],
            )
            .await
            .map_err(|e| errors::Error::I2c(e))?;
            this.delay(execution_time).await;
            Ok(())
        })
        .await
    }

    async fn read<const N: usize, Rx: FromBytes<N, Rx>>(
        &mut self,
        cmd: CommandId,
        execution_time: Milliseconds,
        valid_in: &[State],
    ) -> Result<Rx, crate::Error<T::Error>> {
        self.check_state(valid_in)?;
        let this = &*self;
        this.transaction(async move |i2c| {
            i2c.write(SEN6X_I2C_ADDRESS, &(cmd as u16).to_be_bytes())
                .await
                .map_err(|e| errors::Error::I2c(e))?;
            this.delay(execution_time).await;
            let mut buffer = [0u8; N];
            i2c.read(SEN6X_I2C_ADDRESS, &mut buffer)
                .await
                .map_err(|e| errors::Error::I2c(e))?;
            Rx::from_bytes_with_crc(&buffer)
        })
        .await
    }

    async fn fetch<const NR: usize, const NT: usize, Tx: ToBytes<NT>, Rx: FromBytes<NR, Rx>>(
        &mut self,
        cmd: CommandId,
        execution_time: Milliseconds,
        data: Tx,
        valid_in: &[State],
    ) -> Result<Rx, crate::Error<T::Error>> {
        self.check_state(valid_in)?;
        let this = &*self;

        this.transaction(async move |i2c| {
            i2c.transaction(
                SEN6X_I2C_ADDRESS,
                &mut [
                    Operation::Write(&(cmd as u16).to_be_bytes()),
                    Operation::Write(&data.to_bytes()),
                ],
            )
            .await
            .map_err(|e| errors::Error::I2c(e))?;
            this.delay(execution_time).await;
            let mut buffer = [0u8; NR];
            i2c.read(SEN6X_I2C_ADDRESS, &mut buffer)
                .await
                .map_err(|e| errors::Error::I2c(e))?;
            Rx::from_bytes_with_crc(&buffer)
        })
        .await
    }

    fn update_state(&mut self, state: State) {
        *self.state() = state;
    }
}
