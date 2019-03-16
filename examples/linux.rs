extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate tmp1x2;

use linux_embedded_hal::I2cdev;
use tmp1x2::{SlaveAddr, Tmp1x2};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Tmp1x2::new(dev, SlaveAddr::default());
    let temperature = sensor.read_temperature().unwrap();
    println!("Temperature: {}", temperature);
}
