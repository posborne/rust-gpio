// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Generic Interface for GPIOs Across Many Systems
//!
//! This crate provides a set of core traits and related types
//! for accessing GPIOs across a variety of systems.  The goal
//! is that devices may be written depending on these traits
//! and consequently be portable to various systems.

#![crate_type = "lib"]
#![crate_name = "gpio"]

#![feature(no_std)]
#![no_std]

#[cfg(test)]
#[macro_use] extern crate std;

#[derive(Copy, Clone)]
pub enum Error {
    /// The GPIO was configured as an input but an output action was attempted
    IllegalOperationOnInput,
    /// The GPIO was configured as an output but an input action was attempted
    IllegalOperationOnOutput,
    /// Some other error occurred
    Other(&'static str),
}

pub type Result<T> = core::result::Result<T, Error>;

/// The direction of a GPIO Pin
#[derive(Copy, Clone)]
pub enum Direction {
    /// The pin is an input
    In,
    /// The pin is an output
    Out,
}

/// The logic level of a GPIO Pin
#[derive(Copy, Clone, PartialEq)]
pub enum Level {
    /// Logic low
    Low,
    /// Logic high
    High,
}

/// Generic trait providing access to a single GPIO
pub trait Gpio {
    /// Get the currently configured direciton of this GPIO
    fn get_direction(&self) -> Result<Direction>;
    /// Set the pin to be an input
    fn set_input(&mut self) -> Result<()>;
    /// Set the pin to be an output with the provided level
    fn set_output(&mut self, value: Level) -> Result<()>;
    /// Get the current level of this GPIO (input only)
    fn get_level(&self) -> Result<Level>;
    /// Set the current level of this GPIO (output only)
    fn set_level(&mut self, value: Level) -> Result<()>;
}
