//! # WS2812 LED Control Module
//!
//! ## Features
//! - WS2812 LED driver initialization on configurable GPIO.
//! - Onboard LED control on configurable GPIO.
//! - Control individual LED colors and brightness.
//! - Turn on/off individual or all LEDs.

use embassy_rp::bind_interrupts;
use embassy_rp::dma::Channel;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio, PioPin};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use smart_leds::RGB8;

// Bind PIO0 IRQ for WS2812 DMA interrupts
bind_interrupts!(struct Irqs { PIO0_IRQ_0 => InterruptHandler<PIO0>; });

/// Initializes WS2812 PIO driver and onboard LED.
///
/// # Type Parameters
/// * `N` - Number of WS2812 LEDs.
///
/// # Arguments
/// * `pio` - PIO0 peripheral instance.
/// * `dma_ch0` - DMA channel (must implement `Channel`).
/// * `ws_pin` - Pin to drive WS2812 data (must implement `PioPin`).
/// * `led_pin` - Pin for onboard LED (must implement `Pin`).
///
/// # Returns
/// * Tuple:
///     - `PioWs2812<'static, PIO0, 0, N>`: WS2812 driver configured for `N` LEDs.
///     - `Output<'static>`: Onboard LED initialized.
///
/// # Behavior
/// Initializes PIO, DMA, and sets all `N` LEDs to off.
pub async fn init<const N: usize>(
    pio: PIO0,
    dma_ch0: impl Channel,  // DMA channel 0
    ws_pin: impl PioPin,    // e.g. PIN_17
    led_pin: impl Pin,      // e.g. PIN_25
) -> (PioWs2812<'static, PIO0, 0, N>, Output<'static>) {
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio, Irqs);

    let prog = PioWs2812Program::new(&mut common);
    let mut ws = PioWs2812::new(&mut common, sm0, dma_ch0, ws_pin, &prog);

    let led = Output::new(led_pin, Level::Low);

    let off = [RGB8::default(); N];
    ws.write(&off).await;

    (ws, led)
}

/// Turns on a specific LED with given color and intensity.
///
/// # Arguments
/// * `ws` - Mutable reference to the WS2812 driver.
/// * `target_index` - Index of the LED to turn on (0-based, must be < `N`).
/// * `color` - Desired RGB8 color.
/// * `intensity` - Brightness level (0–255).
///
/// # Behavior
/// Turns off all LEDs except the specified one, which is set to the given color and intensity.
#[allow(dead_code)]
pub async fn turn_on_led<const N: usize>(
    ws: &mut PioWs2812<'_, PIO0, 0, N>,
    target_index: usize,
    color: RGB8,
    intensity: u8,
) {
    if target_index >= N {
        return;
    }

    let mut leds = [RGB8::default(); N];

    leds[target_index] = scale_color(color, intensity);
    ws.write(&leds).await;
}

/// Turns off all LEDs.
///
/// # Arguments
/// * `ws` - Mutable reference to the WS2812 driver.
///
/// # Behavior
/// Writes an array of black to turn all LEDs off.
#[allow(dead_code)]
pub async fn turn_off_all_leds<const N: usize>(ws: &mut PioWs2812<'_, PIO0, 0, N>) {
    let leds = [RGB8::default(); N];
    
    ws.write(&leds).await;
}

/// Turns off a single LED while preserving other LEDs' states.
///
/// # Arguments
/// * `ws` - Mutable reference to the WS2812 driver.
/// * `current_state` - Mutable reference to the current LED array.
/// * `led_index` - Index of the LED to turn off.
///
/// # Behavior
/// Sets the specified LED to black without affecting others.
#[allow(dead_code)]
pub async fn turn_off_single_led<const N: usize>(
    ws: &mut PioWs2812<'_, PIO0, 0, N>,
    current_state: &mut [RGB8; N],
    led_index: usize,
) {
    if led_index < N {
        current_state[led_index] = RGB8::default();
    }
    ws.write(current_state).await;
}

/// Scales an RGB8 color by intensity.
///
/// # Arguments
/// * `color` - The original RGB8 color.
/// * `intensity` - Brightness scalar (0–255).
///
/// # Returns
/// * Scaled `RGB8` color with adjusted brightness.
///
/// # Example
/// ```
/// let red = RGB8 { r: 255, g: 0, b: 0 };
/// let dim_red = scale_color(red, 128); // 50% brightness
/// ```
#[allow(dead_code)]
pub fn scale_color(color: RGB8, intensity: u8) -> RGB8 {
    RGB8 {
        r: ((color.r as u16 * intensity as u16) / 255) as u8,
        g: ((color.g as u16 * intensity as u16) / 255) as u8,
        b: ((color.b as u16 * intensity as u16) / 255) as u8,
    }
}
