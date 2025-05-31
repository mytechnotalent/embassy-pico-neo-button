#![no_std]
#![no_main]

//! # RP2040 WS2812 + Button Example
//!
//! This example lights the onboard LED and a WS2812 (Neopixel) when a button is pressed,
//! and turns them off when released. Runs async with Embassy.

use embassy_executor::Spawner;
use embassy_rp::init;
use {defmt_rtt as _, panic_probe as _};

mod config;
mod ws2812;

/// RP2040 async entry-point.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init(Default::default());
    let (mut ws, mut led, mut button) = config::setup(p).await;

    loop {
        ws2812::run_cycle(&mut ws, &mut led, &mut button).await;
    }
}
