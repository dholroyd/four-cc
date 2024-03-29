# four-cc Change Log

## Unreleased

## 0.4.0 - 2024-03-16

### Changed
 - Updated to Rust 2021 edition

### Fixed
 - The `Display` implementation now escapes control characters as intended.  Previously, control characters that are
   valid in unicode were left unchanged, including things like newline (`\n`).

## 0.3.0 - 2023-05-31

### Added

 - `impl const From<FourCC> for u32` if `nightly` feature enabled.
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
