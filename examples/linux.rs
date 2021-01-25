use linux_embedded_hal::I2cdev;
use tmp1x2::{SlaveAddr, Tmp1x2};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Tmp1x2::new(dev, address);
    let temperature = sensor.read_temperature().unwrap();
    println!("Temperature: {:.1}ºC", temperature);
}
