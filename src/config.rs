//! # Hardware Configuration
//!
//! This module initializes:
//! - WS2812 PIO driver on GPIO17,
//! - Onboard LED on GPIO25,
//! - Button input with pull-up on GPIO16.

use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_rp::{bind_interrupts, peripherals::PIO0};

// Bind PIO0 IRQ for WS2812 DMA interrupts
bind_interrupts!(struct Irqs { PIO0_IRQ_0 => InterruptHandler<PIO0>; });

/// Configures peripherals: WS2812, onboard LED, and button.
///
/// # Returns
/// - `PioWs2812` driver instance for Neopixel control.
/// - `Output` onboard LED.
/// - `Input` button input with pull-up resistor.
pub fn setup(
    p: embassy_rp::Peripherals,
) -> (
    PioWs2812<'static, PIO0, 0, 1>,
    Output<'static>,
    Input<'static>,
) {
    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let prog = PioWs2812Program::new(&mut common);
    let ws = PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_17, &prog);

    let led = Output::new(p.PIN_25, Level::Low);
    let button = Input::new(p.PIN_16, Pull::Up);

    (ws, led, button)
}
