// functions to convert temperature to and from register values

pub fn convert_temp_from_register(msb: u8, lsb: u8) -> f32 {
    let mut sign = (u16::from(msb & 0b1000_0000)) << 8;
    let extended_mode = (lsb & 1) != 0;
    if extended_mode {
        if sign != 0 {
            sign |= 0b1111_0000 << 8;
        }
        let msb = u16::from(msb & 0b0111_1111);
        let value = sign | (msb << 5) | u16::from(lsb >> 3);
        // the value is stored as two's complement
        f32::from(value as i16) * 0.0625
    } else {
        if sign != 0 {
            sign |= 0b1111_1000 << 8;
        }
        let msb = u16::from(msb & 0b0111_1111);
        let value = sign | (msb << 4) | u16::from(lsb >> 4);
        // the value is stored as two's complement
        f32::from(value as i16) * 0.0625
    }
}

pub fn convert_temp_to_register_normal(mut t: f32) -> (u8, u8) {
    if t > 127.9375 {
        t = 127.9375;
    }
    if t < -128.0 {
        t = -128.0
    }
    let value = t / 0.0625;
    let value = (value as i16) << 4;
    ((value >> 8) as u8, (value as u8 & 0b1111_0000))
}

pub fn convert_temp_to_register_extended(mut t: f32) -> (u8, u8) {
    if t > 255.875 {
        t = 255.875;
    }
    if t < -256.0 {
        t = -256.0
    }
    let value = t / 0.0625;
    let value = (value as i16) << 3;
    ((value >> 8) as u8, (value as u8 & 0b1111_1000))
}

#[cfg(test)]
mod tests {
    use super::{
        convert_temp_from_register as convert_from_reg,
        convert_temp_to_register_extended as convert_to_reg_ext,
        convert_temp_to_register_normal as convert_to_reg,
    };

    macro_rules! assert_near {
        ($left:expr, $right:expr) => {
            assert!(($left - $right) < core::f32::EPSILON && ($right - $left) < core::f32::EPSILON);
        };
    }

    #[test]
    fn assert_near_can_succeed() {
        assert_near!(1.0, 1.0);
    }

    #[test]
    #[should_panic]
    fn assert_near_can_fail() {
        assert_near!(1.0, 1.1);
    }

    #[test]
    fn can_convert_temperature_from_register_normal_mode() {
        assert_near!(127.9375, convert_from_reg(0b0111_1111, 0b1111_0000));
        assert_near!(100.0, convert_from_reg(0b0110_0100, 0b0000_0000));
        assert_near!(80.0, convert_from_reg(0b0101_0000, 0b0000_0000));
        assert_near!(75.0, convert_from_reg(0b0100_1011, 0b0000_0000));
        assert_near!(50.0, convert_from_reg(0b0011_0010, 0b0000_0000));
        assert_near!(25.0, convert_from_reg(0b0001_1001, 0b0000_0000));
        assert_near!(0.25, convert_from_reg(0b0000_0000, 0b0100_0000));
        assert_near!(0.0, convert_from_reg(0b0000_0000, 0b0000_0000));
        assert_near!(-0.25, convert_from_reg(0b1111_1111, 0b1100_0000));
        assert_near!(-25.0, convert_from_reg(0b1110_0111, 0b0000_0000));
        assert_near!(-55.0, convert_from_reg(0b1100_1001, 0b0000_0000));
        assert_near!(-64.0, convert_from_reg(0b1100_0000, 0b0000_0000));
        assert_near!(-128.0, convert_from_reg(0b1000_0000, 0b0000_0000));
    }

    #[test]
    fn can_convert_temperature_from_register_extended_mode() {
        assert_near!(255.875, convert_from_reg(0b0111_1111, 0b1111_0001));
        assert_near!(150.0, convert_from_reg(0b0100_1011, 0b0000_0001));
        assert_near!(128.0, convert_from_reg(0b0100_0000, 0b0000_0001));
        assert_near!(127.9375, convert_from_reg(0b0011_1111, 0b1111_1001));
        assert_near!(100.0, convert_from_reg(0b0011_0010, 0b0000_0001));
        assert_near!(80.0, convert_from_reg(0b0010_1000, 0b0000_0001));
        assert_near!(75.0, convert_from_reg(0b0010_0101, 0b1000_0001));
        assert_near!(50.0, convert_from_reg(0b0001_1001, 0b0000_0001));
        assert_near!(25.0, convert_from_reg(0b0000_1100, 0b1000_0001));
        assert_near!(0.25, convert_from_reg(0b0000_0000, 0b0010_0001));
        assert_near!(0.0, convert_from_reg(0b0000_0000, 0b0000_0001));
        assert_near!(-0.25, convert_from_reg(0b1111_1111, 0b1110_0001));
        assert_near!(-25.0, convert_from_reg(0b1111_0011, 0b1000_0001));
        assert_near!(-55.0, convert_from_reg(0b1110_0100, 0b1000_0001));
        assert_near!(-256.0, convert_from_reg(0b1000_0000, 0b0000_0001));
    }

    #[test]
    fn can_convert_temperature_to_register_normal_mode() {
        assert_eq!((0b0111_1111, 0b1111_0000), convert_to_reg(128.0));
        assert_eq!((0b0111_1111, 0b1111_0000), convert_to_reg(127.9375));
        assert_eq!((0b0110_0100, 0b0000_0000), convert_to_reg(100.0));
        assert_eq!((0b0101_0000, 0b0000_0000), convert_to_reg(80.0));
        assert_eq!((0b0100_1011, 0b0000_0000), convert_to_reg(75.0));
        assert_eq!((0b0011_0010, 0b0000_0000), convert_to_reg(50.0));
        assert_eq!((0b0001_1001, 0b0000_0000), convert_to_reg(25.0));
        assert_eq!((0b0000_0000, 0b0100_0000), convert_to_reg(0.25));
        assert_eq!((0b0000_0000, 0b0000_0000), convert_to_reg(0.0));
        assert_eq!((0b1111_1111, 0b1100_0000), convert_to_reg(-0.25));
        assert_eq!((0b1110_0111, 0b0000_0000), convert_to_reg(-25.0));
        assert_eq!((0b1100_1001, 0b0000_0000), convert_to_reg(-55.0));
        assert_eq!((0b1000_0000, 0b0000_0000), convert_to_reg(-128.0));
        assert_eq!((0b1000_0000, 0b0000_0000), convert_to_reg(-129.0));
    }

    #[test]
    fn can_convert_temperature_to_register_extended_mode() {
        assert_eq!((0b0111_1111, 0b1111_0000), convert_to_reg_ext(255.875));
        assert_eq!((0b0100_1011, 0b0000_0000), convert_to_reg_ext(150.0));
        assert_eq!((0b0100_0000, 0b0000_0000), convert_to_reg_ext(128.0));
        assert_eq!((0b0011_1111, 0b1111_1000), convert_to_reg_ext(127.9375));
        assert_eq!((0b0011_0010, 0b0000_0000), convert_to_reg_ext(100.0));
        assert_eq!((0b0010_1000, 0b0000_0000), convert_to_reg_ext(80.0));
        assert_eq!((0b0010_0101, 0b1000_0000), convert_to_reg_ext(75.0));
        assert_eq!((0b0001_1001, 0b0000_0000), convert_to_reg_ext(50.0));
        assert_eq!((0b0000_1100, 0b1000_0000), convert_to_reg_ext(25.0));
        assert_eq!((0b0000_0000, 0b0010_0000), convert_to_reg_ext(0.25));
        assert_eq!((0b0000_0000, 0b0000_0000), convert_to_reg_ext(0.0));
        assert_eq!((0b1111_1111, 0b1110_0000), convert_to_reg_ext(-0.25));
        assert_eq!((0b1111_0011, 0b1000_0000), convert_to_reg_ext(-25.0));
        assert_eq!((0b1110_0100, 0b1000_0000), convert_to_reg_ext(-55.0));
        assert_eq!((0b1000_0000, 0b0000_0000), convert_to_reg_ext(-256.0));
        assert_eq!((0b1000_0000, 0b0000_0000), convert_to_reg_ext(-257.0));
    }
}
