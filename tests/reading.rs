extern crate tmp1x2;
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Transaction as I2cTransaction };

mod common;
use common::{ DEVICE_ADDRESS, setup, Register };

fn get_expectation(register: u8) -> [I2cTransaction; 1] {
    [
        I2cTransaction::write_read(DEVICE_ADDRESS, vec![register], vec![0b0110_0100, 0])
    ]
}

#[test]
fn can_read_temperature() {
    let expectations = get_expectation(Register::TEMPERATURE);
    let mut dev = setup(&expectations);
    let temp = dev.read_temperature().unwrap();
    assert_eq!(100.0, temp);
    dev.destroy().done();
}
