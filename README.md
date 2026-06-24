# Sensirion I2C SEN6x Driver

This library provides an embedded `no_std` driver for the [Sensirion SEN6x series](https://developer.sensirion.com/product-support/sen6x-environmental-sensor-node).
This driver was built using [embedded-hal](https://docs.rs/embedded-hal/) traits.

This driver is compatible with `embedded-hal v1.0`.

[![Crates.io](https://img.shields.io/crates/v/sen6x-driver.svg)](https://crates.io/crates/sen6x-driver)
[![Docs.rs](https://docs.rs/sen6x-driver/badge.svg)](https://docs.rs/sen6x-driver)
[![codecov](https://codecov.io/gh/finomen/sen6x-rs/graph/badge.svg?token=G6HXEWSEEP)](https://codecov.io/gh/finomen/sen6x-rs)

## Reasons for yet another driver

There are several other implementation, but they lack one or more features I needed:
- support for different sensors from Sen6x family
- support for shared I2C bus in asynchronous implementation
- support for calibration/compensation commands

## Features

- **`embedded-hal`** - Enables async I2C support via `embedded-hal`.
- **`embedded-hal-async`** - Enables async I2C support via `embedded-hal-async`.
- **`embassy`** - Enables shared I2C bus support using `embassy::embassy_sync::mutex::Mutex`. This option enables `embedded-hal-async`

```toml
[dependencies]
sen6x-driver = { version = "0.0.3", features = ["embassy"] }
```

## Sensirion SEN6x

The SEN6x sensor module family is an air quality platform that combines critical parameters such as particulate
matter, relative humidity, temperature, VOC, NOx and either CO2 or formaldehyde, all in one compact package.

![sen6x](https://github.com/finomen/sen6x-rs/raw/master/img/sen6x.png)

Further information: [Datasheet SEN6x](https://sensirion.com/media/documents/FAFC548D/693FBB15/PS_DS_SEN6x.pdf)

## GenAI Usage

This project was created with the assistance of Claude and Gemini. Primarily, GenAI is used to:
- Generate documentation
- Check code against the datasheet
- Fix warnings and clean up code

## License

Licensed under [MIT license](https://github.com/finomen/sen6x-rs/blob/master/LICENSE)

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed under MIT License, without any additional terms or conditions.