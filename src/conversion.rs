#![deny(unsafe_code)]

pub fn convert_temp_from_register(msb: u8, lsb: u8) -> f32 {
    let mut sign = ((msb & 0b1000_0000) as u16) << 8;
    let extended_mode = (lsb & 1) != 0;
    if extended_mode {
        if sign != 0 {
            sign |= 0b1111_0000 << 8;
        }
        let msb = (msb & 0b0111_1111) as u16;
        let value = sign | (msb << 5) | (lsb >> 3) as u16;
        // the value is stored as two's complement
        (value as i16) as f32 * 0.0625
    }
    else {
        if sign != 0 {
            sign |= 0b1111_1000 << 8;
        }
        let msb = (msb & 0b0111_1111) as u16;
        let value = sign | (msb << 4) | (lsb >> 4) as u16;
        // the value is stored as two's complement
        (value as i16) as f32 * 0.0625
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_temperature_from_register_normal_mode() {
        assert_eq!(127.9375, convert_temp_from_register(0b0111_1111, 0b1111_0000));
        assert_eq!(100.0,    convert_temp_from_register(0b0110_0100, 0b0000_0000));
        assert_eq!( 80.0,    convert_temp_from_register(0b0101_0000, 0b0000_0000));
        assert_eq!( 75.0,    convert_temp_from_register(0b0100_1011, 0b0000_0000));
        assert_eq!( 50.0,    convert_temp_from_register(0b0011_0010, 0b0000_0000));
        assert_eq!( 25.0,    convert_temp_from_register(0b0001_1001, 0b0000_0000));
        assert_eq!(  0.25,   convert_temp_from_register(0b0000_0000, 0b0100_0000));
        assert_eq!(  0.0,    convert_temp_from_register(0b0000_0000, 0b0000_0000));
        assert_eq!( -0.25,   convert_temp_from_register(0b1111_1111, 0b1100_0000));
        assert_eq!(-25.0,    convert_temp_from_register(0b1110_0111, 0b0000_0000));
        assert_eq!(-55.0,    convert_temp_from_register(0b1100_1001, 0b0000_0000));
    }

    #[test]
    fn can_convert_temperature_from_register_extended_mode() {
        assert_eq!(150.0,    convert_temp_from_register(0b0100_1011, 0b0000_0001));
        assert_eq!(128.0,    convert_temp_from_register(0b0100_0000, 0b0000_0001));
        assert_eq!(127.9375, convert_temp_from_register(0b0011_1111, 0b1111_1001));
        assert_eq!(100.0,    convert_temp_from_register(0b0011_0010, 0b0000_0001));
        assert_eq!( 80.0,    convert_temp_from_register(0b0010_1000, 0b0000_0001));
        assert_eq!( 75.0,    convert_temp_from_register(0b0010_0101, 0b1000_0001));
        assert_eq!( 50.0,    convert_temp_from_register(0b0001_1001, 0b0000_0001));
        assert_eq!( 25.0,    convert_temp_from_register(0b0000_1100, 0b1000_0001));
        assert_eq!(  0.25,   convert_temp_from_register(0b0000_0000, 0b0010_0001));
        assert_eq!(  0.0,    convert_temp_from_register(0b0000_0000, 0b0000_0001));
        assert_eq!( -0.25,   convert_temp_from_register(0b1111_1111, 0b1110_0001));
        assert_eq!(-25.0,    convert_temp_from_register(0b1111_0011, 0b1000_0001));
        assert_eq!(-55.0,    convert_temp_from_register(0b1110_0100, 0b1000_0001));
    }
}

