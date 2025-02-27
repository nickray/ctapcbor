#![allow(unused_variables)]

use core::fmt::{Display, Formatter};
#[cfg(feature = "ufmt")]
use ufmt::derive::uDebug;

/// This is the Result type used by ctapcbor.
pub type Result<T> = core::result::Result<T, Error>;

/// This is the error type used by ctapcbor
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "ufmt", derive(uDebug))]
#[repr(u8)]
pub enum Error {
    /// This is a feature that ctapcbor will never implement
    WontImplement,
    /// This is a feature that ctapcbor intends to support, but does not yet
    NotYetImplemented,
    // /// The serialize buffer is full
    // SerializeBufferFull,
    // /// The length of a sequence must be known
    // SerializeSeqLengthUnknown,
    /// Hit the end of buffer, expected more data
    DeserializeUnexpectedEnd,
    // /// Found a varint that didn't terminate. Is the usize too big for this platform?
    // DeserializeBadVarint,
    /// Found a bool that wasn't 0xf4 or 0xf5
    DeserializeBadBool,
    // /// Found an invalid unicode char
    // DeserializeBadChar,
    /// Tried to parse invalid utf-8
    DeserializeBadUtf8,
    // /// Found an Option discriminant that wasn't 0 or 1
    // DeserializeBadOption,
    // /// Found an enum discriminant that was > u32::max_value()
    // DeserializeBadEnum,
    // /// The original data was not well encoded
    // DeserializeBadEncoding,
    /// Expected a different major type
    DeserializeBadMajor,
    /// Expected a i8, was too large
    DeserializeBadI8,
    /// Expected a i16, was too large
    DeserializeBadI16,
    /// Expected a i32, was too large
    DeserializeBadI32,
    /// Expected a u8
    DeserializeBadU8,
    /// Expected a u16
    DeserializeBadU16,
    /// Expected a u32
    DeserializeBadU32,
    /// Inexistent slice-to-array cast error. Used here to avoid calling unwrap.
    InexistentSliceToArrayError,
    /// Value may be valid, but not encoded in minimal way
    DeserializeNonMinimal,
    /// Serde Serialization Error
    SerdeSerCustom,
    /// Serde Deserialization Error
    SerdeDeCustom,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        use Error::*;
        write!(
            f,
            "{}",
            match self {
                WontImplement => "This is a feature that ctapcbor will never implement",
                NotYetImplemented => {
                    "This is a feature that ctapcbor intends to support, but does not yet"
                }
                // SerializeBufferFull => "The serialize buffer is full",
                // SerializeSeqLengthUnknown => "The length of a sequence must be known",
                DeserializeUnexpectedEnd => "Hit the end of buffer, expected more data",
                // DeserializeBadVarint => {
                //     "Found a varint that didn't terminate. Is the usize too big for this platform?"
                // }
                DeserializeBadBool => "Found a bool that wasn't 0xf4 or 0xf5",
                // DeserializeBadChar => "Found an invalid unicode char",
                DeserializeBadUtf8 => "Tried to parse invalid utf-8",
                // DeserializeBadOption => "Found an Option discriminant that wasn't 0 or 1",
                // DeserializeBadEnum => "Found an enum discriminant that was > u32::max_value()",
                // DeserializeBadEncoding => "The original data was not well encoded",
                DeserializeBadI8 => "Expected a i8",
                DeserializeBadI16 => "Expected a i16",
                DeserializeBadI32 => "Expected a i32",
                DeserializeBadMajor => "Expected a different major type",
                DeserializeBadU8 => "Expected a u8",
                DeserializeBadU16 => "Expected a u16",
                DeserializeBadU32 => "Expected a u32",
                InexistentSliceToArrayError => "",
                DeserializeNonMinimal => "Value may be valid, but not encoded in minimal way",
                SerdeSerCustom => "Serde Serialization Error",
                SerdeDeCustom => "Serde Deserialization Error",
            }
        )
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::SerdeSerCustom
    }
}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::SerdeDeCustom
    }
}

impl serde::ser::StdError for Error {}
