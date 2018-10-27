extern crate tmp1x2;
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Transaction as I2cTransaction };
use tmp1x2::ConversionRate as CR;

mod common;
use common::{ DEVICE_ADDRESS, setup, Register, BitFlagsLow as BFL,
              BitFlagsHigh as BFH, DEFAULT_CONFIG_LSB as DEFAULT_LSB,
              DEFAULT_CONFIG_MSB as DEFAULT_MSB };


fn get_write_expectation(register: u8, lsb: u8, msb: u8) -> [I2cTransaction; 1] {
    [
        I2cTransaction::write(DEVICE_ADDRESS, vec![register, msb, lsb])
    ]
}

macro_rules! config_test {
    ($name:ident, $method:ident, $expected_lsb:expr, $expected_msb:expr) => {
        #[test]
        fn $name() {
            let expectations = get_write_expectation(Register::CONFIG, $expected_lsb, $expected_msb);
            let mut dev = setup(&expectations);
            dev.$method().unwrap();
            dev.destroy().done();
        }
    };
}

config_test!(can_enable,  enable,  DEFAULT_LSB,     DEFAULT_MSB);
config_test!(can_disable, disable, DEFAULT_LSB | 1, DEFAULT_MSB);

config_test!(can_enable_extended_mode,  enable_extended_mode,  DEFAULT_LSB, DEFAULT_MSB | BFH::EXTENDED_MODE);
config_test!(can_disable_extended_mode, disable_extended_mode, DEFAULT_LSB, DEFAULT_MSB);

config_test!(can_trigger_one_shot_measurement, trigger_one_shot_measurement, DEFAULT_LSB | BFL::ONE_SHOT, DEFAULT_MSB);

macro_rules! config_value_test {
    ($name:ident, $method:ident, $value:expr, $expected_lsb:expr, $expected_msb:expr) => {
        #[test]
        fn $name() {
            let expectations = get_write_expectation(Register::CONFIG,
                                                     $expected_lsb,
                                                     $expected_msb);
            let mut dev = setup(&expectations);
            dev.$method($value).unwrap();
            dev.destroy().done();
        }
    };
}

config_value_test!(can_set_cr_0_25, set_conversion_rate, CR::_0_25Hz, DEFAULT_LSB, DEFAULT_MSB & !BFH::CONV_RATE1 & !BFH::CONV_RATE0);
config_value_test!(can_set_cr_1,    set_conversion_rate, CR::_1Hz,    DEFAULT_LSB, DEFAULT_MSB & !BFH::CONV_RATE1 |  BFH::CONV_RATE0);
config_value_test!(can_set_cr_4,    set_conversion_rate, CR::_4Hz,    DEFAULT_LSB, DEFAULT_MSB |  BFH::CONV_RATE1 & !BFH::CONV_RATE0);
config_value_test!(can_set_cr_8,    set_conversion_rate, CR::_8Hz,    DEFAULT_LSB, DEFAULT_MSB |  BFH::CONV_RATE1 |  BFH::CONV_RATE0);