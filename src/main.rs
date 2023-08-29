#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use esp_backtrace as _;
use esp_println::println;
use log::info;

use hal::{
    clock::{ClockControl, CpuClock},
    i2c::I2C,
    peripherals::{Interrupt, Peripherals, I2C0},
    prelude::{_fugit_RateExtU32, *},
    Rng,
    Rtc,
    systimer::SystemTimer,
    timer::TimerGroup,
    IO,
    embassy,
    interrupt,
    spi,
    delay::Delay,
};

use embedded_graphics::{
    pixelcolor::Rgb565, prelude::*,
};
use display_interface_spi::SPIInterfaceNoCS;
use mipidsi::{ColorOrder, Orientation};

use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_wifi::wifi::{WifiController, WifiDevice, WifiEvent, WifiMode, WifiState};
use esp_wifi::{initialize, EspWifiInitFor};

use embassy_executor::Executor;
use embassy_time::{Duration, Timer};
use embassy_executor::_export::StaticCell;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Stack, StackResources, dns::DnsQueryType};

use rust_mqtt::{
    client::{client::MqttClient, client_config::{ClientConfig}},
    utils::rng_generator::CountingRng,
};

use crate::bmp180_async::Bmp180;
mod bmp180_async;

use heapless::String;
use core::fmt::Write;



#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    // setup logger
    // To change the log_level change the env section in .config/cargo.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");
    println!("Hello world!");

    loop {}
}
