use crate::RegisterU16;
use crate::{Config, Error, Register, Tmp1x2};
use embedded_hal::i2c;

impl<I2C, E, MODE> Tmp1x2<I2C, MODE>
where
    I2C: i2c::I2c<Error = E>,
{
    pub(crate) fn write_config(&mut self, data: Config) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, data.clone())?;
        self.config = data;
        Ok(())
    }

    pub(crate) fn write_register(
        &mut self,
        register: u8,
        data: RegisterU16,
    ) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, data.msb, data.lsb])
            .map_err(Error::I2C)
    }

    pub(crate) fn read_register_u16(&mut self, register: u8) -> Result<RegisterU16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)?;
        Ok(RegisterU16 {
            msb: data[0],
            lsb: data[1],
        })
    }
}
