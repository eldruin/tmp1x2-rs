use crate::conversion::{convert_temp_to_register_extended, convert_temp_to_register_normal};
use crate::RegisterU16;
use crate::{
    marker::mode, AlertPolarity, BitFlagsHigh as BFH, BitFlagsLow as BFL, Config,
    ConversionRate as CR, Error, FaultQueue, ModeChangeError, Register, ThermostatMode, Tmp1x2,
};
use core::marker::PhantomData;
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
    /// Change into one-shot conversion mode (shutdown).
    ///
    /// If the mode change failed you will get a `ModeChangeError`.
    /// You can get the unchanged device back from it.
    pub async fn into_one_shot(
        mut self,
    ) -> Result<Tmp1x2<I2C, mode::OneShot>, ModeChangeError<E, Self>> {
        if let Err(Error::I2C(e)) = self.config_one_shot().await {
            return Err(ModeChangeError::I2C(e, self));
        }
        Ok(Tmp1x2 {
            i2c: self.i2c,
            address: self.address,
            config: self.config,
            a_temperature_conversion_was_started: false,
            _mode: PhantomData,
        })
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
    /// Change into continuous conversion mode.
    ///
    /// If the mode change failed you will get a `ModeChangeError`.
    /// You can get the unchanged device back from it.
    pub async fn into_continuous(
        mut self,
    ) -> Result<Tmp1x2<I2C, mode::Continuous>, ModeChangeError<E, Self>> {
        if let Err(Error::I2C(e)) = self.config_continuous().await {
            return Err(ModeChangeError::I2C(e, self));
        }
        Ok(Tmp1x2 {
            i2c: self.i2c,
            address: self.address,
            config: self.config,
            a_temperature_conversion_was_started: false,
            _mode: PhantomData,
        })
    }

    pub(crate) async fn trigger_one_shot_measurement(&mut self) -> Result<(), Error<E>> {
        // This bit is not stored
        self.write_register(Register::CONFIG, self.config.with_high_msb(BFH::ONE_SHOT))
            .await
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
    async fn config_continuous(&mut self) -> Result<(), Error<E>> {
        self.write_config(self.config.with_low_msb(BFH::SHUTDOWN))
            .await
    }

    async fn config_one_shot(&mut self) -> Result<(), Error<E>> {
        self.write_config(self.config.with_high_msb(BFH::SHUTDOWN))
            .await
    }

    /// Enable the extended measurement mode.
    ///
    /// This allows measurement of temperatures above 128°C.
    pub async fn enable_extended_mode(&mut self) -> Result<(), Error<E>> {
        self.write_config(self.config.with_high_lsb(BFL::EXTENDED_MODE))
            .await
    }

    /// Disable the extended measurement mode.
    ///
    /// This puts the device in normal measurement mode. It will not measure
    /// temperatures above 128°C.
    pub async fn disable_extended_mode(&mut self) -> Result<(), Error<E>> {
        self.write_config(self.config.with_low_lsb(BFL::EXTENDED_MODE))
            .await
    }

    /// Set the conversion rate when in continuous conversion mode.
    pub async fn set_conversion_rate(&mut self, rate: CR) -> Result<(), Error<E>> {
        let Config { lsb, msb } = self.config;
        match rate {
            CR::_0_25Hz => {
                self.write_config(Config {
                    lsb: lsb & !BFL::CONV_RATE1 & !BFL::CONV_RATE0,
                    msb,
                })
                .await
            }
            CR::_1Hz => {
                self.write_config(Config {
                    lsb: lsb & !BFL::CONV_RATE1 | BFL::CONV_RATE0,
                    msb,
                })
                .await
            }
            CR::_4Hz => {
                self.write_config(Config {
                    lsb: lsb | BFL::CONV_RATE1 & !BFL::CONV_RATE0,
                    msb,
                })
                .await
            }
            CR::_8Hz => {
                self.write_config(Config {
                    lsb: lsb | BFL::CONV_RATE1 | BFL::CONV_RATE0,
                    msb,
                })
                .await
            }
        }
    }

    /// Set the high temperature threshold.
    ///
    /// The value provided will be capped to be in the interval
    /// `[-128.0, 127.9375]` in normal mode and `[-256.0, 255.875]` in
    /// extended mode.
    pub async fn set_high_temperature_threshold(
        &mut self,
        temperature: f32,
    ) -> Result<(), Error<E>> {
        self.set_temperature_threshold(temperature, Register::T_HIGH)
            .await
    }

    /// Set the low temperature threshold.
    ///
    /// The value provided will be capped to be in the interval
    /// `[-128.0, 127.9375]` in normal mode and `[-256.0, 255.875]` in
    /// extended mode.
    pub async fn set_low_temperature_threshold(
        &mut self,
        temperature: f32,
    ) -> Result<(), Error<E>> {
        self.set_temperature_threshold(temperature, Register::T_LOW)
            .await
    }

    async fn set_temperature_threshold(
        &mut self,
        temperature: f32,
        register: u8,
    ) -> Result<(), Error<E>> {
        if (self.config.lsb & BFL::EXTENDED_MODE) != 0 {
            let (msb, lsb) = convert_temp_to_register_extended(temperature);
            self.write_register(register, RegisterU16 { lsb, msb })
                .await
        } else {
            let (msb, lsb) = convert_temp_to_register_normal(temperature);
            self.write_register(register, RegisterU16 { lsb, msb })
                .await
        }
    }

    /// Set the fault queue.
    ///
    /// Set the number of consecutive faults that will trigger an alert.
    pub async fn set_fault_queue(&mut self, fq: FaultQueue) -> Result<(), Error<E>> {
        let Config { lsb, msb } = self.config;
        match fq {
            FaultQueue::_1 => {
                self.write_config(Config {
                    lsb,
                    msb: msb & !BFH::FAULT_QUEUE1 & !BFH::FAULT_QUEUE0,
                })
                .await
            }
            FaultQueue::_2 => {
                self.write_config(Config {
                    lsb,
                    msb: msb & !BFH::FAULT_QUEUE1 | BFH::FAULT_QUEUE0,
                })
                .await
            }
            FaultQueue::_4 => {
                self.write_config(Config {
                    lsb,
                    msb: msb | BFH::FAULT_QUEUE1 & !BFH::FAULT_QUEUE0,
                })
                .await
            }
            FaultQueue::_6 => {
                self.write_config(Config {
                    lsb,
                    msb: msb | BFH::FAULT_QUEUE1 | BFH::FAULT_QUEUE0,
                })
                .await
            }
        }
    }

    /// Set the alert polarity.
    pub async fn set_alert_polarity(&mut self, polarity: AlertPolarity) -> Result<(), Error<E>> {
        match polarity {
            AlertPolarity::ActiveLow => {
                self.write_config(self.config.with_low_msb(BFH::ALERT_POLARITY))
                    .await
            }
            AlertPolarity::ActiveHigh => {
                self.write_config(self.config.with_high_msb(BFH::ALERT_POLARITY))
                    .await
            }
        }
    }

    /// Set the thermostat mode.
    pub async fn set_thermostat_mode(&mut self, mode: ThermostatMode) -> Result<(), Error<E>> {
        match mode {
            ThermostatMode::Comparator => {
                self.write_config(self.config.with_low_msb(BFH::THERMOSTAT))
                    .await
            }
            ThermostatMode::Interrupt => {
                self.write_config(self.config.with_high_msb(BFH::THERMOSTAT))
                    .await
            }
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
}
