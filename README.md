# Aerodis Space Esp32 NRF24L01

This Rust crate provides a wrapper around the ESP32 SPI Hardware Abstraction Layer (HAL), specifically tailored for controlling the nRF24L01 radio module. It aims to simplify the interaction with the nRF24L01 by abstracting the lower-level SPI commands into higher-level Rust functions.

## Features

- Easy-to-use Rust interface for nRF24L01 radio module.
- Abstraction over ESP32 SPI HAL for nRF24L01 operations.
- Implementation of essential nRF24L01 commands for communication setup.
- Rust idiomatic error handling for robustness.

## Installation

To include this crate in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
aerodis_space_esp32_nrf24l01 = { git = "https://github.com/aerodisSpace/aerodis_space_esp32_nrf24l01.git", branch = "main" }
