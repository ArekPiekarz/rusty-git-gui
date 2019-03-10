use crate::error_handling::exit;
use std::fmt::Display;
use std::marker::Copy;

pub fn toI32<T>(number: T) -> i32
    where i32: cast::From<T, Output=Result<i32,cast::Error>>,
            T: Copy + Display
{
    cast::i32(number)
        .unwrap_or_else(|e| exit(&format!("Failed to cast {} to i32: {}", number, e)))
}
