// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaturatingU16{
    value: u16
}

impl From<u8> for SaturatingU16{
    fn from(value: u8) -> Self {
        Self {
            value: value as u16
        }
    }
}

impl From<&u8> for SaturatingU16{
    fn from(value: &u8) -> Self {
        Self {
            value: *value as u16
        }
    }
}

impl From<&u16> for SaturatingU16{
    fn from(value: &u16) -> Self {
        Self {
            value: *value as u16
        }
    }
}

impl From<u16> for SaturatingU16{
    fn from(value: u16) -> Self {
        Self {value}
    }
}

impl Add for SaturatingU16{
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Self{
            value: self.value.saturating_add(other.value)
        }
    }
}

impl Add<u16> for SaturatingU16{
    type Output = Self;
    fn add(self, other: u16) -> Self{
        Self{
            value: self.value.saturating_add(other)
        }
    }
}

impl Add<&u16> for SaturatingU16{
    type Output = Self;
    fn add(self, other: &u16) -> Self{
        Self{
            value: self.value.saturating_add(*other)
        }
    }
}

impl Add<&Self> for SaturatingU16{
    type Output = Self;
    fn add(self, other: &Self) -> Self{
        Self{
            value: self.value.saturating_add(other.value)
        }
    }
}

impl PartialEq<u16> for SaturatingU16{
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}


