//! # Hardware Configuration
//!
//! This module initializes:
//! - WS2812 PIO driver on GPIO17,
//! - Onboard LED on GPIO25,
//! - Button input with pull-up on GPIO16,
//! and provides helpers to turn on/off individual or all LEDs with intensity.

use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_rp::{bind_interrupts, peripherals::PIO0};
use smart_leds::RGB8;

// Bind PIO0 IRQ for WS2812 DMA interrupts
bind_interrupts!(struct Irqs { PIO0_IRQ_0 => InterruptHandler<PIO0>; });

/// Configures peripherals: WS2812 (64 LEDs), onboard LED, and button.
///
/// # Parameters
/// - `p`: Embassy‐RP2040 peripherals instance.
///
/// # Returns
/// - `PioWs2812<'static, PIO0, 0, 64>`: WS2812 driver (configured for 64 LEDs).
/// - `Output<'static>`: Onboard LED (GPIO25).
/// - `Input<'static>`: Button input with pull‐up (GPIO16).
///
/// Initializes the PIO state machine, DMA channel, and ensures all 64 LEDs start off.
pub async fn init(
    p: embassy_rp::Peripherals,
) -> (
    PioWs2812<'static, PIO0, 0, 64>,
    Output<'static>,
    Input<'static>,
) {
    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let prog = PioWs2812Program::new(&mut common);
    let mut ws = PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_17, &prog);

    let led = Output::new(p.PIN_25, Level::Low);
    let button = Input::new(p.PIN_16, Pull::Up);

    // Turn off all 64 LEDs initially
    let off = [RGB8::default(); 64];
    ws.write(&off).await;

    (ws, led, button)
}

/// Turns on a specific LED at a given index with a specified color and intensity.
///
/// # Parameters
/// - `ws`: Mutable reference to the WS2812 driver instance.
/// - `target_index`: Index of the LED to turn on (0-based, must be < 64).
/// - `color`: Desired base color (RGB8).
/// - `intensity`: Brightness level (0–255).
///
/// Fills a 64‐element array with black, then sets `leds[target_index]` to
/// `color` scaled by `intensity`, and writes the entire buffer to the strip.
pub async fn turn_on_led(
    ws: &mut PioWs2812<'_, PIO0, 0, 64>,
    target_index: usize,
    color: RGB8,
    intensity: u8,
) {
    if target_index >= 64 {
        return;
    }
    let mut leds = [RGB8::default(); 64];
    leds[target_index] = scale_color(color, intensity);
    ws.write(&leds).await;
}

/// Turns off all 64 LEDs in the strip.
///
/// # Parameters
/// - `ws`: Mutable reference to the WS2812 driver instance.
///
/// Writes an all‐black (zero) buffer of length 64 to turn every LED off.
pub async fn turn_off_all_leds(ws: &mut PioWs2812<'_, PIO0, 0, 64>) {
    let leds = [RGB8::default(); 64];
    ws.write(&leds).await;
}

/// Turns off a single LED while preserving the current state of the other LEDs.
///
/// # Parameters
/// - `ws`: Mutable reference to the WS2812 driver instance.
/// - `current_state`: Mutable reference to the current RGB8 array of length 64.
/// - `led_index`: Index of the LED to turn off (0-based).
///
/// Sets `current_state[led_index]` to black and writes the updated state back.
pub async fn turn_off_single_led(
    ws: &mut PioWs2812<'_, PIO0, 0, 64>,
    current_state: &mut [RGB8; 64],
    led_index: usize,
) {
    if led_index < 64 {
        current_state[led_index] = RGB8::default();
    }
    ws.write(current_state).await;
}

/// Scales an RGB8 color by a given intensity (0–255).
///
/// # Parameters
/// - `color`: The original RGB8 color.
/// - `intensity`: Brightness scalar (0 = off, 255 = full brightness).
///
/// # Returns
/// - A new `RGB8` where each channel is multiplied by `intensity/255`.
fn scale_color(color: RGB8, intensity: u8) -> RGB8 {
    RGB8 {
        r: ((color.r as u16 * intensity as u16) / 255) as u8,
        g: ((color.g as u16 * intensity as u16) / 255) as u8,
        b: ((color.b as u16 * intensity as u16) / 255) as u8,
    }
}
