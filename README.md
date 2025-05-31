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

## Reverse Engineering and Debugging with VS Code

This project includes a `.vscode/launch.json` configuration for [probe-rs](https://probe.rs/) debugging. This allows you to reverse engineer and deeply understand the async runtime and your embedded application:

- **Set Breakpoints Anywhere:** You can set breakpoints in your Rust source code, including inside async functions, macros, and even in the expanded code (see `expanded_embassy_pico_neo_button.rs`).
- **Step Through Execution:** Use the VS Code debugger to step through the initialization, executor setup, and your async tasks. This is invaluable for learning how async/await and Embassy's executor work under the hood.
- **Inspect State:** Pause execution and inspect variables, the executor's state, and the contents of the task queue. This helps you see how futures are managed and polled.
- **Reverse Engineering:** By combining the expanded code with live debugging, you can trace exactly how high-level Rust async code is transformed and executed on the device. This is a powerful way to learn embedded async Rust and the Embassy framework.

**How the process and flow works (with expanded code):**

1. **Startup:** The Cortex-M runtime calls the exported `main` symbol, which is the `__cortex_m_rt_main_trampoline` function. This trampoline safely calls the real main logic in `__cortex_m_rt_main`.
2. **Executor Setup:** In `__cortex_m_rt_main`, an `Executor` is created and made static. This executor is the async runtime for your device.
3. **Spawning the Main Task:** The executor's `run` method is called, passing a closure that receives a `Spawner`. The closure uses `spawner.must_spawn(__embassy_main(spawner))` to start the main async task.
4. **Task Storage:** The executor manages a queue of tasks (futures), each stored in a `TaskStorage` structure. This includes metadata, the actual future, and a reference for type-erased polling.
5. **Polling Futures:** The executor dequeues tasks and calls `poll` on their futures. When a future awaits something (like a button press), it yields control, and the executor can poll other tasks.
6. **Debugging:** With the debugger, you can break at any of these steps—startup, executor creation, task spawning, or inside your async logic. You can see the expanded code to understand exactly what is happening at each step, and inspect the executor's queue and the state of your tasks.

**How to use:**
1. Open the project in VS Code.
2. Go to the Run and Debug panel.
3. Select the `probe_rs Executable Test` configuration.
4. Set breakpoints as desired in your code (including in expanded code for deep inspection).
5. Start debugging to flash and connect to your Pico, then step through the process.

This workflow is ideal for both development and reverse engineering, letting you see the real execution flow and internal state of your async embedded application, and how the high-level Rust code is transformed and executed by the Embassy runtime.

## Project Structure
- `src/main.rs`: Async entry point, main loop, and high-level logic
- `src/config.rs`: Hardware setup and pin configuration
- `src/run_cycle.rs`: WS2812 (Neopixel) and LED control main loop logic
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
        run_cycle::run_cycle(&mut ws, &mut led, &mut button).await;
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

## Deep Dive: Async in Embedded Systems and Embassy (Full Technical Walkthrough)

### Async in Embedded vs. Standard Environments
- In standard Rust async (e.g., Tokio, async-std, smol), you have an OS, threads, and high-level executors (e.g., work-stealing, multi-threaded).
- In embedded, you have no OS—everything is bare metal. You must implement async yourself, including polling, task management, and power efficiency.
- Busy-loop polling is unacceptable in embedded: it wastes CPU and battery. Instead, you must sleep the CPU when idle and wake on events (interrupts, timers).

### Futures, Polling, and Non-blocking IO
- A `Future` is a computation that may not be ready yet. You poll it to drive it to completion.
- Non-blocking IO: Instead of waiting (blocking) for an operation (e.g., file read, network), you poll the future and do other work while waiting.
- In embedded, polling must be event-driven (not busy-looped) to save power.

### Executors and Queues
- The executor is the async runtime. It manages a run-queue of tasks (futures), polling them as needed.
- Embassy's executor uses a lock-free atomic linked list for its run-queue (no fixed capacity, highly efficient for embedded).
- The executor is made static (lives for the program's lifetime) for safety and correctness.
- There are different executor modes: thread-mode (lowest privilege, uses WFE/SEV for sleep/wake) and interrupt-mode (uses interrupts for wakeup).

### Spawner and Task Storage
- The `Spawner` is used to enqueue (spawn) tasks onto the executor's queue.
- Each task is stored in a `TaskStorage` struct, which contains:
  - A header (state, poll function pointer, reference to executor, etc.)
  - The actual future (often stored in an `UninitCell`/`MaybeUninit` for zero-cost abstraction)
- Tasks are statically allocated for safety and efficiency.

### Polling, Wakers, and Wakeup Mechanisms
- The executor dequeues tasks and calls `poll` on their futures.
- If a future is not ready, it returns `Pending` and is re-queued or scheduled for wakeup.
- Wakers are used to notify the executor when a future is ready to make progress (e.g., a timer expires, an interrupt fires).
- On Cortex-M, the executor uses WFE (Wait For Event) to sleep and SEV (Send Event) to wake up. Interrupts or task enqueuing trigger SEV.
- For IO or button events, interrupts are used to wake the executor and re-queue the relevant task.

### Timer Implementation
- Timer futures store an expiration time in their task storage.
- The executor keeps polling the timer future until the expiration time is reached, then marks it as ready.
- Embassy provides a timer driver abstraction (e.g., RTC for nRF chips), so you don't need to write your own unless your board is unsupported.

### Tradeoffs and Recommendations
- Async in embedded trades performance and efficiency for increased complexity and abstraction.
- Use async only when it provides real benefits (resource efficiency, IO concurrency, low power).
- Embassy hides much of the complexity, but understanding the internals is important for debugging and advanced use.

### Debugging and Visualization
- Debugging async/await and executors in embedded is challenging due to abstraction and concurrency.
- Use expanded code (`cargo expand`) and live debugging (e.g., probe-rs, VS Code) to understand the runtime and task flow.
- Inspect the executor's queue, task storage, and poll/wake cycles to diagnose issues.

### Summary and Conclusion
- Embassy provides a batteries-included async framework for embedded Rust, with HALs, timer abstractions, and efficient executors.
- The main macro sets up the executor, spawner, and main async task.
- Tasks are statically allocated and managed in a lock-free queue.
- Polling, wakers, and hardware events (interrupts, timers) drive async progress without busy-looping.
- The framework is extensible, with many examples (UART, channels, Ethernet, GPS, etc.) and support for custom HALs.
- Async is powerful but complex—use it judiciously and leverage Embassy's abstractions and documentation.

## Project-Specific Notes: Async, Embassy, and This Project

- **Why Async?** Async/await is only worth the complexity if you need to maximize performance, minimize memory/bandwidth, or handle many concurrent IO-bound tasks. For simple LED blinking, async is overkill, but this project uses it as a learning and demonstration tool.
- **No Busy-Looping:** Embassy avoids busy-loop polling to save battery and CPU. The executor uses WFE/SEV (Wait For Event/Send Event) or interrupts to sleep and wake the MCU efficiently.
- **Executor Details:** The executor is static for the program’s lifetime, enforced with unsafe code (transmute). The run-queue is a lock-free atomic linked list (no fixed capacity). This project uses the thread-mode executor.
- **Spawner and Task Spawning:** The spawner is used to enqueue tasks. The macro uses `must_spawn` to panic on failure, ensuring reliable task startup.
- **Task Storage and Header:** Each task is stored in a `TaskStorage` struct with a header (state, poll function pointer, executor ref, etc.) and the future (in UninitCell/MaybeUninit). The header tracks state (spawned, in run queue, in timer queue), and the poll function pointer is stored for type erasure.
- **Polling, Wakers, and Wakeup:** Polling a future returns Ready or Pending. If Pending, the task is re-queued or scheduled for wakeup. Wakers notify the executor (via SEV or interrupts) when a task is ready to be polled again. For timers, the expiration time is stored in the task, and the executor checks for expired timers.
- **Timer Driver Abstraction:** Embassy provides a timer driver (e.g., RTC for nRF) via embassy-time. You don’t need to write your own unless your board is unsupported.
- **Debugging and Reverse Engineering:** Use `cargo expand` to see macro expansion and VS Code/probe-rs to step through the actual runtime, including expanded code. This is invaluable for understanding how async/await and the executor work under the hood.
- **Tradeoffs:** Async is powerful but complex. Use it only when it makes sense for your use case. Embassy hides much of the complexity, but understanding the internals is important for advanced debugging and optimization.
- **Extensibility:** Embassy is extensible, with many examples for advanced use cases (UART, channels, Ethernet, GPS, etc.). If your board isn’t supported, you can write your own HAL.

## Usage
- Press the button: Both the onboard LED and the WS2812 will light up
- Release the button: Both LEDs will turn off

## Code Overview
The main async loop repeatedly calls `run_cycle::run_cycle`, which handles button state and LED updates. Hardware setup is abstracted in `config.rs` for clarity and reusability.

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
