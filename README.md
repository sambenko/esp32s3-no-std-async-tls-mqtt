<a name="readme-top"></a>

<h3 align="center">esp32s3 no_std Async TLS MQTT</h3>

  <p align="center">
    <br />
    Sending MQTT messages containing sensor data from bme680 to any MQTT broker that requires MQTT over TLS (AWS IoT Core) in no_std Rust!
    <br />
    <br />
    Based on: <a href="https://github.com/JurajSadel/esp32c3-no-std-async-mqtt-demo"><strong>https://github.com/JurajSadel/esp32c3-no-std-async-mqtt-demo</strong></a>
    <br />
</div>

## About The Project

This program allows user to get measured data from bme680 sensor and send them in MQTT messages via a TLS session to a MQTT Broker (in this case AWS IoT Core). All of this happening in Rust without the use of standard library.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With Crates

* <a href="https://github.com/esp-rs/esp-hal"><strong>esp-hal</strong></a>
* <a href="https://github.com/esp-rs/esp-wifi"><strong>esp-wifi</strong></a>
* <a href="https://github.com/embassy-rs/embassy"><strong>embassy</strong></a> for async
* <a href="https://github.com/esp-rs/esp-mbedtls"><strong>esp-mbedtls</strong></a> for TLS
* <a href="https://github.com/obabec/rust-mqtt"><strong>rust-mqtt</strong></a> for MQTT
* <a href="https://github.com/marcelbuesing/bme680"><strong>bme-680</strong></a> for data retrieval

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Prerequisites

<h4>COMING SOON</h4>

## Getting Started

<h4>COMING SOON</h4>
