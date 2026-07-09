#![no_std]
//! A portable trait abstraction for GPIO pins that can switch between input and
//! output modes at runtime, built on top of [`embedded_hal`].
//!
//! See [`FlexPin`], [`FlexInputOutput`] and [`FlexInputOutputPull`] for the
//! capability-layered trait hierarchy.

use embedded_hal::digital::InputPin;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::StatefulOutputPin;

pub trait FlexPin: InputPin + OutputPin + StatefulOutputPin {
    type Pull;
    type Speed;

    /// Put the pin into input mode.
    ///
    /// The internal weak pull-up and pull-down resistors will be configured according to `pull`.
    fn set_as_input(&mut self, pull: Self::Pull);

    /// Put the pin into input mode with the internal pull-up resistor enabled.
    ///
    /// Convenience variant of [`Self::set_as_input()`] that takes no pull argument.
    fn set_as_input_pull_up(&mut self);

    /// Put the pin into input mode with the internal pull-down resistor enabled.
    ///
    /// Convenience variant of [`Self::set_as_input()`] that takes no pull argument.
    fn set_as_input_pull_down(&mut self);

    /// Put the pin into input mode with no internal pull resistor.
    ///
    /// Convenience variant of [`Self::set_as_input()`] that takes no pull argument.
    fn set_as_input_no_pull(&mut self);

    /// Put the pin into push-pull output mode.
    ///
    /// The pin level will be whatever was set before (or low by default). If you want it to begin
    /// at a specific level, call `set_high`/`set_low` on the pin first.
    ///
    /// The internal weak pull-up and pull-down resistors will be disabled.
    fn set_as_output(&mut self, speed: Self::Speed);

    /// Put the pin into push-pull output mode at the implementation's default speed.
    ///
    /// Convenience variant of [`Self::set_as_output()`] that takes no speed argument.
    fn set_as_output_default_speed(&mut self);
}

/// Extension for pins that can additionally be put into input + open-drain output mode.
///
/// This capability is only available on some MCUs, so it lives in a separate trait. Implement it
/// only for pins whose hardware supports open-drain input/output multiplexing.
pub trait FlexInputOutput: FlexPin {
    /// Put the pin into input + open-drain output mode.
    ///
    /// The hardware will drive the line low if you set it to low, and will leave it floating if you set
    /// it to high, in which case you can read the input to figure out whether another device
    /// is driving the line low.
    ///
    /// The pin level will be whatever was set before (or low by default). If you want it to begin
    /// at a specific level, call `set_high`/`set_low` on the pin first.
    ///
    /// The internal weak pull-up and pull-down resistors will be disabled.
    fn set_as_input_output(&mut self, speed: Self::Speed);

    /// Put the pin into input + open-drain output mode at the implementation's default speed.
    ///
    /// Convenience variant of [`Self::set_as_input_output()`] that takes no speed argument.
    fn set_as_input_output_default_speed(&mut self);
}

/// Extension for pins that support open-drain input/output mode *with* internal pull resistors.
///
/// Even fewer MCUs support driving an open-drain line while also enabling an internal weak
/// pull-up/pull-down, so this sits above [`FlexInputOutput`] in the capability hierarchy.
pub trait FlexInputOutputPull: FlexInputOutput {
    /// Put the pin into input + open-drain output mode with internal pullup or pulldown.
    ///
    /// This works like [`FlexInputOutput::set_as_input_output()`], but it also allows to enable
    /// the internal weak pull-up or pull-down resistors.
    fn set_as_input_output_pull(&mut self, speed: Self::Speed, pull: Self::Pull);

    /// Put the pin into input + open-drain output mode with internal pullup or pulldown
    /// at the implementation's default speed.
    ///
    /// Convenience variant of [`Self::set_as_input_output_pull()`] that takes no speed argument.
    fn set_as_input_output_pull_default_speed(&mut self, pull: Self::Pull);

    /// Put the pin into input + open-drain output mode with the internal pull-up resistor enabled.
    ///
    /// Convenience variant of [`Self::set_as_input_output_pull()`] that takes no pull argument.
    fn set_as_input_output_pull_up(&mut self, speed: Self::Speed);

    /// Put the pin into input + open-drain output mode with the internal pull-up resistor enabled,
    /// at the implementation's default speed.
    ///
    /// Convenience variant of [`Self::set_as_input_output_pull_up()`] that takes no speed argument.
    fn set_as_input_output_pull_up_default_speed(&mut self);

    /// Put the pin into input + open-drain output mode with the internal pull-down resistor enabled.
    ///
    /// Convenience variant of [`Self::set_as_input_output_pull()`] that takes no pull argument.
    fn set_as_input_output_pull_down(&mut self, speed: Self::Speed);

    /// Put the pin into input + open-drain output mode with the internal pull-down resistor enabled,
    /// at the implementation's default speed.
    ///
    /// Convenience variant of [`Self::set_as_input_output_pull_down()`] that takes no speed argument.
    fn set_as_input_output_pull_down_default_speed(&mut self);

    /// Put the pin into input + open-drain output mode with no internal pull resistor.
    ///
    /// Convenience variant of [`Self::set_as_input_output_pull()`] that takes no pull argument.
    fn set_as_input_output_no_pull(&mut self, speed: Self::Speed);

    /// Put the pin into input + open-drain output mode with no internal pull resistor,
    /// at the implementation's default speed.
    ///
    /// Convenience variant of [`Self::set_as_input_output_no_pull()`] that takes no speed argument.
    fn set_as_input_output_no_pull_default_speed(&mut self);
}
