//! # Run Cycle Control Module
//!
//! ## Features
//! - Waits for button press and release events.
//! - Controls:
//!     - Onboard LED (GPIO25).
//!     - WS2812 LED strip (64 LEDs on GPIO17).
//! - Lights up LED 32 red on press, turns all LEDs off on release.

use crate::ws2812::{turn_off_all_leds, turn_on_led};
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::ws2812::PioWs2812;
use embassy_time::Timer;
use smart_leds::RGB8;

/// Runs a full button press‐and‐release cycle.
///
/// # Arguments
/// * `ws` - Mutable reference to the WS2812 driver for 64 LEDs (PIO0, SM0).
/// * `led` - Onboard LED output (GPIO25).
/// * `button` - Button input (GPIO16 with pull-up).
///
/// # Behavior
/// - On press: Turns on the onboard LED and lights up LED 32 red at full intensity.
/// - On release: Turns off the onboard LED and all WS2812 LEDs.
/// - Includes a short delay for debounce after the cycle.
///
/// # Example
/// ```no_run
/// run_cycle(&mut ws, &mut led, &mut button).await;
/// ```
pub async fn run_cycle(
    ws: &mut PioWs2812<'_, PIO0, 0, 64>,
    led: &mut Output<'_>,
    button: &mut Input<'_>,
) {
    if button.is_low() {
        button.wait_for_high().await;
    }

    button.wait_for_low().await;

    led.set_high();
    turn_on_led(ws, 32, RGB8 { r: 255, g: 0, b: 0 }, 255).await;

    button.wait_for_high().await;

    led.set_low();
    turn_off_all_leds(ws).await;

    Timer::after_millis(10).await;
}
