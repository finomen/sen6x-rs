use crate::commands::CommandId;
use crate::io::{FromBytes, ToBytes};
use crate::types::Milliseconds;

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Idle,
    Measurement,
}

#[cfg(feature = "embedded-hal")]
pub(crate) trait Sen6xConnection<E> {
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
    use super::State;
    use crate::commands::CommandId;
    use crate::io::{FromBytes, ToBytes};
    use crate::types::Milliseconds;
    pub(crate) trait Sen6xConnection<E> {
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
