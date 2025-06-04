#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::init;
use {defmt_rtt as _, panic_probe as _};

mod button;
mod run_cycle;
mod ws2812;

/// ## Main Entry Point
///
/// Initializes peripherals and continuously runs the button-press cycle.
///
/// # Behavior
/// - Sets up PIO, DMA, onboard LED, and button.
/// - Continuously listens for button presses and reacts accordingly.
///
/// # Example
/// ```no_run
/// embassy_executor::run(|spawner| {
///     main(spawner)
/// });
/// ```
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init(Default::default());
    let mut button = button::init(p.PIN_16);
    let (mut ws, mut led) = ws2812::init::<64>(p.PIO0, p.DMA_CH0, p.PIN_17, p.PIN_25).await;

    loop {
        run_cycle::run_cycle(&mut ws, &mut led, &mut button).await;
    }
}
