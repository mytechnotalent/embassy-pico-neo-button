#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::init;
use {defmt_rtt as _, panic_probe as _};

mod run_cycle;
mod ws2812;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals (PIO0, DMA_CH0, PIN_17, etc.)
    let p = init(Default::default());

    // This now returns a WS2812 driver configured for 64 LEDs:
    let (mut ws, mut led, mut button) = ws2812::init(p).await;

    loop {
        run_cycle::run_cycle(&mut ws, &mut led, &mut button).await;
    }
}