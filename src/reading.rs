use crate::conversion::convert_temp_from_register;
use crate::{marker::mode, BitFlagsHigh, BitFlagsLow, Error, Register, Tmp1x2};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Tmp1x2<I2C, mode::Continuous>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Read the temperature from the sensor.
    pub async fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let data = self.read_register_u16(Register::TEMPERATURE).await?;
        Ok(convert_temp_from_register(data.msb, data.lsb))
    }
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Tmp1x2<I2C, mode::OneShot>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Read whether the one-shot measurement result is ready.
    async fn one_shot_measurement_is_ready(&mut self) -> Result<bool, Error<E>> {
        let config = self.read_register_u16(Register::CONFIG).await?;
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
    pub async fn read_temperature(&mut self) -> nb::Result<f32, Error<E>> {
        if !self.a_temperature_conversion_was_started {
            self.trigger_one_shot_measurement()
                .await
                .map_err(nb::Error::Other)?;
            self.a_temperature_conversion_was_started = true;
            return Err(nb::Error::WouldBlock);
        }
        if !self
            .one_shot_measurement_is_ready()
            .await
            .map_err(nb::Error::Other)?
        {
            Err(nb::Error::WouldBlock)
        } else {
            let data = self
                .read_register_u16(Register::TEMPERATURE)
                .await
                .map_err(nb::Error::Other)?;
            let temp = convert_temp_from_register(data.msb, data.lsb);
            self.a_temperature_conversion_was_started = false;
            Ok(temp)
        }
    }
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E, MODE> Tmp1x2<I2C, MODE>
where
    I2C: AsyncI2c<Error = E>,
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
    pub async fn is_comparator_mode_alert_active(&mut self) -> Result<bool, Error<E>> {
        let config = self.read_register_u16(Register::CONFIG).await?;
        let is_alert_polarity_high = (config.msb & BitFlagsHigh::ALERT_POLARITY) != 0;
        let alert_status = (config.lsb & BitFlagsLow::ALERT) != 0;
        Ok(is_alert_polarity_high == alert_status)
    }
}
