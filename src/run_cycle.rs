//! # Run Cycle Control Module
//!
//! ## Features
//! - Waits for button press and release events.
//! - Controls onboard LED (GPIO25).
//! - Turns LED on when button pressed, off when released.

use crate::led::Led;
use embassy_rp::gpio::Input;
use embassy_time::Timer;

/// Runs a full button press‐and‐release cycle.
///
/// # Arguments
/// * `led` - Mutable reference to the onboard LED controller.
/// * `button` - Button input (GPIO16 with pull-up).
///
/// # Behavior
/// - On press: Turns on the onboard LED.
/// - On release: Turns off the onboard LED.
/// - Includes a short delay for debounce after the cycle.
///
/// # Example
/// ```ignore
/// run_cycle(&mut led, &mut button).await;
/// ```
pub async fn run_cycle(led: &mut Led, button: &mut Input<'_>) {
    if button.is_low() {
        button.wait_for_high().await;
    }

    button.wait_for_low().await;

    led.on();

    button.wait_for_high().await;

    led.off();

    Timer::after_millis(10).await;
}
