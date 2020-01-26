use std::convert::TryInto as _;
use std::num::FpCategory::{Infinite, Nan, Normal, Subnormal, Zero};

pub trait ToI32
{
    fn toI32(&self) -> i32;
}

#[allow(clippy::as_conversions)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::use_self)]
impl ToI32 for f64
{
    fn toI32(&self) -> i32
    {
        match self.classify() {
            Nan | Infinite => panic!("Cannot convert f64: {} into i32", self),
            Zero | Subnormal => 0,
            Normal => {
                if *self < i32::min_value().into() || *self > i32::max_value().into() {
                    panic!("Cannot convert f64: {} into i32 - out of range", self);
                }
                *self as i32
            }
        }
    }
}

impl ToI32 for usize
{
    fn toI32(&self) -> i32
    {
        (*self).try_into().unwrap()
    }
}