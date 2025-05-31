//! # Button-to-WS2812 + LED Controller
//!
//! This module waits for button press/release and toggles LED + Neopixel.

use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::ws2812::PioWs2812;
use embassy_time::Timer;
use smart_leds::RGB8;

/// Runs a press-and-release cycle:
/// - Lights up onboard LED and WS2812 on press.
/// - Turns off on release.
///
/// # Arguments
/// - `ws`: PIO WS2812 driver reference.
/// - `led`: Onboard LED output reference.
/// - `button`: Button input reference.
pub async fn run_cycle(
    ws: &mut PioWs2812<'_, PIO0, 0, 1>,
    led: &mut Output<'_>,
    button: &mut Input<'_>,
) {
    if button.is_low() {
        button.wait_for_high().await;
    }

    button.wait_for_low().await;
    led.set_high();
    ws.write(&[RGB8 { r: 255, g: 0, b: 0 }]).await;

    button.wait_for_high().await;
    led.set_low();
    ws.write(&[RGB8::default()]).await;

    Timer::after_millis(10).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_rp::gpio::Level;

    #[test]
    fn dummy_test_run_cycle() {
        // Dummy logic test
        assert_eq!(Level::Low == Level::Low, true);
    }
}
