use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use tmp1x2::{marker, SlaveAddr, Tmp1x2};

pub const DEVICE_ADDRESS: u8 = 0b100_1000;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const TEMPERATURE: u8 = 0x00;
    pub const CONFIG: u8 = 0x01;
    pub const T_LOW: u8 = 0x02;
    pub const T_HIGH: u8 = 0x03;
}

pub struct BitFlagsHigh;

#[allow(unused)]
impl BitFlagsHigh {
    pub const SHUTDOWN: u8 = 0b0000_0001;
    pub const THERMOSTAT: u8 = 0b0000_0010;
    pub const ALERT_POLARITY: u8 = 0b0000_0100;
    pub const FAULT_QUEUE0: u8 = 0b0000_1000;
    pub const FAULT_QUEUE1: u8 = 0b0001_0000;
    pub const RESOLUTION: u8 = 0b0110_0000;
    pub const ONE_SHOT: u8 = 0b1000_0000;
}

pub struct BitFlagsLow;

#[allow(unused)]
impl BitFlagsLow {
    pub const EXTENDED_MODE: u8 = 0b0001_0000;
    pub const ALERT: u8 = 0b0010_0000;
    pub const CONV_RATE0: u8 = 0b0100_0000;
    pub const CONV_RATE1: u8 = 0b1000_0000;
}

pub const DEFAULT_CONFIG_MSB: u8 = BitFlagsHigh::RESOLUTION;
pub const DEFAULT_CONFIG_LSB: u8 = BitFlagsLow::CONV_RATE1 | BitFlagsLow::ALERT;

pub fn setup(expectations: &[I2cTransaction]) -> Tmp1x2<I2cMock, marker::mode::Continuous> {
    let i2c = I2cMock::new(expectations);
    Tmp1x2::new(i2c, SlaveAddr::default())
}
