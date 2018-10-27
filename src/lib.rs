//! This is a platform agnostic Rust driver for the TMP102 and TMP112
//! high-accuracy, low-power, digital temperature sensors, based on the
//! [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device.
//! - Read the temperature.
//! - Enable/disable the extended measurement mode.
//! - Trigger a one-shot measurement.
//! - Read whether the one-shot measurement result is ready.
//! - Set the conversion rate.
//! - Set the high/low temperature threshold.
//! - Set the fault queue.
//! - Set the alert polarity.
//! - Set the thermostat mode.
//! - Read whether a comparator mode alert is active.
//!
//! ## The devices
//!
//! This driver is compatible with both the TMP102 device as well as the TMP112
//! family of devices, including TMP112A, TMP112B and TMP112N.
//! 
//! ### TMP102
//! The TMP102 device is a digital temperature sensor ideal for NTC/PTC
//! thermistor replacement where high accuracy is required. The device offers
//! an accuracy of +/-0.5°C without requiring calibration or external component
//! signal conditioning. Device temperature sensors are highly linear and do
//! not require complex calculations or lookup tables to derive the
//! temperature. The on-chip 12-bit ADC offers resolutions down to 0.0625°C.
//!
//! The TMP112 family features SMBus(TM), two-wire and I2C interface
//! compatibility, and allows up to four devices on one bus. The device also
//! features an SMBus alert function. The device is specified to operate over
//! supply voltages from 1.4 to 3.6 V with the maximum quiescent current of
//! 10 μA over the full operating range.
//!
//! The TMP102 device is ideal for extended temperature measurement in a
//! variety of communication, computer, consumer, environmental, industrial,
//! and instrumentation applications. The device is specified for operation
//! over a temperature range of -40°C to 125°C.
//!
//! ### TMP112
//! The TMP112 family of devices are digital temperature sensors designed for
//! high-accuracy, low-power, NTC/PTC thermistor replacements where high
//! accuracy is required. The TMP112A and TMP112B offers 0.5°C accuracy and are
//! optimized to provide the best PSR performance for 3.3V and 1.8V operation
//! respectively, while TMP112N offers 1°C accuracy. These temperature sensors
//! are highly linear and do not require complex calculations or lookup tables
//! to derive the temperature. The on-chip 12-bit ADC offers resolutions down
//! to 0.0625°C.
//!
//! The TMP112 family features SMBus, two-wire and I2C interface compatibility,
//! and allows up to four devices on one bus. The device also features an SMBus
//! alert function. The device is specified to operate over supply voltages
//! from 1.4 to 3.6 V with the maximum quiescent current of 10 μA over the full
//! operating range.
//!
//! The TMP112 family is designed for extended temperature measurement in
//! communication, computer, consumer, environmental, industrial, and
//! instrumentation applications. The device is specified for operation over a
//! temperature range of -40°C to +125°C.
//!
//! Datasheets:
//! - [TMP102](http://www.ti.com/lit/ds/symlink/tmp102.pdf)
//! - [TMP112x](http://www.ti.com/lit/ds/symlink/tmp112.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! ### Read temperature
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Tmp1x2::new(dev, address);
//! let temperature = sensor.read_temperature().unwrap();
//! println!("Temperature: {}", temperature);
//! # }
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a1, a0) = (false, true);
//! let address = SlaveAddr::Alternative(a1, a0);
//! let mut sensor = Tmp1x2::new(dev, address);
//! # }
//! ```
//!
//! ### Enable / disable the sensor
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.disable().unwrap(); // shutdown
//! sensor.enable().unwrap();
//! # }
//! ```
//!
//! ### Enable the extended measurement mode
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.enable_extended_mode().unwrap();
//! # }
//! ```
//!
//! ### Trigger a one-shot measurement while in shutdown
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.disable().unwrap(); // shutdown
//! sensor.trigger_one_shot_measurement().unwrap();
//! while(!sensor.is_one_shot_measurement_result_ready().unwrap()) {
//!     // insert some delay here
//! }
//! let temperature = sensor.read_temperature().unwrap();
//! # }
//! ```
//!
//! ### Set the conversion rate to 1Hz
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr, ConversionRate };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_conversion_rate(ConversionRate::_1Hz).unwrap();
//! # }
//! ```
//!
//! ### Set the high and low temperature thresholds
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr, ConversionRate };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_low_temperature_threshold(-15.0).unwrap();
//! sensor.set_high_temperature_threshold(60.0).unwrap();
//! # }
//! ```
//!
//! ### Set the fault queue
//!
//! This sets the number of consecutive faults that will trigger an alert.
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr, FaultQueue };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_fault_queue(FaultQueue::_4).unwrap();
//! # }
//! ```
//!
//! ### Set the alert polarity
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr, AlertPolarity };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_alert_polarity(AlertPolarity::ActiveHigh).unwrap();
//! # }
//! ```
//!
//! ### Set the thermostat mode
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr, ThermostatMode };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_thermostat_mode(ThermostatMode::Interrupt).unwrap();
//! # }
//! ```
//!
//! ### Check whether an alert is active as defined by the comparator mode
//!
//! Note that this ignores the thermostat mode setting and always refers to
//! the status as defined by the comparator mode.
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tmp1x2;
//!
//! use hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! let alert = sensor.is_comparator_mode_alert_active().unwrap();
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Conversion rate for continuous conversion mode
#[derive(Debug, Clone)]
pub enum ConversionRate {
    /// 0.25Hz
    _0_25Hz,
    /// 1Hz
    _1Hz,
    /// 4 Hz (default)
    _4Hz,
    /// 8 Hz
    _8Hz,
}

/// Fault queue
///
/// Number of consecutive faults necessary to trigger an alert.
#[derive(Debug, Clone)]
pub enum FaultQueue {
    /// 1 fault will trigger an alert (default)
    _1,
    /// 2 consecutive faults will trigger an alert
    _2,
    /// 4 consecutive faults will trigger an alert
    _4,
    /// 6 consecutive faults will trigger an alert
    _6,
}
        
/// Alert polarity
#[derive(Debug, Clone)]
pub enum AlertPolarity {
    /// Active low (default)
    ActiveLow,
    /// Active high
    ActiveHigh
}

/// Thermostat mode
#[derive(Debug, Clone)]
pub enum ThermostatMode {
    /// Comparator (default)
    ///
    /// In this mode an alert is generated (set alert pin and alert bit
    /// according to selected active polarity) when the temperature equals or
    /// exceeds the value set as *high* temperature threshold and remains
    /// active until the temperature falls below the value set as *low*
    /// temperature threshold.
    Comparator,
    /// Interrupt
    ///
    /// In this mode an alert is generated (set alert pin and alert bit
    /// according to selected active polarity) when the temperature exceeds the
    /// value set as *high* temperature threshold or goes below the value set
    /// as *low* temperature threshold.
    Interrupt
}

/// Possible slave addresses
#[derive(Debug, Clone)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A1 and A0
    Alternative(bool, bool)
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a1, a0) => default           |
                                              ((a1 as u8) << 1) |
                                                a0 as u8
        }
    }
}


const DEVICE_BASE_ADDRESS: u8 = 0b100_1000;

struct Register;

impl Register {
    const TEMPERATURE : u8 = 0x00;
    const CONFIG      : u8 = 0x01;
    const T_LOW       : u8 = 0x02;
    const T_HIGH      : u8 = 0x03;
}

struct BitFlagsLow;

impl BitFlagsLow {
    const SHUTDOWN        : u8 = 0b0000_0001;
    const THERMOSTAT      : u8 = 0b0000_0010;
    const ALERT_POLARITY  : u8 = 0b0000_0100;
    const FAULT_QUEUE0    : u8 = 0b0000_1000;
    const FAULT_QUEUE1    : u8 = 0b0001_0000;
    const RESOLUTION      : u8 = 0b0110_0000;
    const ONE_SHOT        : u8 = 0b1000_0000;
}

struct BitFlagsHigh;

impl BitFlagsHigh {
    const EXTENDED_MODE : u8 = 0b0001_0000;
    const ALERT         : u8 = 0b0010_0000;
    const CONV_RATE0    : u8 = 0b0100_0000;
    const CONV_RATE1    : u8 = 0b1000_0000;
}

#[derive(Debug, Clone)]
struct Config {
    lsb: u8,
    msb: u8
}

impl Default for Config {
    fn default() -> Self {
        Config { lsb: BitFlagsLow::RESOLUTION,
                 msb: BitFlagsHigh::ALERT | BitFlagsHigh::CONV_RATE1 }
    }
}

/// TMP1X2 device driver.
#[derive(Debug, Default)]
pub struct Tmp1x2<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: Config,
}

impl<I2C, E> Tmp1x2<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Create new instance of the TMP102 or TMP112x device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Tmp1x2 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: Config::default()
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

mod configuration;
mod conversion;
mod reading;

#[cfg(test)]
mod tests {
    use super::*;
    extern crate embedded_hal_mock as hal;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b100_1000, SlaveAddr::Alternative(false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1001, SlaveAddr::Alternative(false,  true).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1010, SlaveAddr::Alternative( true, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1011, SlaveAddr::Alternative( true,  true).addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn default_config() {
        let dev = Tmp1x2::new(hal::i2c::Mock::new(&[]), SlaveAddr::default());
        assert_eq!(0b0110_0000, dev.config.lsb);
        assert_eq!(0b1010_0000, dev.config.msb);
    }
}