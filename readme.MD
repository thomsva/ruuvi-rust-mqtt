[![Build and publish Docker Image](https://github.com/thomsva/ruuvi-rust-mqtt/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/thomsva/ruuvi-rust-mqtt/actions/workflows/docker-publish.yml)

# ruuvi-rust-mqtt
Simple program to gather data from RuuviTag sensors and publish to mqtt broker. Easy integration with Home-Assistant.

The program seems to work fine but has not been thorougly tested.

## Features
- Decodes raw data from RuuviTag sensors into temperature, pressure and humidity
- Use with Docker Compose
- Filter sensors using blacklist and/or whitelist
- Publish discovery messages for Home-Assistant

## Requirements
- Linux computer with Docker installed
- Access to the default Bluetooth interface
- RuuviTag sensors using Data format 5
- A valid config.toml file

## Use with Docker compose
Create a ~/ruuvi/config.toml using the example config file as reference. 

Use the supplied Docker-compose.yml file example and adapt as needed. The example adds containers for Mosquitto MQTT broker and Home-Assistant. Remove or change those as needed. 

Start the services with 
`docker compose up`




