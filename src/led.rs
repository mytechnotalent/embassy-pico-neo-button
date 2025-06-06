//! # Onboard LED Control Module
//!
//! ## Features
//! - Initialize onboard LED on configurable GPIO pin.
//! - Turn the LED on and off.
//! - Toggle the LED state.

#![allow(dead_code)]

use embassy_rp::gpio::{Level, Output, Pin};

/// Represents an onboard LED controller.
pub struct Led {
    led: Output<'static>,
}

impl Led {
    /// Initializes the onboard LED.
    ///
    /// # Arguments
    /// * `led_pin` - Pin to control the onboard LED (must implement `Pin`).
    ///
    /// # Returns
    /// * `Led` instance with the specified pin configured.
    ///
    /// # Example
    /// ```ignore
    /// let led = Led::new(led_pin);
    /// ```
    pub fn new(led_pin: impl Pin) -> Self {
        let led = Output::new(led_pin, Level::Low);
        Self { led }
    }

    /// Turns the LED on.
    ///
    /// # Example
    /// ```ignore
    /// led.on();
    /// ```
    pub fn on(&mut self) {
        self.led.set_high();
    }

    /// Turns the LED off.
    ///
    /// # Example
    /// ```ignore
    /// led.off();
    /// ```
    pub fn off(&mut self) {
        self.led.set_low();
    }

    /// Toggles the LED state.
    ///
    /// # Example
    /// ```ignore
    /// led.toggle();
    /// ```
    pub fn toggle(&mut self) {
        self.led.toggle();
    }
}
