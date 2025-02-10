use crate::conversion::convert_temp_from_register;
use crate::{marker::mode, BitFlagsHigh, BitFlagsLow, Error, Register, Tmp1x2};
use embedded_hal::i2c;

impl<I2C, E> Tmp1x2<I2C, mode::Continuous>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Read the temperature from the sensor.
    pub fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let data = self.read_register_u16(Register::TEMPERATURE)?;
        Ok(convert_temp_from_register(data.msb, data.lsb))
    }
}

impl<I2C, E> Tmp1x2<I2C, mode::OneShot>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Read whether the one-shot measurement result is ready.
    fn one_shot_measurement_is_ready(&mut self) -> Result<bool, Error<E>> {
        let config = self.read_register_u16(Register::CONFIG)?;
        Ok((config.msb & BitFlagsHigh::ONE_SHOT) != 0)
    }

    /// Perform a one-shot temperature measurement.
    ///
    /// This allows triggering a single temperature measurement when in
    /// shutdown mode. The device returns to the shutdown state at the
    /// completion of the temperature conversion. This reduces power
    /// consumption when continuous temperature monitoring is not required.
    ///
    /// If no temperature conversion was started yet, calling this method
    /// will start one and return `nb::Error::WouldBlock`. Subsequent calls
    /// will continue to return `nb::Error::WouldBlock` until the
    /// temperature measurement is finished. Then it will return the
    /// measured temperature.
    pub fn read_temperature(&mut self) -> nb::Result<f32, Error<E>> {
        if !self.a_temperature_conversion_was_started {
            self.trigger_one_shot_measurement()
                .map_err(nb::Error::Other)?;
            self.a_temperature_conversion_was_started = true;
            return Err(nb::Error::WouldBlock);
        }
        if !self
            .one_shot_measurement_is_ready()
            .map_err(nb::Error::Other)?
        {
            Err(nb::Error::WouldBlock)
        } else {
            let data = self
                .read_register_u16(Register::TEMPERATURE)
                .map_err(nb::Error::Other)?;
            let temp = convert_temp_from_register(data.msb, data.lsb);
            self.a_temperature_conversion_was_started = false;
            Ok(temp)
        }
    }
}

impl<I2C, E, MODE> Tmp1x2<I2C, MODE>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Read whether an alert is active as defined by the comparator mode.
    ///
    /// *NOTE*: This ignores the thermostat mode setting and always corresponds
    /// to the activation status as defined by the comparator mode.
    ///
    /// This method takes into account the alert polarity selected.
    ///
    /// See also: [ThermostatMode](enum.ThermostatMode.html),
    /// [AlertPolarity](enum.AlertPolarity.html).
    #[allow(clippy::wrong_self_convention)]
    pub fn is_comparator_mode_alert_active(&mut self) -> Result<bool, Error<E>> {
        let config = self.read_register_u16(Register::CONFIG)?;
        let is_alert_polarity_high = (config.msb & BitFlagsHigh::ALERT_POLARITY) != 0;
        let alert_status = (config.lsb & BitFlagsLow::ALERT) != 0;
        Ok(is_alert_polarity_high == alert_status)
    }
}
