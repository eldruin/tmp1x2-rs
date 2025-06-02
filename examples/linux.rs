#[cfg(not(feature = "async"))]
fn main() {
    let dev = linux_embedded_hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address = tmp1x2::SlaveAddr::default();
    let mut sensor = tmp1x2::Tmp1x2::new(dev, address);
    let temperature = sensor.read_temperature().unwrap();
    println!("Temperature: {:.1}ºC", temperature);
}

#[cfg(feature = "async")]
fn main() {
    panic!("Feature `async` must NOT be enabled to run this example.");
}
