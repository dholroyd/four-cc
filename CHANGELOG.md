# four-cc Change Log

## Unreleased

### Fixed

 - Now supports usage in `no_std` environments.

## 0.2.0 - 2022-05-22

### Changed

 - Representation changed to [repr(C)](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc) +
   [repr(packed)](https://doc.rust-lang.org/nomicon/other-reprs.html#reprpacked).

### Added

- Added optional support for
  [schemars](https://docs.rs/schemars/latest/schemars/),
  [serde](https://docs.rs/serde/latest/serde/) and
  [zerocopy](https://docs.rs/zerocopy/latest/zerocopy/).  Thanks [@daym](https://github.com/daym)!

## 0.1.0

 - Initial release
