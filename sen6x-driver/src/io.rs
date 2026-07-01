use crate::Error;
use sensirion_i2c::crc8::calculate;
use units::quantity::Quantity;
use units::unit::{Unit, UnitTags};

pub(crate) trait FromBytes<const N: usize, R> {
    fn from_bytes_with_crc<E>(bytes: &[u8; N]) -> Result<R, Error<E>>;
}

pub(crate) const fn get_size<O, F, const N: usize>(_: fn(x: O) -> F) -> usize
where
    F: FromBytes<N, F>,
{
    N
}

impl FromBytes<2, u16> for u16 {
    fn from_bytes_with_crc<E>(bytes: &[u8; 2]) -> Result<u16, Error<E>> {
        Ok(u16::from_be_bytes(*bytes))
    }
}
impl FromBytes<2, i16> for i16 {
    fn from_bytes_with_crc<E>(bytes: &[u8; 2]) -> Result<i16, Error<E>> {
        Ok(i16::from_be_bytes(*bytes))
    }
}

impl FromBytes<4, u32> for u32 {
    fn from_bytes_with_crc<E>(bytes: &[u8; 4]) -> Result<u32, Error<E>> {
        Ok(u32::from_be_bytes(*bytes))
    }
}

impl FromBytes<1, u8> for u8 {
    fn from_bytes_with_crc<E>(bytes: &[u8; 1]) -> Result<u8, Error<E>> {
        Ok(bytes[0])
    }
}

impl FromBytes<2, Option<i16>> for Option<i16> {
    fn from_bytes_with_crc<E>(bytes: &[u8; 2]) -> Result<Option<i16>, Error<E>> {
        let res = i16::from_bytes_with_crc(bytes)?;
        if res == i16::MAX {
            return Ok(None);
        }
        Ok(Some(res))
    }
}

impl FromBytes<2, Option<u16>> for Option<u16> {
    fn from_bytes_with_crc<E>(bytes: &[u8; 2]) -> Result<Option<u16>, Error<E>> {
        let res = u16::from_bytes_with_crc(bytes)?;
        if res == u16::MAX {
            return Ok(None);
        }
        Ok(Some(res))
    }
}

impl FromBytes<2, bool> for bool {
    fn from_bytes_with_crc<E>(bytes: &[u8; 2]) -> Result<bool, Error<E>> {
        match bytes[1] {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(Error::InvalidValue),
        }
    }
}

pub(super) trait ToBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

impl ToBytes<2> for bool {
    fn to_bytes(&self) -> [u8; 2] {
        [0x00, *self as u8]
    }
}
impl ToBytes<2> for u16 {
    fn to_bytes(&self) -> [u8; 2] {
        self.to_be_bytes()
    }
}

impl ToBytes<2> for i16 {
    fn to_bytes(&self) -> [u8; 2] {
        self.to_be_bytes()
    }
}

impl ToBytes<2> for Option<i16> {
    fn to_bytes(&self) -> [u8; 2] {
        self.unwrap_or(0x7FFF).to_be_bytes()
    }
}

impl ToBytes<2> for Option<u16> {
    fn to_bytes(&self) -> [u8; 2] {
        self.unwrap_or(0xFFFF).to_be_bytes()
    }
}

impl ToBytes<1> for u8 {
    fn to_bytes(&self) -> [u8; 1] {
        [*self]
    }
}

impl<const N: usize> ToBytes<N> for [u8; N] {
    fn to_bytes(&self) -> [u8; N] {
        *self
    }
}

pub(crate) fn check_crc<const N: usize, E>(bytes: &[u8]) -> Result<[u8; N], Error<E>> {
    assert_eq!(N % 2, 0);
    assert_eq!(bytes.len() * 2 / 3, N);
    let mut dest = [0u8; N];
    for i in 0..N / 2 {
        if calculate(&bytes[i * 3..i * 3 + 2]) != bytes[i * 3 + 2] {
            return Err(Error::Crc);
        }
        for j in 0..2 {
            dest[i * 2 + j] = bytes[i * 3 + j];
        }
    }
    Ok(dest)
}

pub(crate) trait ValueWrapper {
    type Inner;
    fn wrap(value: Self::Inner) -> Self;
    fn unwrap(&self) -> Self::Inner;
}

impl<T, US> ValueWrapper for Quantity<T, US>
where
    T: Copy,
    US: Unit + UnitTags,
{
    type Inner = T;
    fn wrap(value: Self::Inner) -> Self {
        Self::new(value)
    }
    fn unwrap(&self) -> Self::Inner {
        self.value()
    }
}

impl<const N: usize, W> FromBytes<N, W> for W
where
    W: ValueWrapper,
    W::Inner: FromBytes<N, W::Inner>,
{
    fn from_bytes_with_crc<E>(bytes: &[u8; N]) -> Result<W, Error<E>> {
        W::Inner::from_bytes_with_crc(bytes).map(W::wrap)
    }
}

impl<const N: usize, W> FromBytes<N, Option<W>> for Option<W>
where
    W: ValueWrapper,
    Option<W::Inner>: FromBytes<N, Option<W::Inner>>,
{
    fn from_bytes_with_crc<E>(bytes: &[u8; N]) -> Result<Option<W>, Error<E>> {
        Option::<W::Inner>::from_bytes_with_crc(bytes).map(|v| v.map(W::wrap))
    }
}

impl<const N: usize, W> ToBytes<N> for W
where
    W: ValueWrapper,
    W::Inner: ToBytes<N>,
{
    fn to_bytes(&self) -> [u8; N] {
        self.unwrap().to_bytes()
    }
}

impl<const N: usize, W> ToBytes<N> for Option<W>
where
    W: ValueWrapper,
    Option<W::Inner>: ToBytes<N>,
{
    fn to_bytes(&self) -> [u8; N] {
        self.as_ref().map(|v| v.unwrap()).to_bytes()
    }
}
