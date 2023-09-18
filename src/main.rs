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
    Delay,
    {embassy, interrupt},
    spi,
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

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;
    let mut backlight = io.pins.gpio45.into_push_pull_output();
    backlight.set_high().expect("Failed to set backlight high");

    let spi = spi::Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        60u32.MHz(),
        spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );
    
    let di = SPIInterfaceNoCS::new(spi, io.pins.gpio4.into_push_pull_output());
    let reset = io.pins.gpio48.into_push_pull_output();
    let mut delay = Delay::new(&clocks);

    let mut display = mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(320, 240)
        .with_orientation(Orientation::PortraitInverted(false))
        .with_color_order(ColorOrder::Bgr)
        .init(&mut delay, Some(reset))
        .expect("Display failed to initialize");

    display.clear(Rgb565::WHITE).expect("Failed to clear display");

    let init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .expect("Failed to initialize Wifi");

    embassy::init(
        &clocks,
        timer0,
    );

    let (wifi, _) = peripherals.RADIO.split();
    let (wifi_interface, controller) =
        match esp_wifi::wifi::new_with_mode(&init, wifi, WifiMode::Sta) {
            Ok((wifi_interface, controller)) => (wifi_interface, controller),
            Err(..) => panic!("WiFi mode Error!"),
        };

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio41,
        io.pins.gpio40,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let config = Config::dhcpv4(Default::default());

    let seed = 69420;

    let stack = &*singleton!(Stack::new(
        wifi_interface,
        config,
        singleton!(StackResources::<3>::new()),
        seed
    ));

    interrupt::enable(Interrupt::I2C_EXT0, interrupt::Priority::Priority1)
        .expect("Invalid Interrupt Priority Error");

    let executor = EXECUTOR.init(Executor::new());
    

    loop {}
}
