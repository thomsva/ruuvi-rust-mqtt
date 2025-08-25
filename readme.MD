[![Publish Docker release image](https://github.com/thomsva/ruuvi-rust-mqtt/actions/workflows/docker-release.yml/badge.svg)](https://github.com/thomsva/ruuvi-rust-mqtt/actions/workflows/docker-release.yml)

# ruuvi-rust-mqtt

Simple program to gather data from RuuviTag sensors and publish to MQTT broker.
Easy integration with Home Assistant.

> ⚠️ The program seems to work fine but has not been thoroughly tested.

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
- RuuviTag sensors using
  [Data format 5 (RAWv2)](https://docs.ruuvi.com/communication/bluetooth-advertisements/data-format-5-rawv2).
- A valid config.toml file.

## Quick start using docker compose

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

### Read the logs

The docker container logs visible in docker compose attached mode can provide
valuable info about possible problems.

### Check that bluetooth is working on host machine

The `bluez-tools` package provides useful utilities to troubleshoot Bluetooth on
the host machine.

### Hardware combability

Bluetooth adapters may not always work with BLE. A generic USB Bluetooth adapter
based on the Realtek RTL8761BU chip failed intermittently in long term testing
(the manufacturer of this adapter did not claim Linux compability). Replacing
with Asus USB-BT500 solved the problem.
