#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use esp_backtrace as _;
use esp_println::println;
use log::info;

// peripherals imports
use hal::{
    clock::{ClockControl, CpuClock},
    i2c::I2C,
    peripherals::{Interrupt, Peripherals, I2C0},
    prelude::{_fugit_RateExtU32, *},
    systimer::SystemTimer,
    timer::TimerGroup,
    Rng, 
    Rtc, 
    IO, 
    {embassy, interrupt},
};

//display imports 
use embedded_graphics::{
    pixelcolor::Rgb565, prelude::*,
};
use display_interface_spi::SPIInterfaceNoCS;
use mipidsi::{ColorOrder, Orientation};

//wifi imports
use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_wifi::wifi::{WifiController, WifiDevice, WifiEvent, WifiMode, WifiState};
use esp_wifi::{initialize, EspWifiInitFor};

// embassy imports
use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Stack, StackResources, dns::DnsQueryType};
use embassy_time::{Duration, Timer};

// mqtt imports
use rust_mqtt::{
    client::{client::MqttClient, client_config::{ClientConfig}},
    utils::rng_generator::CountingRng,
};

// bmp180 imports
use crate::bmp180_async::Bmp180;
mod bmp180_async;

use heapless::String;
use core::fmt::Write;

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
const ENDPOINT: &'static str = include_str!("../secrets/endpoint.txt");
const CLIENT_ID: &'static str = include_str!("../secrets/client_id.txt");

macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        let (x,) = STATIC_CELL.init(($val,));
        x
    }};
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);

    
    rtc.swd.disable();

    rtc.rwdt.disable();

    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);

    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let timer0 = timer_group0.timer0;

    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    
    rtc.swd.disable();
    rtc.rwdt.disable();
    
    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");

    loop {}
}
