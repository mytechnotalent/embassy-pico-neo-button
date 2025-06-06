#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::init;
use {defmt_rtt as _, panic_probe as _};

mod button;
mod led;
mod run_cycle;

/// ## Main Entry Point
///
/// Initializes peripherals and continuously runs the button-press cycle.
///
/// # Behavior
/// - Sets up onboard LED and button.
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
    let mut led = led::Led::new(p.PIN_25);

    loop {
        run_cycle::run_cycle(&mut led, &mut button).await;
    }
}
