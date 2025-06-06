#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2024::*;
#[macro_use]
extern crate core;
use embassy_executor::Spawner;
use embassy_rp::init;
use {defmt_rtt as _, panic_probe as _};
mod button {
    //! # Button Control Module
    //!
    //! ## Features
    //! - Initializes GPIO input pins as buttons.
    //! - Configures pull-up resistor for stable button reads.
    use embassy_rp::gpio::{Input, Pin, Pull};
    /// Initializes a button input pin with a pull-up resistor.
    ///
    /// # Arguments
    /// * `pin` - Any GPIO pin implementing `Pin`.
    ///
    /// # Returns
    /// * `Input<'static>` - Configured input with pull-up.
    ///
    /// # Example
    /// ```ignore
    /// let button = button::init(p.PIN_16);
    /// ```
    pub fn init(pin: impl Pin) -> Input<'static> {
        Input::new(pin, Pull::Up)
    }
}
mod led {
    //! # Onboard LED Control Module
    //!
    //! ## Features
    //! - Initialize onboard LED on configurable GPIO pin.
    //! - Turn the LED on and off.
    //! - Toggle the LED state.
    #![allow(dead_code)]
    use embassy_rp::gpio::{Level, Output, Pin};
    /// Represents an onboard LED controller.
    pub struct Led {
        led: Output<'static>,
    }
    impl Led {
        /// Initializes the onboard LED.
        ///
        /// # Arguments
        /// * `led_pin` - Pin to control the onboard LED (must implement `Pin`).
        ///
        /// # Returns
        /// * `Led` instance with the specified pin configured.
        ///
        /// # Example
        /// ```ignore
        /// let led = Led::new(led_pin);
        /// ```
        pub fn new(led_pin: impl Pin) -> Self {
            let led = Output::new(led_pin, Level::Low);
            Self { led }
        }
        /// Turns the LED on.
        ///
        /// # Example
        /// ```ignore
        /// led.on();
        /// ```
        pub fn on(&mut self) {
            self.led.set_high();
        }
        /// Turns the LED off.
        ///
        /// # Example
        /// ```ignore
        /// led.off();
        /// ```
        pub fn off(&mut self) {
            self.led.set_low();
        }
        /// Toggles the LED state.
        ///
        /// # Example
        /// ```ignore
        /// led.toggle();
        /// ```
        pub fn toggle(&mut self) {
            self.led.toggle();
        }
    }
}
mod run_cycle {
    //! # Run Cycle Control Module
    //!
    //! ## Features
    //! - Waits for button press and release events.
    //! - Controls onboard LED (GPIO25).
    //! - Turns LED on when button pressed, off when released.
    use crate::led::Led;
    use embassy_rp::gpio::Input;
    use embassy_time::Timer;
    /// Runs a full button press‐and‐release cycle.
    ///
    /// # Arguments
    /// * `led` - Mutable reference to the onboard LED controller.
    /// * `button` - Button input (GPIO16 with pull-up).
    ///
    /// # Behavior
    /// - On press: Turns on the onboard LED.
    /// - On release: Turns off the onboard LED.
    /// - Includes a short delay for debounce after the cycle.
    ///
    /// # Example
    /// ```ignore
    /// run_cycle(&mut led, &mut button).await;
    /// ```
    pub async fn run_cycle(led: &mut Led, button: &mut Input<'_>) {
        if button.is_low() {
            button.wait_for_high().await;
        }
        button.wait_for_low().await;
        led.on();
        button.wait_for_high().await;
        led.off();
        Timer::after_millis(10).await;
    }
}
#[doc(hidden)]
async fn ____embassy_main_task(_spawner: Spawner) {
    let p = init(Default::default());
    let mut button = button::init(p.PIN_16);
    let mut led = led::Led::new(p.PIN_25);
    loop {
        run_cycle::run_cycle(&mut led, &mut button).await;
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
