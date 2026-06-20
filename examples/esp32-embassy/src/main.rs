#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::i2c;
use esp_hal::time::Rate;
use log::info;
use sen6x::Sen66CommandsAsync;

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_s: Spawner) {
    esp_println::logger::init_logger(log::LevelFilter::Info);
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);

    esp_rtos::start(
        timg0.timer0,
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT)
            .software_interrupt0,
    );
    let i2c_master: Mutex<CriticalSectionRawMutex, _> = Mutex::new(
        i2c::master::I2c::new(
            peripherals.I2C0,
            i2c::master::Config::default().with_frequency(Rate::from_khz(100)),
        )
        .unwrap()
        .with_scl(peripherals.GPIO5)
        .with_sda(peripherals.GPIO4)
        .into_async(),
    );
    let mut delay = embassy_time::Delay {};
    let mut sensor = sen6x::Sen6x::new(&i2c_master, &mut delay);

    loop {
        let serial = sensor.serial_number().await;

        match serial {
            Ok(serial) => {
                info!("Got SEN6x serial: {:?}", serial);
                break;
            }
            Err(e) => {
                info!("Error getting SEN6x serial: {:?}", e);
                Timer::after(Duration::from_millis(2_000)).await;
            }
        }
    }
    sensor.start_continuous_measurement().await.unwrap();

    loop {
        let measurement = sensor.measured_values().await.unwrap();
        info!("Got measurement: {:?}", measurement);
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
