use crate::io::{FromBytes, ValueWrapper};
use crate::{Error, io};
use bitrs::layout;
use fixed_str::FixedStr;
use sen6x_macros::SenRead;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use units::Rational;
use units::quantity::QuantityInfo;

pub(crate) type Milliseconds = u16;

/// A particulate-matter mass concentration, in micrograms per cubic metre (µg/m³).
type MicrogramsPerCubicMeter =
    <units::mass_density::MicrogramPerCubicMeter<u16> as QuantityInfo>::Scaled<
        { Rational::new(1, 10) },
    >;

/// A gas-index reading (VOC or NOx index points, nominal range 1–500, 100 ≈ typical).
///
/// Obtain the physical value with `f32::from` (or `.into()`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Index {
    value: i16,
}

/// Converts to index points (the raw register value divided by 10).
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

/// A particle number concentration, in particles per cubic centimetre (#/cm³).
///
/// Obtain the physical value with `f32::from` (or `.into()`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParticlesPerCm3 {
    value: u16,
}

/// Converts to particles per cm³ (the raw register value divided by 10).
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

/// Temperature in Sen6x format
pub type DegCelsius =
    <units::temperature::DegreesCelsius<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 200) }>;

/// The device's product name, as a fixed-capacity (32-byte) string.
pub type ProductName = FixedStr<32>;
/// The device's serial number, as a fixed-capacity (32-byte) string.
pub type SerialNumber = FixedStr<32>;

impl FromBytes<48, FixedStr<32>> for FixedStr<32> {
    fn from_bytes_with_crc<E>(bytes: &[u8; 48]) -> Result<FixedStr<32>, Error<E>> {
        io::check_crc::<32, E>(bytes).map(|v| FixedStr::<32>::from_slice(&v))
    }
}

/// Whether new measurement results are available to read.
#[derive(Debug, SenRead, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DataReady {
    /// `true` if new data is ready. `false` if not, or when no measurement is running.
    pub data_ready: bool,
}

/// Measured values returned by a SEN62.
///
/// A field is `None` when that value is unavailable (for example, when no
/// measurement has been running for at least one second).
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
}

/// Measured values returned by a SEN63C (adds CO₂ over the SEN62).
///
/// A field is `None` when that value is unavailable (for example, when no
/// measurement has been running for at least one second).
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// CO2 concentration
    pub co2: Option<units::parts_per::PerMillion<i16>>,
}
/// Measured values returned by a SEN65 (adds VOC and NOx indices over the SEN62).
///
/// A field is `None` when that value is unavailable (for example, when no
/// measurement has been running for at least one second).
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
}
/// Measured values returned by a SEN66 (PM, RH/T, VOC, NOx and CO₂).
///
/// A field is `None` when that value is unavailable (for example, when no
/// measurement has been running for at least one second).
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// CO2 concentration
    pub co2: Option<units::parts_per::PerMillion<i16>>,
}

/// Measured values returned by a SEN68 (adds formaldehyde over the SEN65).
///
/// A field is `None` when that value is unavailable (for example, when no
/// measurement has been running for at least one second).
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// Formaldehyde concentration
    pub hcho: Option<units::parts_per::PerBillion<i16>>,
}

/// Measured values returned by a SEN69C (PM, RH/T, VOC, NOx, formaldehyde and CO₂).
///
/// A field is `None` when that value is unavailable (for example, when no
/// measurement has been running for at least one second).
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// Formaldehyde concentration
    pub hcho: Option<units::parts_per::PerBillion<i16>>,
    /// CO2 concentration
    pub co2: Option<units::parts_per::PerMillion<i16>>,
}

/// Raw (uncompensated) values from a SEN62 or SEN63C.
///
/// A field is `None` when that value is unavailable.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawValuesSen62Sen63c {
    /// Ambient Humidity
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
}

/// Raw (uncompensated) values from a SEN65, SEN68 or SEN69C.
///
/// A field is `None` when that value is unavailable.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawValuesSen65Sen68Sen69c {
    /// Ambient Humidity
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
}

/// Raw (uncompensated) values from a SEN66.
///
/// A field is `None` when that value is unavailable.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawValuesSen66 {
    /// Ambient Humidity
    pub ambient_humidity:
        Option<<units::parts_per::Percent<i16> as QuantityInfo>::Scaled<{ Rational::new(1, 100) }>>,
    /// Ambient Temperature
    pub ambient_temperature: Option<DegCelsius>,
    /// VOC Index
    pub voc_index: Option<Index>,
    /// NOx Index
    pub nox_index: Option<Index>,
    /// CO2 concentration
    pub co2: Option<units::parts_per::PerMillion<i16>>,
}

/// Particle number concentrations, cumulative per size bin.
///
/// Each field is the number concentration of particles up to the given
/// aerodynamic diameter. A field is `None` when the value is unavailable.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NumberConcentrationValues {
    /// Number concentration of particles ≤ 0.5 µm.
    pub pm_0_5: Option<ParticlesPerCm3>,
    /// Number concentration of particles ≤ 1.0 µm.
    pub pm_1_0: Option<ParticlesPerCm3>,
    /// Number concentration of particles ≤ 2.5 µm.
    pub pm_2_5: Option<ParticlesPerCm3>,
    /// Number concentration of particles ≤ 4.0 µm.
    pub pm_4_0: Option<ParticlesPerCm3>,
    /// Number concentration of particles ≤ 10 µm.
    pub pm_10: Option<ParticlesPerCm3>,
}

/// Custom temperature-offset parameters used to compensate the ambient
/// temperature reading for the host design.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TemperatureOffsetParameters {
    /// Constant temperature offset to subtract.
    pub offset: DegCelsius,
    /// Normalized slope of the offset versus the measured temperature.
    pub slope: i16,
    /// Time constant of the offset filter, in seconds.
    pub time_constant: u16,
    /// Offset slot (0–4) being configured; the device blends all active slots.
    pub slot: u16,
}

/// Custom temperature-acceleration parameters of the RH/T engine, overriding
/// the device defaults. See the datasheet for the exact transfer function.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TemperatureAccelerationParameters {
    /// Filter constant `K`.
    pub k: u16,
    /// Filter constant `P`.
    pub p: u16,
    /// First time constant `T1`.
    pub t1: u16,
    /// Second time constant `T2`.
    pub t2: u16,
}

layout! {
    /// Device status register, decoded into individual flags.
    ///
    /// Each accessor returns a single status bit. Error flags are *sticky* — they
    /// stay set until cleared (see `read_and_clear_device_status` or a reset).
    /// The available flags are:
    ///
    /// - `speed_warning` — fan speed is outside the target range.
    /// - `co2_1_error`, `co2_2_error` — CO₂ sensor errors.
    /// - `pm_error` — particulate-matter sensor error.
    /// - `hcho_error` — formaldehyde sensor error.
    /// - `gas_error` — VOC/NOx gas sensor error.
    /// - `rh_t_error` — humidity/temperature sensor error.
    /// - `fan_error` — fan is mechanically blocked or broken.
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

/// Device firmware version.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Version {
    /// Major firmware version.
    pub major: u8,
    /// Minor firmware version.
    pub minor: u8,
}

/// Humidity and temperature measured by the SHT sensor at the end of a heater cycle.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShtHeaterMeasurements {
    /// Relative humidity reported by the SHT sensor, or `None` if unavailable.
    pub sht_relative_humidity: Option<units::parts_per::Percent<i16>>,
    /// Temperature reported by the SHT sensor, or `None` if unavailable.
    pub sht_temperature: Option<DegCelsius>,
}

/// Tuning parameters for the VOC gas-index algorithm.
///
/// See Sensirion's
/// [VOC Index for Indoor Air Applications](https://sensirion.com/media/documents/02232963/6294E043/Info_Note_VOC_Index.pdf)
/// for the meaning and valid ranges of each parameter.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VocAlgorithmTuningParameters {
    /// Index value the algorithm maps the average condition to (default 100).
    pub index_offset: i16,
    /// Time constant (hours) for the offset's adaptive learning.
    pub learning_time_offset_hours: i16,
    /// Time constant (hours) for the gain's adaptive learning.
    pub learning_time_gain_hours: i16,
    /// Maximum duration (minutes) that gating may stall learning.
    pub gating_max_duration_minutes: i16,
    /// Initial standard deviation used to estimate the gain.
    pub std_initial: i16,
    /// Gain factor applied to the normalized signal.
    pub gain_factor: i16,
}

/// Opaque backup of the VOC algorithm's internal state.
///
/// Read it to persist learning across a power cycle or reset, and write it back
/// before the next measurement to skip the initial learning phase.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VocAlgorithmState {
    /// The 8 raw state bytes, treated as an opaque blob.
    pub state: [u8; 8],
}

impl FromBytes<12, [u8; 8]> for [u8; 8] {
    fn from_bytes_with_crc<E>(bytes: &[u8; 12]) -> Result<[u8; 8], Error<E>> {
        io::check_crc::<8, E>(bytes)
    }
}

/// Tuning parameters for the NOx gas-index algorithm.
///
/// See Sensirion's
/// [NOx Index for Indoor Air Applications](https://sensirion.com/media/documents/9F289B95/6294DFFC/Info_Note_NOx_Index.pdf)
/// for the meaning and valid ranges of each parameter.
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NoxAlgorithmTuningParameters {
    /// Index value the algorithm maps the average condition to (default 1).
    pub index_offset: i16,
    /// Time constant (hours) for the offset's adaptive learning.
    pub learning_time_offset_hours: i16,
    /// Time constant (hours) for the gain's adaptive learning (unused for NOx).
    pub learning_time_gain_hours: i16,
    /// Maximum duration (minutes) that gating may stall learning.
    pub gating_max_duration_minutes: i16,
    /// Initial standard deviation used to estimate the gain.
    pub std_initial: i16,
    /// Gain factor applied to the normalized signal.
    pub gain_factor: i16,
}

/// Result of a forced CO₂ recalibration (FRC).
#[derive(SenRead, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Co2Correction {
    /// Raw FRC result word, or `None` if the recalibration failed (`0xFFFF`).
    ///
    /// Prefer [`Co2Correction::value`], which decodes this into a correction in ppm.
    pub result: Option<u16>,
}

impl Co2Correction {
    /// The applied CO₂ correction, in ppm, or `None` if the recalibration failed.
    ///
    /// The raw result is offset-encoded around `0x8000`; this subtracts the offset.
    ///
    /// ```
    /// use sen6x::types::Co2Correction;
    /// // 0x8000 encodes a zero correction.
    /// let c = Co2Correction { result: Some(0x8000) };
    /// assert_eq!(f32::from(c.value().unwrap().value()), 0.0);
    /// ```
    pub fn value(&self) -> Option<units::parts_per::PerMillion<i16>> {
        const OFFSET: i32 = 0x8000i32;
        self.result
            .map(|v| units::parts_per::PerMillion::new(v as i32 - OFFSET).convert())
    }
}
