use crate::io::{FromBytes, ValueWrapper};
use crate::{Error, io};
use bitrs::layout;
use fixed_str::FixedStr;
use sen6x_macros::SenRead;

pub(crate) type Milliseconds = u16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MicrogramsPerCubicMeter {
    value: u16,
}
impl From<MicrogramsPerCubicMeter> for f32 {
    fn from(value: MicrogramsPerCubicMeter) -> f32 {
        value.value as f32 / 10f32
    }
}


impl ValueWrapper for MicrogramsPerCubicMeter {
    type Inner = u16;
    fn wrap(value: u16) -> Self {
        MicrogramsPerCubicMeter { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent {
    value: i16,
}

impl From<Percent> for f32 {
    fn from(value: Percent) -> f32 {
        value.value as f32 / 100f32
    }
}

impl ValueWrapper for Percent {
    type Inner = i16;
    fn wrap(value: i16) -> Self {
        Percent { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DegCelsius {
    value: i16,
}
impl From<DegCelsius> for f32 {
    fn from(value: DegCelsius) -> f32 {
        value.value as f32 / 200f32
    }
}

impl ValueWrapper for DegCelsius {
    type Inner = i16;
    fn wrap(value: i16) -> Self {
        DegCelsius { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index {
    value: i16,
}

impl From<Index> for f32 {
    fn from(value: Index) -> f32 {
        value.value as f32 / 10f32
    }
}

impl ValueWrapper for Index {
    type Inner = i16;
    fn wrap(value: i16) -> Self {
        Index { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ppm {
    value: i16,
}

impl From<Ppm> for f32 {
    fn from(value: Ppm) -> f32 {
        value.value as f32
    }
}

impl ValueWrapper for Ppm {
    type Inner = i16;
    fn wrap(value: i16) -> Self {
        Ppm { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct PpmU16 {
    value: u16,
}

impl From<PpmU16> for f32 {
    fn from(value: PpmU16) -> f32 {
        value.value as f32
    }
}

impl ValueWrapper for PpmU16 {
    type Inner = u16;
    fn wrap(value: u16) -> Self {
        PpmU16 { value }
    }

    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ppb {
    value: i16,
}

impl From<Ppb> for f32 {
    fn from(value: Ppb) -> f32 {
        value.value as f32 / 10f32
    }
}

impl ValueWrapper for Ppb {
    type Inner = i16;
    fn wrap(value: i16) -> Self {
        Ppb { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParticlesPerCm3 {
    value: u16,
}

impl From<ParticlesPerCm3> for f32 {
    fn from(value: ParticlesPerCm3) -> f32 {
        value.value as f32 / 10f32
    }
}

impl ValueWrapper for ParticlesPerCm3 {
    type Inner = u16;
    fn wrap(value: u16) -> Self {
        ParticlesPerCm3 { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

// hectapascals
#[derive(Debug, Copy, Clone)]
pub struct Hpa {
    value: u16,
}

impl Hpa {
    pub fn new(value: u16) -> Self {
        Hpa { value }
    }
}

impl From<Hpa> for f32 {
    fn from(value: Hpa) -> f32 {
        value.value as f32
    }
}

impl ValueWrapper for Hpa {
    type Inner = u16;
    fn wrap(value: u16) -> Self {
        Hpa { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Meters {
    value: u16,
}


impl From<Meters> for f32 {
    fn from(value: Meters) -> f32 {
        value.value as f32
    }
}

impl ValueWrapper for Meters {
    type Inner = u16;
    fn wrap(value: u16) -> Self {
        Meters { value }
    }
    fn unwrap(&self) -> Self::Inner {
        self.value
    }
}

pub type ProductName = FixedStr<32>;
pub type SerialNumber = FixedStr<32>;

impl FromBytes<48, FixedStr<32>> for FixedStr<32> {
    fn from_bytes_with_crc<E>(bytes: &[u8; 48]) -> Result<FixedStr<32>, Error<E>> {
        io::check_crc::<32, E>(bytes).map(|v| FixedStr::<32>::from_slice(&v))
    }
}

#[derive(Debug, SenRead, PartialEq)]
pub struct DataReady {
    /// True if data is ready, False if not. When no measurement is running, False will be returned.
    pub data_ready: bool,
}

#[derive(SenRead, Debug, Clone, PartialEq)]
pub struct MeasuredValuesSen62 {
    ///Mass Concentration PM1.0
    pub pm_1_0: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM2.5
    pub pm_2_5: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM4.0
    pub pm_4_0: Option<MicrogramsPerCubicMeter>,
    /// Mass Concentration PM10.0
    pub pm_10_0: Option<MicrogramsPerCubicMeter>,
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
}

#[derive(SenRead, Debug, Clone, PartialEq)]
pub struct MeasuredValuesSen63c {
    ///Mass Concentration PM1.0
    pub pm_1_0: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM2.5
    pub pm_2_5: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM4.0
    pub pm_4_0: Option<MicrogramsPerCubicMeter>,
    /// Mass Concentration PM10.0
    pub pm_10_0: Option<MicrogramsPerCubicMeter>,
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// CO2 concentration
    pub co2: Option<Ppm>,
}
#[derive(SenRead, Debug, Clone, PartialEq)]
pub struct MeasuredValuesSen65 {
    ///Mass Concentration PM1.0
    pub pm_1_0: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM2.5
    pub pm_2_5: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM4.0
    pub pm_4_0: Option<MicrogramsPerCubicMeter>,
    /// Mass Concentration PM10.0
    pub pm_10_0: Option<MicrogramsPerCubicMeter>,
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
}
#[derive(SenRead, Debug, Clone, PartialEq)]
pub struct MeasuredValuesSen66 {
    ///Mass Concentration PM1.0
    pub pm_1_0: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM2.5
    pub pm_2_5: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM4.0
    pub pm_4_0: Option<MicrogramsPerCubicMeter>,
    /// Mass Concentration PM10.0
    pub pm_10_0: Option<MicrogramsPerCubicMeter>,
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// CO2 concentration
    pub co2: Option<Ppm>,
}

#[derive(SenRead, Debug, Clone, PartialEq)]
pub struct MeasuredValuesSen68 {
    ///Mass Concentration PM1.0
    pub pm_1_0: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM2.5
    pub pm_2_5: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM4.0
    pub pm_4_0: Option<MicrogramsPerCubicMeter>,
    /// Mass Concentration PM10.0
    pub pm_10_0: Option<MicrogramsPerCubicMeter>,
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// Formaldehyde concentration
    pub hcho: Option<Ppb>,
}

#[derive(SenRead, Debug, Clone, PartialEq)]
pub struct MeasuredValuesSen69c {
    ///Mass Concentration PM1.0
    pub pm_1_0: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM2.5
    pub pm_2_5: Option<MicrogramsPerCubicMeter>,
    ///Mass Concentration PM4.0
    pub pm_4_0: Option<MicrogramsPerCubicMeter>,
    /// Mass Concentration PM10.0
    pub pm_10_0: Option<MicrogramsPerCubicMeter>,
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// Formaldehyde concentration
    pub hcho: Option<Ppb>,
    /// CO2 concentration
    pub co2: Option<Ppm>,
}

#[derive(SenRead, Debug, Clone)]
pub struct RawValuesSen62_3c {
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
}

#[derive(SenRead, Debug, Clone)]
pub struct RawValuesSen65_8_9c {
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
}

#[derive(SenRead, Debug, Clone)]
pub struct RawValuesSen66 {
    /// Ambient Humidity
    pub ambient_humidity: Option<Percent>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// CO2 concentration
    pub co2: Option<Ppm>,
}

#[derive(SenRead, Debug, Clone)]
pub struct NumberConcentrationValues {
    pub pm_0_5: Option<ParticlesPerCm3>,
    pub pm_1_0: Option<ParticlesPerCm3>,
    pub pm_2_5: Option<ParticlesPerCm3>,
    pub pm_4_0: Option<ParticlesPerCm3>,
    pub pm_10: Option<ParticlesPerCm3>,
}

#[derive(SenRead, Debug, Clone)]
pub struct TemperatureOffsetParameters {
    pub offset: DegCelsius,
    pub slope: i16,
    pub time_constant: u16,
    pub slot: u16,
}

#[derive(SenRead, Debug, Clone)]
pub struct TemperatureAccelerationParameters {
    pub k: u16,
    pub p: u16,
    pub t1: u16,
    pub t2: u16,
}

layout! {
    pub struct DeviceStatus(u32);
    {
        let __ @ 31..22;
        let speed_warning @ 21;
        let __ @ 20..13;
        let co2_1_error @ 12;
        let pm_error @ 11;
        let hcho_error @ 10;
        let co2_2_error @ 9;
        let __ @ 8;
        let gas_error @ 7;
        let rh_t_error @ 6;
        let __ @ 5;
        let fan_error @ 4;
        let __ @ 3..0;
    }
}

impl ValueWrapper for DeviceStatus {
    type Inner = u32;
    fn wrap(value: u32) -> Self {
        DeviceStatus::from(value)
    }
    fn unwrap(&self) -> Self::Inner {
        self.0
    }
}

#[derive(SenRead, Debug, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(SenRead, Debug, Clone)]
pub struct ShtHeaterMeasurements {
    pub sht_relative_humidity: Option<Percent>,
    pub sht_temperature: Option<DegCelsius>,
}

#[derive(SenRead, Debug, Clone)]
pub struct VocAlgorithmTuningParameters {
    pub index_offset: i16,
    pub learning_time_offset_hours: i16,
    pub learning_time_gain_hours: i16,
    pub gating_max_duration_minutes: i16,
    pub std_initial: i16,
    pub gain_factor: i16,
}

#[derive(SenRead, Debug, Clone)]
pub struct VocAlgorithmState {
    pub state: [u8; 8],
}

impl FromBytes<12, [u8; 8]> for [u8; 8] {
    fn from_bytes_with_crc<E>(bytes: &[u8; 12]) -> Result<[u8; 8], Error<E>> {
        io::check_crc::<8, E>(bytes)
    }
}

#[derive(SenRead, Debug, Clone)]
pub struct NoxAlgorithmTuningParameters {
    pub index_offset: i16,
    pub learning_time_offset_hours: i16,
    pub learning_time_gain_hours: i16,
    pub gating_max_duration_minutes: i16,
    pub std_initial: i16,
    pub gain_factor: i16,
}

#[derive(SenRead, Debug, Clone)]
pub struct Co2Correction {
    pub result: Option<u16>,
}

impl Co2Correction {
    pub fn value(&self) -> Option<Ppm> {
        self.result
            .map(|v| <Ppm as ValueWrapper>::wrap(((v as i32) - 0x8000i32) as i16))
    }
}
