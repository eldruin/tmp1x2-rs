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

#[test]
fn can_read_temperature() {
    let expectations = get_expectation(Register::TEMPERATURE, 0, 0b0110_0100);
    let mut dev = setup(&expectations);
    let temp = dev.read_temperature().unwrap();
    assert_eq!(100.0, temp);
    dev.destroy().done();
}

#[test]
fn can_one_shot_measurement_is_not_ready() {
    let expectations = get_expectation(Register::CONFIG, DEFAULT_CONFIG_LSB, DEFAULT_CONFIG_MSB);
    let mut dev = setup(&expectations);
    let is_ready = dev.is_one_shot_measurement_result_ready().unwrap();
    assert!(!is_ready);
    dev.destroy().done();
}

#[test]
fn can_one_shot_measurement_is_ready() {
    let expectations = get_expectation(Register::CONFIG, DEFAULT_CONFIG_LSB | BFL::ONE_SHOT, DEFAULT_CONFIG_MSB);
    let mut dev = setup(&expectations);
    let is_ready = dev.is_one_shot_measurement_result_ready().unwrap();
    assert!(is_ready);
    dev.destroy().done();
}
