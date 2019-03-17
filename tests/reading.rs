extern crate embedded_hal_mock as hal;
extern crate tmp1x2;
use hal::i2c::Transaction as I2cTransaction;

mod common;
use common::{
    setup, BitFlagsHigh as BFH, BitFlagsLow as BFL, Register, DEFAULT_CONFIG_LSB,
    DEFAULT_CONFIG_MSB, DEVICE_ADDRESS,
};

fn get_expectation(register: u8, lsb: u8, msb: u8) -> [I2cTransaction; 1] {
    [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![register],
        vec![msb, lsb],
    )]
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

read_test!(
    comp_alert_not_active,
    is_comparator_mode_alert_active,
    CONFIG,
    DEFAULT_CONFIG_LSB,
    DEFAULT_CONFIG_MSB | BFH::ALERT,
    false
);
read_test!(
    comp_alert_active,
    is_comparator_mode_alert_active,
    CONFIG,
    DEFAULT_CONFIG_LSB,
    DEFAULT_CONFIG_MSB & !BFH::ALERT,
    true
);

read_test!(
    comp_alert_not_active_high_pol,
    is_comparator_mode_alert_active,
    CONFIG,
    DEFAULT_CONFIG_LSB | BFL::ALERT_POLARITY,
    DEFAULT_CONFIG_MSB & !BFH::ALERT,
    false
);
read_test!(
    comp_alert_active_high_pol,
    is_comparator_mode_alert_active,
    CONFIG,
    DEFAULT_CONFIG_LSB | BFL::ALERT_POLARITY,
    DEFAULT_CONFIG_MSB | BFH::ALERT,
    true
);

macro_rules! assert_near {
    ($left:expr, $right:expr) => {
        assert!(($left - $right) < core::f32::EPSILON && ($right - $left) < core::f32::EPSILON);
    };
}

#[test]
fn in_one_shot_read_temperature_triggers_measurement() {
    let expectations = [
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                Register::CONFIG,
                DEFAULT_CONFIG_MSB,
                DEFAULT_CONFIG_LSB | BFL::SHUTDOWN,
            ],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                Register::CONFIG,
                DEFAULT_CONFIG_MSB,
                DEFAULT_CONFIG_LSB | BFL::ONE_SHOT | BFL::SHUTDOWN,
            ],
        ),
    ];
    let dev = setup(&expectations);
    let mut dev = dev.into_one_shot().unwrap();
    match dev.read_temperature() {
        Err(nb::Error::WouldBlock) => (),
        _ => panic!(),
    }
    dev.destroy().done();
}

#[test]
fn in_one_shot_read_temperature_returns_would_block_if_not_ready() {
    let expectations = [
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                Register::CONFIG,
                DEFAULT_CONFIG_MSB,
                DEFAULT_CONFIG_LSB | BFL::SHUTDOWN,
            ],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                Register::CONFIG,
                DEFAULT_CONFIG_MSB,
                DEFAULT_CONFIG_LSB | BFL::ONE_SHOT | BFL::SHUTDOWN,
            ],
        ),
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![Register::CONFIG],
            vec![DEFAULT_CONFIG_MSB, DEFAULT_CONFIG_LSB],
        ),
    ];
    let dev = setup(&expectations);
    let mut dev = dev.into_one_shot().unwrap();
    dev.read_temperature().expect_err("Should return an error");
    match dev.read_temperature() {
        Err(nb::Error::WouldBlock) => (),
        _ => panic!(),
    }
    dev.destroy().done();
}

#[test]
fn in_one_shot_can_read_temperature() {
    let expectations = [
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                Register::CONFIG,
                DEFAULT_CONFIG_MSB,
                DEFAULT_CONFIG_LSB | BFL::SHUTDOWN,
            ],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                Register::CONFIG,
                DEFAULT_CONFIG_MSB,
                DEFAULT_CONFIG_LSB | BFL::ONE_SHOT | BFL::SHUTDOWN,
            ],
        ),
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![Register::CONFIG],
            vec![DEFAULT_CONFIG_MSB, DEFAULT_CONFIG_LSB | BFL::ONE_SHOT],
        ),
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![Register::TEMPERATURE],
            vec![0b0110_0100, 0],
        ),
    ];
    let dev = setup(&expectations);
    let mut dev = dev.into_one_shot().unwrap();
    dev.read_temperature().expect_err("Should return an error");
    let temp = dev.read_temperature().unwrap();
    assert_near!(100.0, temp);
    dev.destroy().done();
}

#[test]
fn in_continuous_can_read_temperature() {
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![Register::TEMPERATURE],
        vec![0b0110_0100, 0],
    )];
    let mut dev = setup(&expectations);
    let value = dev.read_temperature().unwrap();
    assert_near!(100.0, value);
    dev.destroy().done();
}
