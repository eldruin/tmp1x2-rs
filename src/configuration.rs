#![deny(unsafe_code)]

extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Tmp1x2, Register, BitFlagsLow, BitFlagsHigh, Config,
             ConversionRate as CR, Error };


impl<I2C, E> Tmp1x2<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Enable the sensor.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb & !BitFlagsLow::SHUTDOWN, msb)
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb | BitFlagsLow::SHUTDOWN, msb)
    }

    /// Enable the extended measurement mode.
    ///
    /// This allows measurement of temperatures above 128°C.
    pub fn enable_extended_mode(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb, msb | BitFlagsHigh::EXTENDED_MODE)
    }

    /// Disable the extended measurement mode.
    ///
    /// This puts the device in normal measurement mode. It will not measure
    /// temperatures above 128°C.
    pub fn disable_extended_mode(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb, msb & !BitFlagsHigh::EXTENDED_MODE)
    }

    /// Trigger a one-shot measurement when in shutdown mode (disabled).
    ///
    /// This allows triggering a single temperature measurement when in
    /// shutdown mode. The device returns to the shutdown state at the
    /// completion of the temperature conversion. This reduces power
    /// consumption when continuous temperature monitoring is not required.
    ///
    /// See also: [is_one_shot_measurement_result_ready()](#method.is_one_shot_measurement_result_ready)
    pub fn trigger_one_shot_measurement(&mut self) -> Result<(), Error<E>> {
        // This bit is not stored
        self.i2c
            .write(self.address, &[Register::CONFIG, self.config.msb,
                                   self.config.lsb | BitFlagsLow::ONE_SHOT])
            .map_err(Error::I2C)
    }

    /// Set the conversion rate when in continuous conversion mode.
    pub fn set_conversion_rate(&mut self, rate: CR) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        match rate {
            CR::_0_25Hz => self.write_config(lsb, msb & !BitFlagsHigh::CONV_RATE1 & !BitFlagsHigh::CONV_RATE0),
            CR::_1Hz    => self.write_config(lsb, msb & !BitFlagsHigh::CONV_RATE1 |  BitFlagsHigh::CONV_RATE0),
            CR::_4Hz    => self.write_config(lsb, msb |  BitFlagsHigh::CONV_RATE1 & !BitFlagsHigh::CONV_RATE0),
            CR::_8Hz    => self.write_config(lsb, msb |  BitFlagsHigh::CONV_RATE1 |  BitFlagsHigh::CONV_RATE0),
        }
    }

    fn write_config(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIG, msb, lsb])
            .map_err(Error::I2C)?;
        self.config = Config { lsb, msb };
        Ok(())
    }
}
