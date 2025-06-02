use crate::RegisterU16;
use crate::{Config, Error, Register, Tmp1x2};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E, MODE> Tmp1x2<I2C, MODE>
where
    I2C: AsyncI2c<Error = E>,
{
    pub(crate) async fn write_config(&mut self, data: Config) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, data.clone()).await?;
        self.config = data;
        Ok(())
    }

    pub(crate) async fn write_register(
        &mut self,
        register: u8,
        data: RegisterU16,
    ) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, data.msb, data.lsb])
            .await
            .map_err(Error::I2C)
    }

    pub(crate) async fn read_register_u16(
        &mut self,
        register: u8,
    ) -> Result<RegisterU16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .await
            .map_err(Error::I2C)?;
        Ok(RegisterU16 {
            msb: data[0],
            lsb: data[1],
        })
    }
}
