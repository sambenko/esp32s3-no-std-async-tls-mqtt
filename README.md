# 📡 esp32s3 no_std Async TLS MQTT 

Sending MQTT messages containing sensor data from BME680 to any MQTT broker that requires MQTT over TLS (AWS IoT Core) in no_std Rust! 🦀

📚 Based on: [esp32c3-no-std-async-mqtt-demo](https://github.com/JurajSadel/esp32c3-no-std-async-mqtt-demo)

---

## 📋 Table of Contents

- [🎯 About The Project](#-about-the-project)
- [📦 Built With Crates](#-built-with-crates)
- [🔧 Prerequisites](#-prerequisites)
  - [Software Requirements](#software-requirements)
  - [Hardware Requirements](#hardware-requirements)
- [🚀 Getting Started](#-getting-started)
  - [1. 🔌 Hardware Setup](#1--hardware-setup)
  - [2. 🛡️ Setting up MQTT Configuration and Certificates](#setting-up-mqtt)
    - [MQTT Endpoint and Client ID](#mqtt-endpoint-and-client-id)
    - [MQTT over TLS](#mqtt-over-tls)
  - [3. 📶 WiFi Setup and Program Execution](#3--wifi-setup-and-program-execution)

---

## 🎯 About The Project

This program allows the user to get measured data from a BME680 sensor 🌡 and send them in MQTT messages 📬 via a TLS session to an MQTT Broker (in this case AWS IoT Core). All of this is done in Rust 🦀 without the use of the standard library. The program uses the [ESP32S3-BOX](https://github.com/espressif/esp-box/blob/master/docs/hardware_overview/esp32_s3_box/hardware_overview_for_box.md) Devkit.

[🔝 back to top](#-table-of-contents)

---

## 📦 Built With Crates

- [esp-hal](https://github.com/esp-rs/esp-hal) 🎛️ for peripheral access to the chip
- [esp-wifi](https://github.com/esp-rs/esp-wifi) 📶 for wifi connection
- [embassy](https://github.com/embassy-rs/embassy) 🔄 for async
- [esp-mbedtls](https://github.com/esp-rs/esp-mbedtls) 🔒 for TLS
- [rust-mqtt](https://github.com/obabec/rust-mqtt) 📬 for MQTT
- [bme-680](https://github.com/marcelbuesing/bme680) 🌡 for data retrieval

[🔝 back to top](#-table-of-contents)

---

## 🔧 Prerequisites

### Software Requirements

To run this project, you'll need to install the following:

- [Rust](https://rustup.rs) 🦀
- Install [espup](https://github.com/esp-rs/espup) tool and follow the instructions there 🛠

### Hardware Requirements

- Any [ESP32S3-BOX](https://github.com/espressif/esp-box/tree/master) devkit 🛠
- [BME680](https://www.bosch-sensortec.com/products/environmental-sensors/gas-sensors/bme680) environmental sensor 🌡

[🔝 back to top](#-table-of-contents)

---

## 🚀 Getting Started

### 1. 🔌 Hardware Setup

Before running the program, make sure your hardware is properly set up.

- **Connect the BME680 Sensor to ESP32S3-BOX device:**
   - `SDA` to `G41` on the device
   - `SCL` to `G40` on the device
   - `2-5V` to `3v3` on the device
   - `GND` to `GND` on the device
   > All 4 wires should be next to each other in the end.
   
- **Connect the ESP32S3-BOX to your computer**:
   - Use a USB-C cable to establish the connection.

<br>
<br>

<a name="setting-up-mqtt"></a>
### 2. 🛡️ Setting up MQTT Configuration and Certificates

#### 2.1 MQTT Endpoint and Client ID

- **Endpoint Address**: Write your MQTT broker's endpoint address into `endpoint.txt` in the `secrets/` folder.
- **Client ID**: If you are using a private broker like AWS MQTT, a client ID is also required. Paste the client ID into `client_id.txt` in the `secrets/` folder.

#### 2.2 MQTT over TLS

If your MQTT broker requires secure connections, you'll need to provide the necessary TLS X.509 certificates:

- **Trusted Root Certificate**: This is generally needed to verify the broker's identity. Paste the content of your root certificate into a file named `rootCA.pem` in the `secrets/` folder.
- **Client Certificate**: This is your device's certificate, which is presented to the broker during the handshake. Save it as `client_cert.pem.crt` in the `secrets/` folder.
- **Private Key**: This key matches the client certificate and must be kept private. Save it as `client_private.pem.key` in the `secrets/` folder.

The certificate and key files are read in the code snippet below after crate imports in `src/main.rs`:

```rust
const CERT: &'static str = concat!(include_str!("../secrets/rootCA.pem"), "\0");
const CLIENT_CERT: &'static str = concat!(include_str!("../secrets/client_cert.pem.crt"), "\0");
const PRIVATE_KEY: &'static str = concat!(include_str!("../secrets/client_private.pem.key"), "\0");
```

You can rename the files however you like, but you need to change the paths in consts accordingly.
> 🚨 Warning: Never commit your secrets and certificates to GitHub or any other public repository. The .gitignore file is set up to ignore these files, but you must also ensure not to manually include them in commits.

<br>
<br>

### 3. 📶 WiFi Setup and Program Execution

To set up your WiFi credentials and execute the program, you have two options:

#### Option 1: Use the Script 📜

1. **Edit the Script**: Open `run_with_wifi_credentials.sh` and enter your WiFi SSID and PASSWORD.
2. **Run the Script**: Execute the script to set the environment variables and run the program.
    ```bash
    ./run_with_wifi_credentials.sh
    ```

#### Option 2: Manual Setup 🤖

1. **Export Environment Variables**: Manually set the environment variables for your WiFi credentials.
    ```bash
    export SSID=your_wifi_ssid
    export PASSWORD=your_wifi_password
    ```
2. **Run the Program**: Use the following command to run the program.
    ```bash
    cargo run --release
    ```

Choose one of these options to set up your WiFi and execute the program.

[🔝 back to top](#-table-of-contents)
