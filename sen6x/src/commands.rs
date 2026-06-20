use crate::types::*;
use sen6x_macros::SenCmd;

#[derive(Debug, PartialEq, SenCmd)]
pub(crate) enum CommandId {
    /// Starts a continuous measurement. After starting the measurement, it takes some time (~1.1s) until
    /// the first measurement results are available. You could poll with the command Get Data Ready to check when
    /// the results are ready to be read.
    ///
    /// *Note:* For SEN63C and SEN69C only: SEN63C and SEN69C are conditioning the CO2 sensor during the initial
    /// 24 seconds after starting a measurement. As this process cannot be interrupted, the following limitations apply
    /// during this period:
    /// - You may stop the measurement if needed, but do not start it again until at least 24 seconds have
    ///   passed to avoid a CO2-1 – CO2 Sensor Error.
    /// - Do not stop the sensor and use the commands Perform Forced CO2 Recalibration, Set CO2 Sensor
    ///   Automatic Self Calibration or Perform CO2 Sensor Factory Reset.
    #[execution_time(50)]
    #[send(allowed_in = [Idle], new_state = Measurement)]
    StartContinuousMeasurement = 0x0021,
    ///  Stops the measurement and returns to idle mode.
    #[execution_time(1400)]
    #[send(allowed_in = [Measurement], new_state = Idle)]
    StopMeasurement = 0x0104,
    /// This command can be used to check if new measurement results are ready to read. The data_ready flag is automatically reset after reading the measurement values.
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = DataReady)]
    DataReady = 0x0202,
    ///  Returns the measured values. The command data_ready can be used to check if new data is available since the last read operation. If no new data is available, the previous values will be returned. If no
    /// data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = MeasuredValuesSen62)]
    #[target(Sen62)]
    #[alias(MeasuredValues)]
    MeasuredValuesSen62 = 0x04A3,
    ///  Returns the measured values. The command data_ready can be used to check if new data is available since the last read operation. If no new data is available, the previous values will be returned. If no
    /// data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = MeasuredValuesSen63c)]
    #[target(Sen63c)]
    #[alias(MeasuredValues)]
    MeasuredValuesSen63c = 0x0471,
    ///  Returns the measured values. The command data_ready can be used to check if new data is available since the last read operation. If no new data is available, the previous values will be returned. If no
    /// data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = MeasuredValuesSen65)]
    #[target(Sen65)]
    #[alias(MeasuredValues)]
    MeasuredValuesSen65 = 0x0446,
    ///  Returns the measured values. The command data_ready can be used to check if new data is available since the last read operation. If no new data is available, the previous values will be returned. If no
    /// data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = MeasuredValuesSen66)]
    #[target(Sen66)]
    #[alias(MeasuredValues)]
    MeasuredValuesSen66 = 0x0300,
    ///  Returns the measured values. The command data_ready can be used to check if new data is available since the last read operation. If no new data is available, the previous values will be returned. If no
    /// data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = MeasuredValuesSen68)]
    #[target(Sen68)]
    #[alias(MeasuredValues)]
    MeasuredValuesSen68 = 0x0467,
    ///  Returns the measured values. The command data_ready can be used to check if new data is available since the last read operation. If no new data is available, the previous values will be returned. If no
    /// data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = MeasuredValuesSen69c)]
    #[target(Sen69c)]
    #[alias(MeasuredValues)]
    MeasuredValuesSen69c = 0x04B5,
    /// Returns the measured raw values. The command data_ready can be used to check if new
    /// data is available since the last read operation. If no new data is available, the previous values will be returned.
    /// If no data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = RawValuesSen62_3c)]
    #[target(Sen62)]
    #[target(Sen63c)]
    #[alias(RawValues)]
    RawValuesSen62_3c = 0x0492,
    /// Returns the measured raw values. The command data_ready can be used to check if new
    /// data is available since the last read operation. If no new data is available, the previous values will be returned.
    /// If no data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = RawValuesSen65_8_9c)]
    #[target(Sen65)]
    #[target(Sen68)]
    #[target(Sen69c)]
    #[alias(RawValues)]
    RawValuesSen65_8_9c = 0x0455,
    /// Returns the measured raw values. The command data_ready can be used to check if new
    /// data is available since the last read operation. If no new data is available, the previous values will be returned.
    /// If no data is available at all (e.g. measurement not running for at least one second), all values will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = RawValuesSen66)]
    #[target(Sen66)]
    #[alias(RawValues)]
    RawValuesSen66 = 0x0405,
    /// Returns the measured number concentration values. The command Get Data Ready can be used
    /// to check if new data is available since the last read operation. If no new data is available, the previous values
    /// will be returned. If no data is available at all (e.g. measurement not running for at least one second), all values
    /// will be None
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = NumberConcentrationValues)]
    NumberConcentrationValues = 0x0316,
    /// This command allows to compensate temperature effects of the design-in at customer side by
    /// applying custom temperature offsets to the ambient temperature.
    #[execution_time(20)]
    #[write(allowed_in = [Idle, Measurement], tx = TemperatureOffsetParameters)]
    TemperatureOffsetParameters = 0x60B2,
    /// This command allows to set custom temperature acceleration parameters of the RH/T engine. It
    /// overwrites the default temperature acceleration parameters of the RH/T engine with custom values.
    #[execution_time(20)]
    #[write(allowed_in = [Idle], tx = TemperatureAccelerationParameters)]
    TemperatureAccelerationParameters = 0x6100,
    /// Gets the product name from the device
    #[execution_time(20)]
    #[read(allowed_in = [Measurement], rx = ProductName)]
    ProductName = 0xD014,
    ///  Gets the serial number from the device.
    #[execution_time(20)]
    #[read(allowed_in = [Idle, Measurement], rx = SerialNumber)]
    SerialNumber = 0xD033,

    /// Reads the current device status.
    ///
    /// Use this command to get detailed information about the device status. The device status is encoded in flags.
    /// Each device status flag represents a single bit in a 32-bit integer value. If more than one error is present, the
    /// device status register value is the sum of the corresponding flag values. For details about the available flags,
    /// refer to the Device Status Register documentation.
    ///
    /// *Note*: The status flags of type "Error" are sticky, i.e. they are not cleared automatically even if the error condition
    /// no longer exists. So, they can only be cleared manually with read_and_clear_device_status or through a reset,
    /// either by calling Device Reset or through a power cycle. All other flags are not sticky, i.e. they are cleared
    /// automatically if the trigger condition disappears
    #[execution_time(20)]
    #[read(allowed_in = [Idle, Measurement], rx = DeviceStatus)]
    DeviceStatus = 0xD206,
    /// Reads the current device status (like command Read device_status) and afterwards clears all flags
    #[execution_time(20)]
    #[read(allowed_in = [Idle, Measurement], rx = DeviceStatus)]
    ReadAndClearDeviceStatus = 0xD210,
    ///  Gets the version information for the firmware
    #[execution_time(20)]
    #[read(allowed_in = [Idle, Measurement], rx = Version)]
    Version = 0xD100,
    /// Executes a reset on the device. This has the same effect as a power cycle
    #[execution_time(1200)]
    #[send(allowed_in = [Idle])]
    DeviceReset = 0xD304,
    ///  This command triggers fan cleaning. The fan is set to the maximum speed for 10 seconds and
    /// then automatically stopped. Wait at least 10s after this command before starting a measurement.
    #[execution_time(20)]
    #[send(allowed_in = [Idle])]
    StartFanCleaning = 0x5607,
    /// This command allows you to use the inbuilt heater in SHT sensor to reverse creep at high humidity.
    /// This command activates the SHT sensor heater with 200mW for 1s. The heater is then automatically deactivated
    /// again. For firmware versions with an Execution Time of 20ms in the table below, the Get SHT Heater
    /// Measurements command can be polled to check whether the heating is finished to trigger another cycle to
    /// maximize the duty cycle. Older firmware version do not yet support Get SHT Heater Measurements.
    /// Wait at least 20s after this command before starting a measurement to get coherent temperature values
    /// (heating consequence to disappear).
    #[execution_time(20)] // FIXME: execution time should depend on Version, this is value for new sensors
    #[send(allowed_in = [Idle])]
    ActivateShtHeater = 0x6765,
    /// Get the measurement values when the SHT sensor heating is finished.
    /// *Note*: This command is only available from the Firmware Version specified in the table below. It must be used
    /// after the Activate SHT Heater command. The command can be queried every 50ms to check if the heating
    /// cycle is finished and measurements are available.
    #[execution_time(20)] // FIXME: availability depend on Version
    #[read(allowed_in = [Idle], rx = ShtHeaterMeasurements)]
    ShtHeaterMeasurements = 0x6790,

    /// Gets the parameters to customize the VOC algorithm. For more information on what the
    /// parameters below do, refer to Sensirion’s VOC Index for Indoor Air Applications [4].
    #[read(allowed_in = [Idle], rx = VocAlgorithmTuningParameters)]
    ///  Sets the parameters to customize the VOC algorithm. It has no effect if at least one parameter is
    /// outside the specified range. For more information on what the parameters below do, refer to Sensirion’s VOC
    /// Index for Indoor Air Applications [4].
    #[write(allowed_in = [Idle], tx = VocAlgorithmTuningParameters)]
    #[execution_time(20)]
    #[target(Sen65)]
    #[target(Sen66)]
    #[target(Sen68)]
    #[target(Sen69c)]
    VocAlgorithmTuningParameters = 0x60D0,

    /// Allows backup of the VOC algorithm state to resume operation after a power cycle or device reset,
    /// skipping initial learning phase. By default, the VOC Engine is reset, and the algorithm state is retained if a
    /// measurement is stopped and started again. If the VOC algorithm state shall be reset, a device reset, or a power
    /// cycle can be executed.
    #[read(allowed_in = [Idle, Measurement], rx = VocAlgorithmState)]
    /// Allows restoration of the VOC algorithm state to resume operation after a power cycle or device
    /// reset, skipping initial learning phase. By default, the VOC Engine is reset, and the algorithm state is retained if
    /// a measurement is stopped and started again. If the VOC algorithm state shall be reset, a device reset, or a
    /// power cycle can be executed.
    /// Sets the VOC algorithm state previously received with the Get VOC Algorithm State command. This command
    /// is only available in idle mode and the state will be applied only once when starting the next measurement. In
    /// measurement mode, this command has no effect.
    #[write(allowed_in = [Idle], tx = VocAlgorithmState)]
    #[execution_time(20)]
    #[target(Sen65)]
    #[target(Sen66)]
    #[target(Sen68)]
    #[target(Sen69c)]
    VocAlgorithmState = 0x6181,

    /// Gets the parameters to customize the NOx algorithm. For more information on what the
    /// parameters below do, refer to Sensirion’s NOx Index for Indoor Air Applications [5].
    #[read(allowed_in = [Idle], rx = NoxAlgorithmTuningParameters)]
    /// Sets the parameters to customize the NOx algorithm. It has no effect if at least one parameter is
    /// outside the specified range. To check whether the parameters have been set successfully, use the Get NOx
    /// Algorithm Tuning Parameters command. For more information on what the parameters below do, refer to
    /// Sensirion’s NOx Index for Indoor Air Applications [5].
    #[write(allowed_in = [Idle], tx = NoxAlgorithmTuningParameters)]
    #[execution_time(20)]
    #[target(Sen65)]
    #[target(Sen66)]
    #[target(Sen68)]
    #[target(Sen69c)]
    NoxAlgorithmTuningParameters = 0x60E1,

    //TODO: implement calibration isntead of exposing raw method
    /// Execute the forced recalibration (FRC) of the CO2 signal. To successfully conduct an accurate FRC,
    /// the following steps need to be taken:
    /// 1. Start a measurement with the command Start Continuous Measurement and operate the sensor for
    /// at least 3 minutes in an environment with homogenous and constant CO2 concentration. If applicable,
    /// the reference value for altitude or pressure compensation must be provided to the sensor beforehand
    /// with the command Set Sensor Altitude or Set Ambient Pressure respectively.
    /// 2. Stop the measurement with the command Stop Measurement and wait at least 1400ms.
    /// 3. Issue the Perform Forced CO2 Recalibration command with the reference CO2 concentration that the
    /// sensor should be set to. The recalibration procedure will take about 500 ms to complete, during which
    /// time no other functions can be executed. A return value of 0xFFFF indicates that the FRC has failed
    #[execution_time(500)]
    #[fetch(allowed_in = [Idle], rx = Co2Correction, tx = PpmU16 )]
    #[target(Sen63c)]
    #[target(Sen66)]
    #[target(Sen69c)]
    PerformForcedCo2Recalibration = 0x6707,

    // FIXME: availability depend on Version
    /// This command resets all CO2 sensor configuration settings stored in the EEPROM and erases the
    /// forced recalibration (FRC) and automatic self-calibration (ASC) algorithm history of the CO2 sensor, restarting
    /// the bypass phase. Refer to the datasheet of the STCC4 for more information [6].
    #[execution_time(1400)]
    #[send(allowed_in = [Idle])]
    #[target(Sen63c)]
    #[target(Sen66)]
    #[target(Sen69c)]
    PerformCo2SensorFactoryReset = 0x6754,

    /// Gets the status of the CO2 sensor automatic self-calibration (ASC). The CO2 sensor supports
    /// automatic self-calibration (ASC) for long-term stability of the CO2 output. This feature can be enabled or
    /// disabled. By default, it is enabled.
    #[read(allowed_in = [Idle], rx = bool)]
    /// Sets the status of the CO2 sensor automatic self-calibration (ASC). The CO2 sensor supports
    /// automatic self-calibration (ASC) for long-term stability of the CO2 output. This feature can be enabled or
    /// disabled. By default, it is enabled.
    /// The automatic self-calibration can be disabled for testing under lab conditions where concentrations below
    /// 400ppm are expected, to avoid an alteration of the baseline. In the field, ASC must be enabled and exposure
    /// to fresh air (i.e. CO2 concentration at 400 ppm) at least once per week is required to reach datasheet
    /// specifications
    #[write(allowed_in = [Idle], tx = bool)]
    #[execution_time(20)]
    #[target(Sen63c)]
    #[target(Sen66)]
    #[target(Sen69c)]
    Co2SensorAutomaticSelfCalibration = 0x6711,

    /// Gets the ambient pressure value that was set with Set Ambient Pressure. The ambient pressure
    /// can be used for pressure compensation in the CO2 sensor
    #[read(allowed_in = [Idle, Measurement], rx = Hpa)]
    /// Sets the ambient pressure value. The ambient pressure can be used for pressure compensation in
    /// the CO2 sensor. Setting an ambient pressure overrides any pressure compensation based on a previously set
    /// sensor altitude. Use of this command is recommended for applications experiencing significant ambient
    /// pressure changes to ensure CO2 sensor accuracy. Valid input values are between 700 to 1’200 hPa. The default
    /// value is 1013 hPa.
    #[write(allowed_in = [Idle, Measurement], tx = Hpa)]
    #[execution_time(20)]
    #[target(Sen63c)]
    #[target(Sen66)]
    #[target(Sen69c)]
    AmbientPressure = 0x6720,

    /// Gets the current sensor altitude. The sensor altitude can be used for pressure compensation in
    /// the CO2 sensor.
    #[read(allowed_in = [Idle], rx = Meters)]
    /// Sets the current sensor altitude. The sensor altitude can be used for pressure compensation in the
    /// CO2 sensor. The default sensor altitude value is set to 0 meters above sea level. Valid input values are between
    /// 0 and 3000m.
    #[write(allowed_in = [Idle], tx = Meters)]
    #[execution_time(20)]
    #[target(Sen63c)]
    #[target(Sen66)]
    #[target(Sen69c)]
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
