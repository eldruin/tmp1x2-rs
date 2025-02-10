//! This is a platform agnostic Rust driver for the TMP102 and TMP112
//! high-accuracy, low-power, digital temperature sensors, based on the
//! [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Change into one-shot or continuous conversion mode.
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
//! These temperature sensors are highly linear and do not require complex
//! calculations or lookup tables to derive the temperature. The on-chip
//! 12-bit ADC offers resolutions down to 0.0625°C.
//!
//! The TMP102 device is a digital temperature sensor ideal for NTC/PTC
//! thermistor replacement where high accuracy is required. The device offers an
//! accuracy of +/-0.5°C without requiring calibration or external component
//! signal conditioning.
//!
//! The TMP112 family of devices are digital temperature sensors designed for
//! high-accuracy, low-power, NTC/PTC thermistor replacements where high accuracy
//! is required. The TMP112A and TMP112B offers 0.5°C accuracy and are optimized
//! to provide the best PSR performance for 3.3V and 1.8V operation respectively,
//! while TMP112N offers 1°C accuracy.
//!
//! The devices feature SMBus(TM), two-wire and I2C interface compatibility,
//! and allows up to four devices on one bus.
//!
//! Datasheets:
//! - [TMP102](http://www.ti.com/lit/ds/symlink/tmp102.pdf)
//! - [TMP112x](http://www.ti.com/lit/ds/symlink/tmp112.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read temperature in continuous mode
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{Tmp1x2, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! // Per default the device is in continuous mode
//! let mut sensor = Tmp1x2::new(dev, address);
//! let temperature = sensor.read_temperature().unwrap();
//! println!("Temperature: {}", temperature);
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{Tmp1x2, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a1, a0) = (false, true);
//! let address = SlaveAddr::Alternative(a1, a0);
//! let mut sensor = Tmp1x2::new(dev, address);
//! ```
//!
//! ### Change into one-shot mode and trigger a measurement
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//! use tmp1x2::{Tmp1x2, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! let mut sensor = sensor.into_one_shot().ok().expect("Mode change error");
//! let temperature = block!(sensor.read_temperature());
//! ```
//!
//! ### Get the device back if there was an error during a mode change
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{ModeChangeError, Tmp1x2, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor_continuous = Tmp1x2::new(dev, SlaveAddr::default());
//! let result = sensor_continuous.into_one_shot();
//! if let Err(ModeChangeError::I2C(e, dev)) = result {
//!     sensor_continuous = dev;
//! } else if let Ok(one_shot_sensor) = result {
//!     // do something with one-shot sensor...
//! } else {
//!     unreachable!();
//! }
//! ```
//!
//! ### Enable the extended measurement mode
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{Tmp1x2, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.enable_extended_mode().unwrap();
//! ```
//!
//! ### Set the conversion rate to 1Hz
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{Tmp1x2, SlaveAddr, ConversionRate};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_conversion_rate(ConversionRate::_1Hz).unwrap();
//! ```
//!
//! ### Set the high and low temperature thresholds
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{Tmp1x2, SlaveAddr, ConversionRate};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_low_temperature_threshold(-15.0).unwrap();
//! sensor.set_high_temperature_threshold(60.0).unwrap();
//! ```
//!
//! ### Set the fault queue
//!
//! This sets the number of consecutive faults that will trigger an alert.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{Tmp1x2, SlaveAddr, FaultQueue};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_fault_queue(FaultQueue::_4).unwrap();
//! ```
//!
//! ### Set the alert polarity
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr, AlertPolarity };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_alert_polarity(AlertPolarity::ActiveHigh).unwrap();
//! ```
//!
//! ### Set the thermostat mode
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{ Tmp1x2, SlaveAddr, ThermostatMode };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! sensor.set_thermostat_mode(ThermostatMode::Interrupt).unwrap();
//! ```
//!
//! ### Check whether an alert is active as defined by the comparator mode
//!
//! Note that this ignores the thermostat mode setting and always refers to
//! the status as defined by the comparator mode.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tmp1x2::{Tmp1x2, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
//! let alert = sensor.is_comparator_mode_alert_active().unwrap();
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use core::marker::PhantomData;
use embedded_hal::i2c;
pub use nb;

/// Possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Error type for mode changes.
///
/// This allows to retrieve the unchanged device in case of an error.
#[derive(Debug)]
pub enum ModeChangeError<E, DEV> {
    /// I²C bus error while changing mode.
    ///
    /// `E` is the error that happened.
    /// `DEV` is the device with the mode unchanged.
    I2C(E, DEV),
}

/// Conversion rate for continuous conversion mode
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConversionRate {
    /// 0.25Hz
    _0_25Hz,
    /// 1Hz
    _1Hz,
    /// 4 Hz (default)
    #[default]
    _4Hz,
    /// 8 Hz
    _8Hz,
}

/// Fault queue
///
/// Number of consecutive faults necessary to trigger an alert.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultQueue {
    /// 1 fault will trigger an alert (default)
    #[default]
    _1,
    /// 2 consecutive faults will trigger an alert
    _2,
    /// 4 consecutive faults will trigger an alert
    _4,
    /// 6 consecutive faults will trigger an alert
    _6,
}

/// Alert polarity
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertPolarity {
    /// Active low (default)
    #[default]
    ActiveLow,
    /// Active high
    ActiveHigh,
}

/// Thermostat mode
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermostatMode {
    /// Comparator (default)
    ///
    /// In this mode an alert is generated (set alert pin and alert bit
    /// according to selected active polarity) when the temperature equals or
    /// exceeds the value set as *high* temperature threshold and remains
    /// active until the temperature falls below the value set as *low*
    /// temperature threshold.
    #[default]
    Comparator,
    /// Interrupt
    ///
    /// In this mode an alert is generated (set alert pin and alert bit
    /// according to selected active polarity) when the temperature exceeds the
    /// value set as *high* temperature threshold or goes below the value set
    /// as *low* temperature threshold.
    Interrupt,
}

/// Possible slave addresses
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlaveAddr {
    /// Default slave address
    #[default]
    Default,
    /// Alternative slave address providing bit values for A1 and A0
    Alternative(bool, bool),
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a1, a0) => default | ((a1 as u8) << 1) | a0 as u8,
        }
    }
}

const DEVICE_BASE_ADDRESS: u8 = 0b100_1000;

struct Register;

impl Register {
    const TEMPERATURE: u8 = 0x00;
    const CONFIG: u8 = 0x01;
    const T_LOW: u8 = 0x02;
    const T_HIGH: u8 = 0x03;
}

struct BitFlagsHigh;

impl BitFlagsHigh {
    const SHUTDOWN: u8 = 0b0000_0001;
    const THERMOSTAT: u8 = 0b0000_0010;
    const ALERT_POLARITY: u8 = 0b0000_0100;
    const FAULT_QUEUE0: u8 = 0b0000_1000;
    const FAULT_QUEUE1: u8 = 0b0001_0000;
    const RESOLUTION: u8 = 0b0110_0000;
    const ONE_SHOT: u8 = 0b1000_0000;
}

struct BitFlagsLow;

impl BitFlagsLow {
    const EXTENDED_MODE: u8 = 0b0001_0000;
    const ALERT: u8 = 0b0010_0000;
    const CONV_RATE0: u8 = 0b0100_0000;
    const CONV_RATE1: u8 = 0b1000_0000;
}

#[derive(Debug, Clone)]
struct RegisterU16 {
    lsb: u8,
    msb: u8,
}

type Config = RegisterU16;

impl Default for Config {
    fn default() -> Self {
        Config {
            msb: BitFlagsHigh::RESOLUTION,
            lsb: BitFlagsLow::ALERT | BitFlagsLow::CONV_RATE1,
        }
    }
}

impl RegisterU16 {
    fn with_high_msb(&self, mask: u8) -> Self {
        Config {
            msb: self.msb | mask,
            lsb: self.lsb,
        }
    }
    fn with_low_msb(&self, mask: u8) -> Self {
        Config {
            msb: self.msb & !mask,
            lsb: self.lsb,
        }
    }
    fn with_high_lsb(&self, mask: u8) -> Self {
        Config {
            msb: self.msb,
            lsb: self.lsb | mask,
        }
    }
    fn with_low_lsb(&self, mask: u8) -> Self {
        Config {
            msb: self.msb,
            lsb: self.lsb & !mask,
        }
    }
}

#[doc(hidden)]
pub mod marker {
    pub mod mode {
        #[derive(Debug)]
        pub struct Continuous(());
        #[derive(Debug)]
        pub struct OneShot(());
    }
}

/// TMP1X2 device driver.
#[derive(Debug, Default)]
pub struct Tmp1x2<I2C, MODE> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: Config,
    /// A temperature conversion was started.
    a_temperature_conversion_was_started: bool,
    _mode: PhantomData<MODE>,
}

impl<I2C, E> Tmp1x2<I2C, marker::mode::Continuous>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Create new instance of the TMP102 or TMP112x device.
    ///
    /// By default they are in continuous conversion mode.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Tmp1x2 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: Config::default(),
            a_temperature_conversion_was_started: false,
            _mode: PhantomData,
        }
    }
}

impl<I2C, MODE> Tmp1x2<I2C, MODE> {
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

mod configuration;
mod conversion;
mod interface;
mod reading;

//impl<E> core::fmt::Debug for nb::Error<E> {}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock as hal;
    use DEVICE_BASE_ADDRESS as BASE_ADDR;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(BASE_ADDR, addr.addr(BASE_ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(
            0b100_1000,
            SlaveAddr::Alternative(false, false).addr(BASE_ADDR)
        );
        assert_eq!(
            0b100_1001,
            SlaveAddr::Alternative(false, true).addr(BASE_ADDR)
        );
        assert_eq!(
            0b100_1010,
            SlaveAddr::Alternative(true, false).addr(BASE_ADDR)
        );
        assert_eq!(
            0b100_1011,
            SlaveAddr::Alternative(true, true).addr(BASE_ADDR)
        );
    }

    #[test]
    fn default_config() {
        let dev = Tmp1x2::new(hal::eh1::i2c::Mock::new(&[]), SlaveAddr::default());
        assert_eq!(0b0110_0000, dev.config.msb);
        assert_eq!(0b1010_0000, dev.config.lsb);
        dev.destroy().done()
    }
}
