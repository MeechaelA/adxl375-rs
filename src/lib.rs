#![allow(non_camel_case_types)]

//! Adxl375 embedded-hal SPI driver crate
//!
//! A platform agnostic driver to interface with the Adxl375 Accelerometer.
//! This driver uses SPI via [embedded-hal] and implements the [`Accelerometer` trait][trait]
//! from the `accelerometer` crate.
//!
//! [embedded-hal]: https://docs.rs/embedded-hal
//! [trait]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html
//!
//!
//! # Usage
//!
//! Use embedded-hal implementation to get SPI and a GPIO OutputPin for the chip select,
//! then create the accelerometer handle
//!
//! ```
//!
//! use Adxl375::{Adxl375, Config as ADXLConfig, ODR_LPF, Range, Accelerometer};
//!
//! // to create sensor with default configuration:
//! let mut accelerometer = Adxl375::default(spi, cs)?;
//!
//! // start measurements
//! accelerometer.start();
//!
//! // to get 3d accerlation data:
//! let accel = accelerometer.accel_norm()?;
//! println!("{:?}", accel);
//!
//! // One can also use conf module to supply configuration:
//!
//! let mut accelerometer =
//!     Adxl375::new(spi, cs,
//!                     ADXLConfig::new()
//!                     .odr(ODR_LPF::ODR_31_25_Hz)
//!                     .range(Range::_2G))?;
//! ```
//!
//! # References
//!
//! - [Register Map][1]
//!
//! [1]: https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL375.PDF
//!
//! - [`embedded-hal`][2]
//!
//! [2]: https://github.com/rust-embedded/embedded-hal
//!
//!



#![no_std]

mod conf;
mod register;

use core::fmt::Debug;

use embedded_hal as hal;

use hal::blocking::spi;
use hal::digital::v2::OutputPin;

pub use accelerometer::{Accelerometer, RawAccelerometer, error, Error, vector::{I32x3, F32x3}};

pub use conf::*;
use register::Register;

const SPI_READ: u8 = 0x01;
const SPI_WRITE: u8 = 0x00;

const EXPECTED_DEVICE_ID: u8 = 0xE5;

const ACCEL_MAX_I20: u32 = 524_287; // = 2^(20-1)-1


/// Adxl375 driver
pub struct Adxl375<SPI, CS> {
    spi: SPI,
    cs: CS,

    // configuration
    odr: ODR_LPF,
}


impl<SPI, CS, E, PinError> Adxl375<SPI, CS>
where
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    CS: OutputPin<Error = PinError>
{


    /// Creates a new `Adxl375` driver from a SPI peripheral with
    /// default configuration.
    pub fn default(spi:SPI, cs:CS) -> Result<Self, E> {
        Adxl375::new(spi, cs, &Config::new())
    }

    /// Takes a config object to initialize the Adxl375 driver
    pub fn new(spi:SPI, cs:CS, config: &Config) -> Result<Self, E> {
        let mut Adxl375 = Adxl375 {
            spi,
            cs,
            odr: config.odr.unwrap_or_default(),
        };


        let id = Adxl375.get_device_id();

        if id != EXPECTED_DEVICE_ID {
            // error

        }
        


        Ok(Adxl375)
    }

    /// Puts the device in `Measurement mode`. The defaut after power up is `Standby mode`.
    pub fn start(&mut self) {
        self.write_reg(Register::POWER_CTL.addr(), 0);
        self.write_reg(Register::POWER_CTL.addr(), 0x0F);
        //self.write_reg(Register::DATA_FORMAT.addr(), 0);
    }

    /// Get the device ID
    pub fn get_device_id(&mut self) -> u8 {
        let reg = Register::DEVID.addr();
        let mut output = [1u8];
        self.read_reg(reg, &mut output);
        output[0]
    }

    pub fn write_reg(&mut self, reg: u8, value: u8) {
        let mut bytes = [(reg << 1)  | SPI_WRITE, value];
        self.cs.set_low().ok();
        self.spi.write(&mut bytes).ok();
        self.cs.set_high().ok();
    }

    pub fn read_reg(&mut self, reg: u8, buffer: &mut [u8]) {
        let mut bytes = [(reg << 1)  | SPI_READ, 0];
        self.cs.set_low().ok();
        self.spi.transfer(&mut bytes).ok();
        self.cs.set_high().ok();
        buffer[0] = bytes[1];
    }

    pub fn read(&mut self, bytes: &mut [u8]) {
        self.cs.set_low().ok();
        self.spi.transfer(bytes).ok();
        self.cs.set_high().ok();
    }
}

impl<SPI, CS, E, EO> RawAccelerometer<I32x3> for Adxl375<SPI, CS>
where
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    CS: OutputPin<Error = EO>,
    E: Debug
{
    type Error = E;

    /// Gets acceleration vector reading from the accelerometer
    /// Returns a 3D vector with x,y,z, fields in a Result
    fn accel_raw(&mut self) -> Result<I32x3, Error<E>> {
        let mut bytes = [0u8; 9+1];
        bytes[0] = (Register::DATAX0.addr() << 1)  | SPI_READ;
        self.read(&mut bytes);

        // combine 3 bytes into one i32 value
        // right-shift with sign-extend to 20-bit
        let x = ((((bytes[1] as i32) << 24) | ((bytes[2] as i32) << 16) | ((bytes[3] & 0xF0) as i32) << 8)) >> 12;
        let y = ((((bytes[4] as i32) << 24) | ((bytes[5] as i32) << 16) | ((bytes[6] & 0xF0) as i32) << 8)) >> 12;
        let z = ((((bytes[7] as i32) << 24) | ((bytes[8] as i32) << 16) | ((bytes[9] & 0xF0) as i32) << 8)) >> 12;

        Ok(I32x3::new(x, y, z))
    }

}

impl<SPI, CS, E, PinError> Accelerometer for Adxl375<SPI, CS>
where
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    CS: OutputPin<Error = PinError>,
    E: Debug
{
    type Error = E;

    fn sample_rate(&mut self) -> Result<f32, Error<Self::Error>> {
        Ok(self.odr.into())
    }

    fn accel_norm(&mut self) -> Result<F32x3, Error<Self::Error>> {
        let raw_data: I32x3 = self.accel_raw()?;

        let x = (raw_data.x as f32 / ACCEL_MAX_I20 as f32);
        let y = (raw_data.y as f32 / ACCEL_MAX_I20 as f32);
        let z = (raw_data.z as f32 / ACCEL_MAX_I20 as f32);

        Ok(F32x3::new(x, y, z))
    }
}