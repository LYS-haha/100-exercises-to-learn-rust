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
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}


//implementing conversions from u16
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

//implementing conversions from u8
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self { value: value.into() }
    }
}

//implementing conversions from &u16 by use u16.into
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        (*value).into()
    }
}

//implementing conversions from &u8 by use u8.into
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        (*value).into()
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = Self;
    fn add(self, other: SaturatingU16) -> Self::Output {
        self + other.value  //SaturatingU16 + u16
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, other: &SaturatingU16) -> Self::Output {
        self + (*other).value  //SaturatingU16 + u16
    }
}

// Implementing addition with u16
impl std::ops::Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(other)
        }
    }
}

impl std::cmp::PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}