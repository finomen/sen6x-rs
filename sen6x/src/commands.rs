use crate::io::ValueWrapper;
use crate::types::*;
use sen6x_macros::SenCmd;

#[derive(Debug, PartialEq, SenCmd)]
pub(crate) enum CommandId {
    #[cmd(execution_time = 50, Idle, new_state = Measurement)]
    StartContinuousMeasurement = 0x0021,
    #[cmd(execution_time = 1400, Measurement, new_state = Idle)]
    StopMeasurement = 0x0104,
    #[cmd(execution_time = 20, Measurement, rx = DataReady)]
    GetDataReady = 0x0202,
    #[cfg(feature = "sen62")]
    #[cmd(execution_time = 20, Measurement, rx = MeasuredValues)]
    ReadMeasuredValues = 0x04A3,
    #[cfg(feature = "sen63c")]
    #[cmd(execution_time = 20, Measurement, rx = MeasuredValues)]
    ReadMeasuredValues = 0x0471,
    #[cfg(feature = "sen65")]
    #[cmd(execution_time = 20, Measurement, rx = MeasuredValues)]
    ReadMeasuredValues = 0x0446,
    #[cfg(feature = "sen66")]
    #[cmd(execution_time = 20, Measurement, rx = MeasuredValues)]
    ReadMeasuredValues = 0x0300,
    #[cfg(feature = "sen68")]
    #[cmd(execution_time = 20, Measurement, rx = MeasuredValues)]
    ReadMeasuredValues = 0x0467,
    #[cfg(feature = "sen69c")]
    #[cmd(execution_time = 20, Measurement, rx = MeasuredValues)]
    ReadMeasuredValues = 0x04B5,
    #[cfg(any(feature = "sen62", feature = "sen63c"))]
    #[cmd(execution_time = 20, Measurement, rx = RawValues)]
    ReadRawValues = 0x0492,
    #[cfg(any(feature = "sen65", feature = "sen68", feature = "sen69c"))]
    #[cmd(execution_time = 20, Measurement, rx = RawValues)]
    ReadRawValues = 0x0455,
    #[cfg(feature = "sen66")]
    #[cmd(execution_time = 20, Measurement, rx = RawValues)]
    ReadRawValues = 0x0405,
    #[cmd(execution_time = 20, Measurement, rx = NumberConcentrationValues)]
    ReadNumberConcentrationValues = 0x0316,
    #[cmd(execution_time = 20, Idle, Measurement, tx = TemperatureOffsetParameters)]
    SetTemperatureOffsetParameters = 0x60B2,
    #[cmd(execution_time = 20, Idle, tx = TemperatureAccelerationParameters)]
    SetTemperatureAccelerationParameters = 0x6100,
    #[cmd(execution_time = 20, Idle, Measurement, rx = ProductName)]
    GetProductName = 0xD014,
    #[cmd(execution_time = 20, Idle, Measurement, rx = SerialNumber)]
    GetSerialNumber = 0xD033,
    #[cmd(execution_time = 20, Idle, Measurement, rx = DeviceStatus)]
    ReadDeviceStatus = 0xD206,
    #[cmd(execution_time = 20, Idle, Measurement, rx = DeviceStatus)]
    ReadAndClearDeviceStatus = 0xD210,
    #[cmd(execution_time = 20, Idle, Measurement, rx = Version)]
    GetVersion = 0xD100,
    #[cmd(execution_time = 1200, Idle)]
    DeviceReset = 0xD304,
    #[cmd(execution_time = 20, Idle)]
    StartFanCleaning = 0x5607,
    #[cmd(execution_time = 20, Idle)]
    ActivateSHTHeater = 0x6765,
    #[cmd(execution_time = 20, Idle, rx = SHTHeaterMeasurements)]
    GetSHTHeaterMeasurements = 0x6790,
    #[cfg(any(
        feature = "sen65",
        feature = "sen66",
        feature = "sen68",
        feature = "sen69c"
    ))]
    #[cmd(execution_time = 20, Idle, reg = VOCAlgorithmTuningParameters)]
    VOCAlgorithmTuningParameters = 0x60D0,
    #[cfg(any(
        feature = "sen65",
        feature = "sen66",
        feature = "sen68",
        feature = "sen69c"
    ))]
    #[cmd(execution_time = 20, Idle, Measurement, reg = VOCAlgorithmState)]
    // FIXME: set only in idle!
    VOCAlgorithmState = 0x6181,
    #[cfg(any(
        feature = "sen65",
        feature = "sen66",
        feature = "sen68",
        feature = "sen69c"
    ))]
    #[cmd(execution_time = 20, Idle, reg = NOxAlgorithmTuningParameters)]
    NOxAlgorithmTuningParameters = 0x60E1,
    #[cfg(any(feature = "sen63c", feature = "sen66", feature = "sen69c"))]
    #[cmd(execution_time = 500, Idle, rx = Co2Correction, tx = PpmU16)]
    PerformForcedCO2Recalibration = 0x6707,
    #[cfg(any(feature = "sen63c", feature = "sen66", feature = "sen69c"))]
    #[cmd(execution_time = 1400, Idle)]
    PerformCO2SensorFactoryReset = 0x6754,
    #[cfg(any(feature = "sen63c", feature = "sen66", feature = "sen69c"))]
    #[cmd(execution_time = 20, Idle, reg = bool)]
    CO2SensorAutomaticSelfCalibration = 0x6711,
    #[cfg(any(feature = "sen63c", feature = "sen66", feature = "sen69c"))]
    #[cmd(execution_time = 20, Idle, Measurement, reg = HPa)]
    AmbientPressure = 0x6720,
    #[cfg(any(feature = "sen63c", feature = "sen66", feature = "sen69c"))]
    #[cmd(execution_time = 20, Idle, reg = Meters)]
    SensorAltitude = 0x6736,
}

#[cfg(test)]
mod tests {
    use crate::errors::Error;
    use crate::io::{FromBytes, ToBytes};
    use crate::types::DataReady;
    use crate::types::SerialNumber;

    //TODO: generate only needed traits!
    #[test]
    fn parse_data_ready() {
        assert_eq!(
            DataReady::from_bytes_with_crc::<u32>(&[0x00, 0x02, 0x00]),
            Err(Error::Crc)
        );
        assert_eq!(
            DataReady::from_bytes_with_crc::<u32>(&[
                0x00,
                0x00,
                sensirion_i2c::crc8::calculate(&[0x00, 0x00])
            ]),
            Ok(DataReady { data_ready: false })
        );
        assert_eq!(
            DataReady::from_bytes_with_crc::<u32>(&[
                0x00,
                0x01,
                sensirion_i2c::crc8::calculate(&[0x00, 0x01])
            ]),
            Ok(DataReady { data_ready: true })
        );
        assert_eq!(
            DataReady::from_bytes_with_crc::<u32>(&[
                0x00,
                0x02,
                sensirion_i2c::crc8::calculate(&[0x00, 0x02])
            ]),
            Err(Error::InvalidValue)
        );
    }

    #[test]
    fn parse_serial() {
        let raw_serial: [u8; 32] = [
            0x34, 0x32, 0x34, 0x35, 0x39, 0x39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut raw_data = [0u8; 48];
        for i in 0..raw_serial.len() / 2 {
            raw_data[i * 3] = raw_serial[i * 2];
            raw_data[i * 3 + 1] = raw_serial[i * 2 + 1];
            raw_data[i * 3 + 2] = sensirion_i2c::crc8::calculate(&raw_data[i * 3..i * 3 + 2]);
        }

        assert_eq!(
            SerialNumber::from_bytes_with_crc::<u32>(&raw_data),
            Ok(SerialNumber::new("424599"))
        );
    }

    #[test]
    fn emit_data_ready() {
        assert_eq!(
            DataReady { data_ready: false }.to_bytes(),
            [0x00, 0x00, sensirion_i2c::crc8::calculate(&[0x00, 0x00])]
        );
        assert_eq!(
            DataReady { data_ready: true }.to_bytes(),
            [0x00, 0x01, sensirion_i2c::crc8::calculate(&[0x00, 0x01])]
        );
    }
}
