extern crate embedded_hal_mock as hal;
extern crate tmp1x2;
use hal::i2c::Transaction as I2cTransaction;
use tmp1x2::{AlertPolarity as AP, ConversionRate as CR, FaultQueue as FQ, ThermostatMode as TM};

mod common;
use common::{
    setup, BitFlagsHigh as BFH, BitFlagsLow as BFL, Register, DEFAULT_CONFIG_LSB as DEFAULT_LSB,
    DEFAULT_CONFIG_MSB as DEFAULT_MSB, DEVICE_ADDRESS,
};

fn get_write_expectation(register: u8, lsb: u8, msb: u8) -> [I2cTransaction; 1] {
    [I2cTransaction::write(
        DEVICE_ADDRESS,
        vec![register, msb, lsb],
    )]
}

macro_rules! config_test {
    ($name:ident, $method:ident, $expected_lsb:expr, $expected_msb:expr) => {
        #[test]
        fn $name() {
            let expectations =
                get_write_expectation(Register::CONFIG, $expected_lsb, $expected_msb);
            let mut dev = setup(&expectations);
            dev.$method().unwrap();
            dev.destroy().done();
        }
    };
}

#[test]
fn can_change_into_one_shot() {
    let expectations = [I2cTransaction::write(
        DEVICE_ADDRESS,
        vec![Register::CONFIG, DEFAULT_MSB, DEFAULT_LSB | 1],
    )];
    let dev = setup(&expectations);
    let dev = dev.into_one_shot().unwrap();
    dev.destroy().done();
}

#[test]
fn can_change_into_continuous() {
    let expectations = [
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![Register::CONFIG, DEFAULT_MSB, DEFAULT_LSB | 1],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![Register::CONFIG, DEFAULT_MSB, DEFAULT_LSB],
        ),
    ];
    let dev = setup(&expectations);
    let dev = dev.into_one_shot().unwrap();
    let dev = dev.into_continuous().unwrap();
    dev.destroy().done();
}

config_test!(
    can_enable_extended_mode,
    enable_extended_mode,
    DEFAULT_LSB,
    DEFAULT_MSB | BFH::EXTENDED_MODE
);
config_test!(
    can_disable_extended_mode,
    disable_extended_mode,
    DEFAULT_LSB,
    DEFAULT_MSB
);

macro_rules! config_value_test {
    ($name:ident, $method:ident, $value:expr, $expected_lsb:expr, $expected_msb:expr) => {
        #[test]
        fn $name() {
            let expectations =
                get_write_expectation(Register::CONFIG, $expected_lsb, $expected_msb);
            let mut dev = setup(&expectations);
            dev.$method($value).unwrap();
            dev.destroy().done();
        }
    };
}

config_value_test!(
    can_set_cr_0_25,
    set_conversion_rate,
    CR::_0_25Hz,
    DEFAULT_LSB,
    DEFAULT_MSB & !BFH::CONV_RATE1 & !BFH::CONV_RATE0
);
config_value_test!(
    can_set_cr_1,
    set_conversion_rate,
    CR::_1Hz,
    DEFAULT_LSB,
    DEFAULT_MSB & !BFH::CONV_RATE1 | BFH::CONV_RATE0
);
config_value_test!(
    can_set_cr_4,
    set_conversion_rate,
    CR::_4Hz,
    DEFAULT_LSB,
    DEFAULT_MSB | BFH::CONV_RATE1 & !BFH::CONV_RATE0
);
config_value_test!(
    can_set_cr_8,
    set_conversion_rate,
    CR::_8Hz,
    DEFAULT_LSB,
    DEFAULT_MSB | BFH::CONV_RATE1 | BFH::CONV_RATE0
);

config_value_test!(
    can_set_fq_1,
    set_fault_queue,
    FQ::_1,
    DEFAULT_LSB & !BFL::FAULT_QUEUE1 & !BFL::FAULT_QUEUE0,
    DEFAULT_MSB
);
config_value_test!(
    can_set_fq_2,
    set_fault_queue,
    FQ::_2,
    DEFAULT_LSB & !BFL::FAULT_QUEUE1 | BFL::FAULT_QUEUE0,
    DEFAULT_MSB
);
config_value_test!(
    can_set_fq_4,
    set_fault_queue,
    FQ::_4,
    DEFAULT_LSB | BFL::FAULT_QUEUE1 & !BFL::FAULT_QUEUE0,
    DEFAULT_MSB
);
config_value_test!(
    can_set_fq_6,
    set_fault_queue,
    FQ::_6,
    DEFAULT_LSB | BFL::FAULT_QUEUE1 | BFL::FAULT_QUEUE0,
    DEFAULT_MSB
);

config_value_test!(
    can_set_ap_low,
    set_alert_polarity,
    AP::ActiveLow,
    DEFAULT_LSB & !BFL::ALERT_POLARITY,
    DEFAULT_MSB
);
config_value_test!(
    can_set_ap_high,
    set_alert_polarity,
    AP::ActiveHigh,
    DEFAULT_LSB | BFL::ALERT_POLARITY,
    DEFAULT_MSB
);

config_value_test!(
    can_set_tm_comp,
    set_thermostat_mode,
    TM::Comparator,
    DEFAULT_LSB & !BFL::THERMOSTAT,
    DEFAULT_MSB
);
config_value_test!(
    can_set_tm_int,
    set_thermostat_mode,
    TM::Interrupt,
    DEFAULT_LSB | BFL::THERMOSTAT,
    DEFAULT_MSB
);

macro_rules! set_value_test {
    ($name:ident, $method:ident, $value:expr, $register:expr, $expected_lsb:expr, $expected_msb:expr) => {
        #[test]
        fn $name() {
            let expectations = get_write_expectation($register, $expected_lsb, $expected_msb);
            let mut dev = setup(&expectations);
            dev.$method($value).unwrap();
            dev.destroy().done();
        }
    };
}

set_value_test!(
    can_set_high_temp_th_m0_25,
    set_high_temperature_threshold,
    -0.25,
    Register::T_HIGH,
    0b1100_0000,
    0b1111_1111
);
set_value_test!(
    can_set_high_temp_th_127,
    set_high_temperature_threshold,
    127.9375,
    Register::T_HIGH,
    0b1111_0000,
    0b0111_1111
);

set_value_test!(
    can_set_low_temp_th_m0_25,
    set_low_temperature_threshold,
    -0.25,
    Register::T_LOW,
    0b1100_0000,
    0b1111_1111
);
set_value_test!(
    can_set_low_temp_th_127,
    set_low_temperature_threshold,
    127.9375,
    Register::T_LOW,
    0b1111_0000,
    0b0111_1111
);

#[test]
fn can_set_extended_high_temp_threshold() {
    let expectations = [
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                Register::CONFIG,
                DEFAULT_MSB | BFH::EXTENDED_MODE,
                DEFAULT_LSB,
            ],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![Register::T_HIGH, 0b0111_1111, 0b1111_0000],
        ),
    ];
    let mut dev = setup(&expectations);
    dev.enable_extended_mode().unwrap();
    dev.set_high_temperature_threshold(255.875).unwrap();
    dev.destroy().done();
}
