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
cd embassy-pico-neobutton
```

### 3. Build the Project
```sh
cargo build --release --target=thumbv6m-none-eabi
```

### 4. Flash to the Pico
- Use your preferred tool (e.g., probe-rs, elf2uf2, or drag-and-drop UF2)
- Example with probe-rs:
  ```sh
  cargo run --release --target=thumbv6m-none-eabi
  ```

### 5. Connect Hardware
- Connect the WS2812 data line to the configured GPIO pin (see `config.rs`)
- Connect the button between the configured GPIO pin and ground
- Power the Pico via USB

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

---

*Happy hacking with async Rust on embedded hardware!*
