//! Newtype wrapper providing a convenient representation of _four-character-code_ values.
//!
//! Using this type in a public APIs as an alternative to simply passing the equivalent `u32`
//! makes the value's expected use explicit.
//!
//!
//! ## Creating a FourCC value
//!
//! ```rust
//! use four_cc::FourCC;
//!
//! let uuid = FourCC(*b"uuid");
//! ```
//!
//! ## From a slice
//!
//! ```rust
//! # use four_cc::FourCC;
//! let data = b"moofftyp";
//! let code = FourCC::from(&data[0..4]);  // would panic if fewer than 4 bytes
//! assert_eq!(FourCC(*b"moof"), code);
//! ```
//!
//! ## Constants
//!
//! FourCC values can be used in const expressions
//!
//! ```rust
//! # use four_cc::FourCC;
//! const UUID: FourCC = FourCC(*b"uuid");
//! ```
//!
//! ## Matching
//!
//! You can use FourCC values in match patterns as long as you define constants to match against,
//!
//! ```rust
//! # use four_cc::FourCC;
//! const UUID: FourCC = FourCC(*b"uuid");
//! const MOOV: FourCC = FourCC(*b"moov");
//! # let other_value = UUID;
//! match other_value {
//!     MOOV => println!("movie"),
//!     UUID => println!("unique identifier"),
//!     // compiler will not accept: FourCC(*b"trun") => println!("track fragment run"),
//!     _ => println!("Other value; scary stuff")
//! }
//! ```
//!
//! ## Invalid literal values
//!
//! If the literal has other than four bytes, compilation will fail
//!
//! ```compile_fail
//! # use four_cc::FourCC;
//! let bad_fourcc = FourCC(*b"uuid123");
//! // -> expected an array with a fixed size of 4 elements, found one with 7 elements
//! ```
//! **Note** the FourCC value _may_ contain non-printable byte values, including the byte-value zero.
//!
//! ## Debug display
//!
//! ```rust
//! # use four_cc::FourCC;
//! # use std::fmt::Debug;
//! let uuid = FourCC(*b"uuid");
//! # assert_eq!("FourCC{uuid}", format!("{:?}", &uuid));
//! println!("it's {:?}", uuid);  // produces: it's FourCC{uuid}
//! ```
//!
//! Note that if the FourCC bytes are not able to be converted to UTF8, then a fallback
//! representation will be used (as it would be suprising for `format!()` to panic).
//!
//! ```rust
//! # use four_cc::FourCC;
//! # use std::fmt::Debug;
//! let uuid = FourCC(*b"u\xFFi\0");
//! # assert_eq!("FourCC{u\\xffi\\x00}", format!("{:?}", &uuid));
//! println!("it's {:?}", uuid);  // produces: it's FourCC{u\xffi\x00}
//! ```

use std::fmt;

/// A _four-character-code_ value.
///
/// See the [module level documentation](index.html).
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct FourCC (
    pub [u8; 4]
);
impl<'a> From<&'a[u8; 4]> for FourCC {
    fn from(buf: &[u8; 4]) -> FourCC {
        FourCC([buf[0], buf[1], buf[2], buf[3]])
    }
}
impl<'a> From<&'a[u8]> for FourCC {
    fn from(buf: &[u8]) -> FourCC {
        FourCC([buf[0], buf[1], buf[2], buf[3]])
    }
}
impl From<u32> for FourCC {
    fn from(val: u32) -> FourCC {
        FourCC([
            (val >> 24 & 0xff) as u8,
            (val >> 16 & 0xff) as u8,
            (val >> 8  & 0xff) as u8,
            (val       & 0xff) as u8
        ])
    }
}
impl From<FourCC> for u32 {
    fn from(val: FourCC) -> Self {
        ((val.0[0] as u32) << 24 & 0xff000000) |
        ((val.0[1] as u32) << 16 & 0x00ff0000) |
        ((val.0[2] as u32) << 8  & 0x0000ff00) |
        ((val.0[3] as u32)       & 0x000000ff)
    }
}
impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match std::str::from_utf8(&self.0) {
            Ok(s) => f.write_str(s),
            Err(_) => {
                // If we return fmt::Error, then for example format!() will panic, so we choose
                // an alternative representation instead
                let s = &self.0
                    .iter()
                    .flat_map(|b| std::ascii::escape_default(*b) )
                    .collect::<Vec<u8>>()[..];
                f.write_str(&String::from_utf8_lossy(s))
            },
        }
    }
}
impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("FourCC{")?;
        fmt::Display::fmt(self, f)?;
        f.write_str("}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        assert_eq!(FourCC(*b"uuid"), b"uuid".into());
        assert_ne!(FourCC(*b"uuid"), b"diuu".into());
    }

    #[test]
    fn int_conversions() {
        assert_eq!(0x41424344u32, FourCC(*b"ABCD").into());
        assert_eq!(FourCC(*b"ABCD"), 0x41424344u32.into());
    }
}
