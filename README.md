![image](https://github.com/mytechnotalent/embassy-pico-neo-button/blob/main/embassy-pico-neo-button.jpg?raw=true)

# Embassy Pico Neo Button

This project demonstrates how to use the Raspberry Pi Pico (RP2040) microcontroller with Embassy (an async embedded framework) to control a WS2812 (Neopixel) LED and an onboard LED in response to a button press. The project is written in Rust and is designed for async execution on embedded hardware.

<br>

## FREE Reverse Engineering Self-Study Course [HERE](https://github.com/mytechnotalent/Reverse-Engineering-Tutorial)

<br>

## Wiring
![image](https://github.com/mytechnotalent/embassy-pico-neo-button/blob/main/diagrams/Debug-Probe-Wiring.png?raw=true)
![image](https://github.com/mytechnotalent/embassy-pico-neo-button/blob/main/diagrams/Embassy-Pico-Neo-Button.png?raw=true)

## Features
- Lights up both the onboard LED and a WS2812 (Neopixel) when a button is pressed
- Turns off the LEDs when the button is released
- Uses async/await with Embassy for efficient, non-blocking operation
- Modular code structure for easy extension

## Hardware Requirements
- Raspberry Pi Pico (RP2040)
- WS2812 (Neopixel) LED
- Push button
- Breadboard and jumper wires (for prototyping)

## Software Requirements
- Rust toolchain (with support for embedded targets)
- [Embassy](https://embassy.dev/) async embedded framework
- [defmt](https://github.com/knurling-rs/defmt) for logging
- [panic-probe](https://github.com/knurling-rs/panic-probe) for panic handling
- [smart-leds](https://github.com/smart-leds-rs/smart-leds) for LED control
- [probe-rs](https://probe.rs/) for flashing and debugging (see below for VS Code integration)

## VS Code Debugging with probe-rs
To debug your Raspberry Pi Pico project in VS Code, install the [probe-rs Debugger extension](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger):

1. Open the Extensions view in VS Code (Ctrl+Shift+X or Cmd+Shift+X).
2. Search for `probe-rs Debugger` and install it.
3. Connect your debug probe to the Pico and your computer.
4. Use the Run and Debug panel in VS Code to start a debug session.

For more details, see the [probe-rs Debugger documentation](https://probe.rs/docs/tools/vscode/).

## Project Structure
- `src/main.rs`: Async entry point, main loop, and high-level logic
- `src/config.rs`: Hardware setup and pin configuration
- `src/ws2812.rs`: WS2812 (Neopixel) control logic
- `Cargo.toml`: Project dependencies and metadata
- `memory.x`: Linker script for RP2040
- `build.rs`: Build script (if needed)

## Getting Started

### 1. Install Rust and Required Tools
- Install Rust: https://rustup.rs/
- Add the target for RP2040:
  ```sh
  rustup target add thumbv6m-none-eabi
  ```
- Install [probe-rs](https://probe.rs/) or [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs) for flashing

### 2. Clone the Repository
```sh
git clone <your-repo-url>
cd embassy-pico-neo-button
```

### 3. Build the Project
```sh
cargo build
```

### 4. Flash to the Pico
- Use your preferred tool (e.g., probe-rs, elf2uf2, or drag-and-drop UF2)
- Example with probe-rs:
  ```sh
  cargo run
  ```

### 5. Connect Hardware
- Connect the WS2812 data line to the configured GPIO pin (see `config.rs`)
- Connect the button between the configured GPIO pin and ground
- Power the Pico via USB

### 6. Expand the Source Code (Optional, for Exploration)
To view the fully expanded Rust source code (with all macros and attributes expanded), you can use [`cargo-expand`](https://github.com/dtolnay/cargo-expand):

1. Install `cargo-expand` (if you haven't already):
   ```sh
   cargo install cargo-expand
   ```
2. Run the following command to generate the expanded source for the main binary:
   ```sh
   cargo expand --bin embassy-pico-neo-button > expanded_embassy_pico_neo_button.rs
   ```
   This will create a file named `expanded_embassy_pico_neo_button.rs` in your project directory.

**What does this do?**

- `cargo expand` shows you the result of all macro expansions, attribute expansions, and code generation performed by the Rust compiler for your code. This is useful for understanding what your async code, macros, and attributes actually generate under the hood.
- The output file (`expanded_embassy_pico_neo_button.rs`) is a single, self-contained Rust file that represents the code as the compiler sees it after all expansions. This can help with debugging, learning, or porting code to other environments.

## Example: Main Embassy Task Future

Below is the main async task for this project, as seen in the expanded code:

```rust
#[doc(hidden)]
async fn ____embassy_main_task(_spawner: Spawner) {
    let p = init(Default::default());
    let (mut ws, mut led, mut button) = config::setup(p).await;
    loop {
        ws2812::run_cycle(&mut ws, &mut led, &mut button).await;
    }
}
```

**Explanation:**
- This function is marked `async`, so calling it does not immediately run the code inside. Instead, it returns a `Future`—an object representing a computation that will complete later.
- The Embassy executor takes this future and polls it to completion, driving the async logic.
- The loop inside means the future never completes (it is an infinite task), which is typical for embedded main loops.
- The `.await` calls inside the loop yield control back to the executor until the awaited operation (like button press/release) is ready, allowing other tasks to run efficiently.

This is how async/await and the Embassy executor work together to provide efficient, non-blocking concurrency on embedded devices.

**How does the async main task get executed?**

When the macro expands, the following happens under the hood:

- An `Executor` instance is created. The code ensures this executor has a `'static` lifetime so it can safely run forever on embedded hardware.
- The `run` method is called on the executor, and a `Spawner` is passed in. The `Spawner` is used to spawn tasks (futures) onto the executor.
- Executors internally wrap a queue which holds our futures. These futures are stored in a structure called `TaskStorage`, which contains:
  - A `TaskHeader` (metadata for the task)
  - The actual future, wrapped in a custom type called `UninitCell`
  - A `TaskRef`, which is just a reference to the `TaskStorage` where the future has been type-erased
- The job of the executor is to dequeue futures from its run-queue and call `poll` on them, driving them forward until they complete (or, in the case of an infinite loop, never complete).

This mechanism is what allows async/await code to run efficiently and concurrently on embedded devices, with the executor managing all the scheduling and polling of tasks behind the scenes.

## Example: Embassy Runtime Entry Point (Trampoline and Executor)

Below is the entry point code generated for the Embassy async runtime:

```rust
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
```

**Explanation:**

- The `#[export_name = "main"]` attribute ensures this function is used as the program's entry point, as required by the Cortex-M runtime.
- `__cortex_m_rt_main_trampoline` is a small wrapper ("trampoline") that safely calls the real main function. It is marked `unsafe` and `extern "C"` to match the expected ABI for embedded startup code.
- The trampoline calls `__cortex_m_rt_main()`, which contains the actual application logic.
- Inside `__cortex_m_rt_main`:
  - `__make_static` transmutes a local variable's reference to `'static` so it can live for the entire program (safe here because the function never returns).
  - An `Executor` is created and made static. This executor is responsible for running async tasks.
  - The executor's `run` method is called, passing a closure that receives a `Spawner`.
  - The closure uses `spawner.must_spawn(__embassy_main(spawner))` to start the main async task.
- The executor manages a queue of tasks (futures), polling them as needed to drive the async runtime.

This pattern is typical for async embedded Rust, where the entry point must match the runtime's requirements and set up the async executor to run forever.

## Usage
- Press the button: Both the onboard LED and the WS2812 will light up
- Release the button: Both LEDs will turn off

## Code Overview
The main async loop repeatedly calls `ws2812::run_cycle`, which handles button state and LED updates. Hardware setup is abstracted in `config.rs` for clarity and reusability.

## License
This project is licensed under the Apache License. See [LICENSE](https://www.apache.org/licenses/LICENSE-2.0) for details.

## Acknowledgments
- [Pico](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html)
- [Pico Debug Probe](https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html#updating-the-firmware-on-the-debug-probe)
- [Embassy](https://embassy.dev/)
- [smart-leds](https://github.com/smart-leds-rs/smart-leds)
- [defmt](https://github.com/knurling-rs/defmt)
- [panic-probe](https://github.com/knurling-rs/panic-probe)
- [probe-rs](https://probe.rs/)

---

*Happy hacking with async Rust on embedded hardware!*
