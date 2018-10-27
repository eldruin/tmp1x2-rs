#![deny(unsafe_code)]

extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Tmp1x2, Register, BitFlagsLow as BFL, BitFlagsHigh as BFH, Config,
             ConversionRate as CR, Error };


impl<I2C, E> Tmp1x2<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Enable the sensor.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb & !BFL::SHUTDOWN, msb)
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb | BFL::SHUTDOWN, msb)
    }

    /// Enable the extended measurement mode.
    ///
    /// This allows measurement of temperatures above 128°C.
    pub fn enable_extended_mode(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb, msb | BFH::EXTENDED_MODE)
    }

    /// Disable the extended measurement mode.
    ///
    /// This puts the device in normal measurement mode. It will not measure
    /// temperatures above 128°C.
    pub fn disable_extended_mode(&mut self) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        self.write_config(lsb, msb & !BFH::EXTENDED_MODE)
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
                                   self.config.lsb | BFL::ONE_SHOT])
            .map_err(Error::I2C)
    }

    /// Set the conversion rate when in continuous conversion mode.
    pub fn set_conversion_rate(&mut self, rate: CR) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        match rate {
            CR::_0_25Hz => self.write_config(lsb, msb & !BFH::CONV_RATE1 & !BFH::CONV_RATE0),
            CR::_1Hz    => self.write_config(lsb, msb & !BFH::CONV_RATE1 |  BFH::CONV_RATE0),
            CR::_4Hz    => self.write_config(lsb, msb |  BFH::CONV_RATE1 & !BFH::CONV_RATE0),
            CR::_8Hz    => self.write_config(lsb, msb |  BFH::CONV_RATE1 |  BFH::CONV_RATE0),
        }
    }

    /// Reset the internal state of this driver to the default values.
    ///
    /// *Note:* This does not alter the state or configuration of the device.
    ///
    /// This resets the cached configuration register value in this driver to
    /// the power-up (reset) configuration of the device.
    ///
    /// This needs to be called after performing a reset on the device, for
    /// example through an I2C general-call Reset command, which was not done
    /// through this driver to ensure that the configurations in the device
    /// and in the driver match.
    pub fn reset_internal_driver_state(&mut self) {
        self.config = Config::default();
    }

    fn write_config(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIG, msb, lsb])
            .map_err(Error::I2C)?;
        self.config = Config { lsb, msb };
        Ok(())
    }
}
