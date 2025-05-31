#![feature(prelude_import)]
#![no_std]
#![no_main]
//! # RP2040 WS2812 + Button Example
//!
//! This example lights the onboard LED and a WS2812 (Neopixel) when a button is pressed,
//! and turns them off when released. Runs async with Embassy.
#[prelude_import]
use core::prelude::rust_2024::*;
#[macro_use]
extern crate core;
use embassy_executor::Spawner;
use embassy_rp::init;
use {defmt_rtt as _, panic_probe as _};
mod config {
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
    use smart_leds::RGB8;
    struct Irqs;
    #[automatically_derived]
    impl ::core::marker::Copy for Irqs {}
    #[automatically_derived]
    impl ::core::clone::Clone for Irqs {
        #[inline]
        fn clone(&self) -> Irqs {
            *self
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe extern "C" fn PIO0_IRQ_0() {
        <InterruptHandler<
            PIO0,
        > as ::embassy_rp::interrupt::typelevel::Handler<
            ::embassy_rp::interrupt::typelevel::PIO0_IRQ_0,
        >>::on_interrupt();
    }
    unsafe impl ::embassy_rp::interrupt::typelevel::Binding<
        ::embassy_rp::interrupt::typelevel::PIO0_IRQ_0,
        InterruptHandler<PIO0>,
    > for Irqs {}
    /// Configures peripherals: WS2812, onboard LED, and button.
    ///
    /// # Returns
    /// - `PioWs2812` driver instance for Neopixel control.
    /// - `Output` onboard LED.
    /// - `Input` button input with pull-up resistor.
    pub async fn setup(
        p: embassy_rp::Peripherals,
    ) -> (PioWs2812<'static, PIO0, 0, 1>, Output<'static>, Input<'static>) {
        let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);
        let prog = PioWs2812Program::new(&mut common);
        let mut ws = PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_17, &prog);
        let led = Output::new(p.PIN_25, Level::Low);
        let button = Input::new(p.PIN_16, Pull::Up);
        ws.write(&[RGB8::default()]).await;
        (ws, led, button)
    }
}
mod ws2812 {
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
}
#[doc(hidden)]
async fn ____embassy_main_task(_spawner: Spawner) {
    let p = init(Default::default());
    let (mut ws, mut led, mut button) = config::setup(p).await;
    loop {
        ws2812::run_cycle(&mut ws, &mut led, &mut button).await;
    }
}
fn __embassy_main(_spawner: Spawner) -> ::embassy_executor::SpawnToken<impl Sized> {
    const POOL_SIZE: usize = 1;
    static POOL: ::embassy_executor::_export::TaskPoolRef = ::embassy_executor::_export::TaskPoolRef::new();
    unsafe {
        POOL.get::<_, POOL_SIZE>()
            ._spawn_async_fn(move || ____embassy_main_task(_spawner))
    }
}
#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    #[allow(static_mut_refs)] __cortex_m_rt_main()
}
fn __cortex_m_rt_main() -> ! {
    unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
        ::core::mem::transmute(t)
    }
    let mut executor = ::embassy_executor::Executor::new();
    let executor = unsafe { __make_static(&mut executor) };
    executor
        .run(|spawner| {
            spawner.must_spawn(__embassy_main(spawner));
        })
}
