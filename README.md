# Rust TMP102 and TMP112 Temperature Sensor Driver

[![crates.io](https://img.shields.io/crates/v/tmp1x2.svg)](https://crates.io/crates/tmp1x2)
[![Docs](https://docs.rs/tmp1x2/badge.svg)](https://docs.rs/tmp1x2)
[![Build Status](https://travis-ci.org/eldruin/tmp1x2-rs.svg?branch=master)](https://travis-ci.org/eldruin/tmp1x2-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/tmp1x2-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/tmp1x2-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the TMP102 and TMP112
high-accuracy, low-power, digital temperature sensors, based on the
[`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Change into one-shot or continuous conversion mode.
- Read the temperature.
- Enable/disable the extended measurement mode.
- Trigger a one-shot measurement.
- Read whether the one-shot measurement result is ready.
- Set the conversion rate.
- Set the high/low temperature threshold.
- Set the fault queue.
- Set the alert polarity.
- Set the thermostat mode.
- Read whether a comparator mode alert is active.

## The devices

This driver is compatible with both the TMP102 device as well as the TMP112
family of devices, including TMP112A, TMP112B and TMP112N.

The TMP102 device is a digital temperature sensor ideal for NTC/PTC
thermistor replacement where high accuracy is required. The device offers
an accuracy of +/-0.5°C without requiring calibration or external component
signal conditioning. Device temperature sensors are highly linear and do
not require complex calculations or lookup tables to derive the
temperature. The on-chip 12-bit ADC offers resolutions down to 0.0625°C.

The TMP112 family of devices are digital temperature sensors designed for
high-accuracy, low-power, NTC/PTC thermistor replacements where high
accuracy is required. The TMP112A and TMP112B offers 0.5°C accuracy and are
optimized to provide the best PSR performance for 3.3V and 1.8V operation
respectively, while TMP112N offers 1°C accuracy. These temperature sensors
are highly linear and do not require complex calculations or lookup tables
to derive the temperature. The on-chip 12-bit ADC offers resolutions down
to 0.0625°C.

The devices feature SMBus(TM), two-wire and I2C interface
compatibility, and allows up to four devices on one bus. The device also
features an SMBus alert function. The devices are specified to operate over
supply voltages from 1.4 to 3.6 V with the maximum quiescent current of
10 μA over the full operating range.

The devices are designed for extended temperature measurement in
communication, computer, consumer, environmental, industrial, and
instrumentation applications. The devices are specified for operation over a
temperature range of -40°C to +125°C.

Datasheets:
- [TMP102](http://www.ti.com/lit/ds/symlink/tmp102.pdf)
- [TMP112x](http://www.ti.com/lit/ds/symlink/tmp112.pdf)

### Usage

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate linux_embedded_hal as hal;
extern crate tmp1x2;

use tmp1x2::{Tmp1x2, SlaveAddr};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Tmp1x2::new(dev, address);
    let temperature = sensor.read_temperature().unwrap();
    println!("Temperature: {:.1}ºC", temperature);
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/tmp1x2-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

