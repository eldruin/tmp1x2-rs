# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.2.0] - 2019-03-17

### Changed
- [breaking-change] The modes are now modeled through a marker type. Per default
  the devices are in continuous mode. In this mode the `read_temperature()` behaves
  as previously. However, now it is possible to change the mode to one-shot.
  In this mode the `read_temperature()` method offers a non-blocking interface.
  It returns `nb::Error::WouldBlock` if the result is not ready.
  Because of this new interface, the following methods have been removed:
  - `enable()`: Now modeled through `into_continuous()`.
  - `disable()`: Now modeled through `into_one_shot()`.
  - `trigger_one_shot_measurement()`: Rendered unnecessary.
  - `is_one_shot_measurement_result_ready()`: Rendered unnecessary.

## 0.1.0 - 2018-10-27

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future. All changes will be documented in this
CHANGELOG.

[Unreleased]: https://github.com/eldruin/tmp1x2-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/tmp1x2-rs/compare/v0.1.0...v0.2.0
