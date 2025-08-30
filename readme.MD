[![Publish Docker release image](https://github.com/thomsva/ruuvi-rust-mqtt/actions/workflows/docker-release.yml/badge.svg)](https://github.com/thomsva/ruuvi-rust-mqtt/actions/workflows/docker-release.yml)

# Ruuvi Rust MQTT

Simple program to gather data from RuuviTag sensors and publish to MQTT broker.
Easy integration with Home Assistant.

> ⚠️ The program seems to work fine but has not been thoroughly tested.

## Table of Contents
- [Features](#features)
- [Requirements](#requirements)
- [Configuration](#configuration)
  - [MQTT Configuration](#mqtt-configuration)
  - [Sensor Filtering / Logging](#sensor-filtering--logging)
  - [Publish Options](#publish-options)
  - [Example configuration](#example-configuration)
- [Quick start guide using docker compose](#quick-start-guide-using-docker-compose)
- [Troubleshooting](#troubleshooting)

## Features

- Decodes raw data from RuuviTag sensors into temperature, pressure and
  humidity.
- Works with Docker Compose.
- If there are unwanted sensors nearby, they can be ignored using the blacklist
  and/or whitelist feature.
- Publish discovery messages for Home Assistant.

## Requirements

- Linux computer with Docker installed.
- Access to the default Bluetooth interface (BlueZ).
- Bluetooth adapter compatible with Bluetooth Low Energy (BLE).
- RuuviTag sensors using
  [Data format 5 (RAWv2)](https://docs.ruuvi.com/communication/bluetooth-advertisements/data-format-5-rawv2).
- A valid config.toml file.

## Configuration

This project uses a `config.toml` file for settings. Below are explanations for each section.

### MQTT Configuration

Configure the MQTT broker connection.

| Key        | Type    | Default   | Description |
|------------|---------|-----------|-------------|
| `host`     | string  | `"mqtt"`  | Hostname or IP address of your MQTT broker |
| `port`     | integer | `1883`    | Port on which the MQTT broker listens |
| `username` | string  | `""`      | Optional MQTT username. Leave empty if not required |
| `password` | string  | `""`      | Optional MQTT password. Leave empty if not required |


### Sensor Filtering / Logging

This section allows you to control which sensors are included or excluded in your setup and whether sensor data is printed for debugging. It can useful for managing large numbers of sensors or for troubleshooting specific devices.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `blacklist` | list of strings | `[]` | List of sensor MAC addresses to **exclude**. Example: `["F1:CC:CA:55:44:33", "F1:CC:CA:55:22:11"]`. Sensors in this list will be ignored if `use_blacklist` is `true`. |
| `use_blacklist` | boolean | `false` | If `true`, any sensor in the blacklist will always be excluded. If `false`, the blacklist is ignored. |
| `whitelist` | list of strings | `[]` | List of sensor MAC addresses to **include**. Example: `["F1:CC:CA:55:44:33", "F1:CC:CA:55:22:11"]`. Only sensors in this list will be accepted if `use_whitelist` is `true`. |
| `use_whitelist` | boolean | `false` | If `true`, only sensors in the whitelist are allowed. If `false`, all detected sensors are accepted. |
| `debug_print` | boolean | `false` | If `true`, sensor measurements will be printed to the console for debugging. Recommended to disable in production deployments. |

### Publish Options

This section controls what data is published via MQTT. 

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `discovery` | boolean | `true` | Enable Home Assistant MQTT discovery for newly detected sensors. Automatically creates devices in Home Assistant. |
| `decoded_data` | boolean | `true` | Publish decoded sensor measurements such as temperature, humidity, and pressure. This is the primary data most users need. |
| `raw_data` | boolean | `false` | Publish raw sensor data in hexadecimal format. Typically used for debugging or analysis; not necessary for standard operation. |

### Example configuration

```toml
# File: config.toml
[mqtt]
host = "mqtt"
port = 1883
username = ""
password = ""

[sensors]
blacklist = []
use_blacklist = false
whitelist = []
use_whitelist = false
debug_print = false

[publish]
discovery = true
decoded_data = true
raw_data = false
```







## Quick start guide using docker compose

1. Create `~/ruuvi/config.toml` using the example config file as reference.

2. Use the supplied `docker-compose.yml` file as a starting point. If using
   Mosquitto it will require a config file `~/mosquitto/config/mosquitto.conf`.
   An example can be found in the
   [instructions](https://hub.docker.com/_/eclipse-mosquitto) for the
   eclipse-mosquitto image.

3. Start the containers with

   ```
   docker compose up
   ```

   when in the same folder as the `docker-compose.yml` file

4. In Home Assistant, add a new integration by selecting
   `Settings → Devices & services → Add integration → MQTT`. Type the host name
   and port. When using Docker networking the host name is the service name, in
   this case `mqtt`.

5. If discovery is enabled in `config.toml`, any nearby sensors should be
   discovered automatically to appear in Home Assistant.

## Troubleshooting

### Docker container logs

The docker container logs visible in docker compose attached mode can provide
valuable info about possible problems.

### Check that bluetooth is working on host machine

The `bluez-tools` package provides useful utilities to troubleshoot Bluetooth on
the host machine.

### Hardware compatibility

Bluetooth adapters may not always work with BLE. In testing I found that a generic USB Bluetooth adapter based on the Realtek RTL8761BU did not work correctly. The manufacturer of this adapter did not claim Linux compability. Replacing
with Asus USB-BT500 solved the problem.
