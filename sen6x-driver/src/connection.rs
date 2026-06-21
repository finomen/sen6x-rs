#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-async"))]
use crate::commands::CommandId;
#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-async"))]
use crate::io::{FromBytes, ToBytes};
#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-async"))]
use crate::types::Milliseconds;

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Idle,
    Measurement,
}

pub(crate) trait SensorModel {
    const I2C_ADDRESS: u8;
}

macro_rules! sensor_model {
    ($model:ident, $addr:literal) => {
        pub struct $model;
        impl SensorModel for $model {
            const I2C_ADDRESS: u8 = $addr;
        }
    };
}

sensor_model!(Sen62, 0x6B);
sensor_model!(Sen63c, 0x6B);
sensor_model!(Sen65, 0x6B);
sensor_model!(Sen66, 0x6B);
sensor_model!(Sen68, 0x6B);
sensor_model!(Sen69c, 0x6B);

#[cfg(feature = "embedded-hal")]
pub(crate) trait Sen6xConnection<S, E>
where
    S: SensorModel,
{
    fn send(
        &mut self,
        md: CommandId,
        execution_time: Milliseconds,
        valid_in: &[State],
    ) -> Result<(), crate::errors::Error<E>>;
    fn write<const N: usize, Tx: ToBytes<N>>(
        &mut self,
        cmd: CommandId,
        execution_time: Milliseconds,
        data: Tx,
        valid_in: &[State],
    ) -> Result<(), crate::errors::Error<E>>;
    fn read<const N: usize, Rx: FromBytes<N, Rx>>(
        &mut self,
        cmd: CommandId,
        execution_time: Milliseconds,
        valid_in: &[State],
    ) -> Result<Rx, crate::errors::Error<E>>;
    fn fetch<const NT: usize, const NR: usize, Rx: FromBytes<NR, Rx>, Tx: ToBytes<NT>>(
        &mut self,
        cmd: CommandId,
        execution_time: Milliseconds,
        data: Tx,
        valid_in: &[State],
    ) -> Result<Rx, crate::errors::Error<E>>;
    fn update_state(&mut self, state: State);
}

#[cfg(feature = "embedded-hal-async")]
pub mod hal_async {
    use super::{SensorModel, State};
    use crate::commands::CommandId;
    use crate::io::{FromBytes, ToBytes};
    use crate::types::Milliseconds;
    pub(crate) trait Sen6xConnection<S, E>
    where
        S: SensorModel,
    {
        async fn send(
            &mut self,
            cmd: CommandId,
            execution_time: Milliseconds,
            valid_in: &[State],
        ) -> Result<(), crate::errors::Error<E>>;
        async fn write<const N: usize, Tx: ToBytes<N>>(
            &mut self,
            cmd: CommandId,
            execution_time: Milliseconds,
            data: Tx,
            valid_in: &[State],
        ) -> Result<(), crate::errors::Error<E>>;
        async fn read<const N: usize, Rx: FromBytes<N, Rx>>(
            &mut self,
            cmd: CommandId,
            execution_time: Milliseconds,
            valid_in: &[State],
        ) -> Result<Rx, crate::errors::Error<E>>;
        async fn fetch<const NR: usize, const NT: usize, Tx: ToBytes<NT>, Rx: FromBytes<NR, Rx>>(
            &mut self,
            cmd: CommandId,
            execution_time: Milliseconds,
            data: Tx,
            valid_in: &[State],
        ) -> Result<Rx, crate::errors::Error<E>>;
        fn update_state(&mut self, state: State);
    }
}
