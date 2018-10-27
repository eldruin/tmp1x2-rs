extern crate tmp1x2;
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Transaction as I2cTransaction };

mod common;
use common::{ DEVICE_ADDRESS, setup, Register, BitFlagsLow as BFL,
              DEFAULT_CONFIG_LSB, DEFAULT_CONFIG_MSB };


fn get_expectation(register: u8, lsb: u8, msb: u8) -> [I2cTransaction; 1] {
    [
        I2cTransaction::write_read(DEVICE_ADDRESS, vec![register], vec![msb, lsb])
    ]
}

macro_rules! read_test {
    ($name:ident, $method:ident, $register:ident, $lsb:expr, $msb:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let expectations = get_expectation(Register::$register, $lsb, $msb);
            let mut dev = setup(&expectations);
            let value = dev.$method().unwrap();
            assert_eq!($expected, value);
            dev.destroy().done();
        }
    };
}

read_test!(can_read_temperature, read_temperature, TEMPERATURE, 0, 0b0110_0100, 100.0);

read_test!(one_shot_result_not_ready, is_one_shot_measurement_result_ready, CONFIG,
           DEFAULT_CONFIG_LSB,                 DEFAULT_CONFIG_MSB, false);
read_test!(one_shot_result_ready,     is_one_shot_measurement_result_ready, CONFIG,
           DEFAULT_CONFIG_LSB | BFL::ONE_SHOT, DEFAULT_CONFIG_MSB, true);
