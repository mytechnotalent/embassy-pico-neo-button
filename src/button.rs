//! # Button Control Module
//!
//! ## Features
//! - Initializes GPIO input pins as buttons.
//! - Configures pull-up resistor for stable button reads.

use embassy_rp::gpio::{Input, Pin, Pull};

/// Initializes a button input pin with a pull-up resistor.
///
/// # Arguments
/// * `pin` - Any GPIO pin implementing `Pin`.
///
/// # Returns
/// * `Input<'static>` - Configured input with pull-up.
///
/// # Example
/// ```ignore
/// let button = button::init(p.PIN_16);
/// ```
pub fn init(pin: impl Pin) -> Input<'static> {
    Input::new(pin, Pull::Up)
}
