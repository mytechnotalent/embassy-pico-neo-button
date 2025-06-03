//! Main Loop functionality for the Raspberry Pi Pico.
//!
//! This module waits for a button press/release and toggles:
//! - The onboard LED (GPIO25), and
//! - WS2812 “pixel” number 32 (turning it red at full intensity, then turning everything off).

use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::PIO0;
use embassy_time::Timer;
use smart_leds::RGB8;
use crate::ws2812::{turn_on_led, turn_off_all_leds};
use embassy_rp::pio_programs::ws2812::PioWs2812;

/// Runs a press-and-release cycle:
/// - On button-press: turn the onboard LED on and turn LED 32 red at full intensity.
/// - On button-release: turn the onboard LED off and turn _all_ WS2812 LEDs off.
///
/// # Arguments
/// - `ws`: Mutable reference to the WS2812 driver for 64 LEDs (PIO0, SM0).
/// - `led`: Onboard LED output (GPIO25).
/// - `button`: Button input (GPIO16, pull-up).
pub async fn run_cycle(
    ws: &mut PioWs2812<'_, PIO0, 0, 64>,
    led: &mut Output<'_>,
    button: &mut Input<'_>,
) {
    // If the button is already pressed, wait until it is released
    if button.is_low() {
        button.wait_for_high().await;
    }

    // Wait for the next press
    button.wait_for_low().await;
    // Button pressed: turn onboard LED on, WS2812 #32 red at full intensity (255)
    led.set_high();
    turn_on_led(ws, 32, RGB8 { r: 255, g: 0, b: 0 }, 255).await;

    // Wait for the release
    button.wait_for_high().await;
    // Button released: turn onboard LED off, and turn off all WS2812s
    led.set_low();
    turn_off_all_leds(ws).await;

    // Simple debounce / spacing
    Timer::after_millis(10).await;
}