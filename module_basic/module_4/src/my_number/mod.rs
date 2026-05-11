// A.20.6 - module path attribute: conversion_utility → conversion.rs
#[path = "conversion.rs"]
pub mod conversion_utility;

pub fn is_odd_number(number: i32) -> bool {
    number % 2 != 0
}
