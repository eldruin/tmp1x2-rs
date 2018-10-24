# Rust TMP102 and TMP112 Temperature Sensor Driver [![crates.io](https://img.shields.io/crates/v/tmp1x2.svg)](https://crates.io/crates/tmp1x2) [![Docs](https://docs.rs/tmp1x2/badge.svg)](https://docs.rs/tmp1x2) [![Build Status](https://travis-ci.org/eldruin/tmp1x2-rs.svg?branch=master)](https://travis-ci.org/eldruin/tmp1x2-rs)

This is a platform agnostic Rust driver for the TMP102 and TMP112
high-accuracy, low-power, digital temperature sensors, based on the
[`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the device.
- Read the temperature.

## The devices

Datasheets:
- [TMP102](http://www.ti.com/lit/ds/symlink/tmp102.pdf)
- [TMP112x](http://www.ti.com/lit/ds/symlink/tmp112.pdf)

This driver is also compatible with the TMP112 family of devices, including
TMP112A, TMP112B and TMP112N.

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

