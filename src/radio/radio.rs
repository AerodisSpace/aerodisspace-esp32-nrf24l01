#![allow(unused)]

use super::spi_cmds::CommandNRF24SPI;
use esp_idf_hal::{gpio::*, peripheral::Peripheral, prelude::*, spi::*};
use log::error;

pub struct Esp32Nrf24l01<'a> {
    _spi_device_driver: SpiDeviceDriver<'a, SpiDriver<'a>>,
    pub config: config::Config, // SPI Configuration
}

impl<'a> Esp32Nrf24l01<'a> {
    /// Creates a new instance of the NRF24L01 driver
    /// <br>
    /// This function will initialize the SPI peripheral and the pins for the CE and CSN pins
    pub fn new<GpioPinCE, GpioPinCSN, GpioPinSCK, GpioPinMOSI, GpioPinMISO>(
        spi: SPI2,
        ce: GpioPinCE,
        csn: GpioPinCSN,
        sck: GpioPinSCK,
        mosi: GpioPinMOSI,
        miso: GpioPinMISO,
        cfg: Option<config::Config>,
    ) -> Result<Self, Esp32Nrf24l01Error>
    // FIXME: For some reason, the Traits are not being recognized just for a common generic type for GpioPin,
    // so I had to specify the traits for each pin
    where
        GpioPinCE: IOPin,
        GpioPinCSN: IOPin,
        GpioPinSCK: IOPin,
        GpioPinMOSI: IOPin,
        GpioPinMISO: IOPin,
    {
        let driver = match SpiDriver::new(spi, sck, miso, Some(mosi), &SpiDriverConfig::new()) {
            Ok(_driver) => _driver,
            Err(_err) => Err(Esp32Nrf24l01Error::SpiError(Some(_err.to_string())))?,
        };

        // Downgrade the pins to be used as output
        let ce = ce.downgrade_output();
        let csn = csn.downgrade_output();

        let config = if let Some(cfg) = cfg {
            cfg
        } else {
            config::Config::default().baudrate(10_000_000.into())
        };

        let _spi_device_driver = match SpiDeviceDriver::new(driver, Some(ce), &config) {
            Ok(_spi_dd) => _spi_dd,
            Err(_err) => Err(Esp32Nrf24l01Error::SpiError(Some(_err.to_string())))?,
        };

        Ok(Esp32Nrf24l01 {
            _spi_device_driver,
            config,
        })
    }
}

#[derive(Debug)]
pub enum Esp32Nrf24l01Error {
    SpiError(Option<String>),
    PinError(Option<String>),
    InvalidCommand(Option<String>),
    UnknownError(Option<String>),
}

impl std::fmt::Display for Esp32Nrf24l01Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Esp32Nrf24l01Error::SpiError(msg) => {
                write!(f, "SPI Error: {:?}", msg)
            }
            Esp32Nrf24l01Error::InvalidCommand(msg) => {
                write!(f, "Invalid Command: {:?}", msg)
            }
            Esp32Nrf24l01Error::UnknownError(msg) => {
                write!(f, "Unknown Error: {:?}", msg)
            }
            Esp32Nrf24l01Error::PinError(msg) => {
                write!(f, "Pin Error: {:?}", msg)
            }
        }
    }
}
impl std::error::Error for Esp32Nrf24l01Error {}
