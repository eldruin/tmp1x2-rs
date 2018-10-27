#![deny(unsafe_code)]

extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Tmp1x2, Register, BitFlagsLow as BFL, BitFlagsHigh as BFH, Config,
             ConversionRate as CR, FaultQueue, AlertPolarity, ThermostatMode,
             Error };
use super::conversion::{ convert_temp_to_register_normal,
                         convert_temp_to_register_extended };


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

    /// Set the high temperature threshold.
    ///
    /// The value provided will be capped to be in the interval
    /// `[-128.0, 127.9375]` in normal mode and `[-256.0, 255.875]` in
    /// extended mode.
    pub fn set_high_temperature_threshold(&mut self, temperature: f32) -> Result<(), Error<E>> {
        self.set_temperature_threshold(temperature, Register::T_HIGH)
    }

    /// Set the low temperature threshold.
    ///
    /// The value provided will be capped to be in the interval
    /// `[-128.0, 127.9375]` in normal mode and `[-256.0, 255.875]` in
    /// extended mode.
    pub fn set_low_temperature_threshold(&mut self, temperature: f32) -> Result<(), Error<E>> {
        self.set_temperature_threshold(temperature, Register::T_LOW)
    }

    fn set_temperature_threshold(&mut self, temperature: f32, register: u8) -> Result<(), Error<E>> {
        if (self.config.msb & BFH::EXTENDED_MODE) != 0 {
            let (msb, lsb) = convert_temp_to_register_extended(temperature);
            self.write_register(register, lsb, msb)
        }
        else {
            let (msb, lsb) = convert_temp_to_register_normal(temperature);
            self.write_register(register, lsb, msb)
        }
    }

    /// Set the fault queue.
    ///
    /// Set the number of consecutive faults that will trigger an alert.
    pub fn set_fault_queue(&mut self, fq: FaultQueue) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        match fq {
            FaultQueue::_1 => self.write_config(lsb & !BFL::FAULT_QUEUE1 & !BFL::FAULT_QUEUE0, msb),
            FaultQueue::_2 => self.write_config(lsb & !BFL::FAULT_QUEUE1 |  BFL::FAULT_QUEUE0, msb),
            FaultQueue::_4 => self.write_config(lsb |  BFL::FAULT_QUEUE1 & !BFL::FAULT_QUEUE0, msb),
            FaultQueue::_6 => self.write_config(lsb |  BFL::FAULT_QUEUE1 |  BFL::FAULT_QUEUE0, msb),
        }
    }

    /// Set the alert polarity.
    pub fn set_alert_polarity(&mut self, polarity: AlertPolarity) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        match polarity {
            AlertPolarity::ActiveLow  => self.write_config(lsb & !BFL::ALERT_POLARITY, msb),
            AlertPolarity::ActiveHigh => self.write_config(lsb |  BFL::ALERT_POLARITY, msb),
        }
    }

    /// Set the thermostat mode.
    pub fn set_thermostat_mode(&mut self, mode: ThermostatMode) -> Result<(), Error<E>> {
        let Config{ lsb, msb } = self.config;
        match mode {
            ThermostatMode::Comparator => self.write_config(lsb & !BFL::THERMOSTAT, msb),
            ThermostatMode::Interrupt  => self.write_config(lsb |  BFL::THERMOSTAT, msb),
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
        self.write_register(Register::CONFIG, lsb, msb)?;
        self.config = Config { lsb, msb };
        Ok(())
    }

    fn write_register(&mut self, register: u8, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, msb, lsb])
            .map_err(Error::I2C)
    }
}
