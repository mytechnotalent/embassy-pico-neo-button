![image](https://github.com/mytechnotalent/rust_embassy_pico_project/blob/main/rust_embassy_pico_project.jpg?raw=true)

# Rust Embassy Pico Project

A simple embedded Rust project running on the Raspberry Pi Pico (RP2040), built with Embassy async framework and no_std runtime.

<br>

## FREE Reverse Engineering Self-Study Course [HERE](https://github.com/mytechnotalent/Reverse-Engineering-Tutorial)

<br>

## Wiring
![image](https://github.com/mytechnotalent/embassy-pico-neo-button/blob/main/diagrams/Debug-Probe-Wiring.png?raw=true)
![image](https://github.com/mytechnotalent/embassy-pico-neo-button/blob/main/diagrams/rust_embassy_pico_project.png?raw=true)

## Features
- Configures an onboard LED (GPIO25).
- Configures a button (GPIO16) with internal pull-up.
- Turns LED on when button is pressed, off when released.
- Debounces the button with async timer.
- Runs under Embassy's async executor with no RTOS, no heap.

## Project Structure
- `main.rs`: Initializes Embassy, spawns the main async task.
- `button.rs`: Provides button GPIO initialization.
- `led.rs`: Provides simple onboard LED control abstraction.
- `run_cycle.rs`: Defines a single button-press LED-control cycle.

## How It Works (Step-by-Step)

1. **Startup**
   - The RP2040 boot ROM loads your program from flash.
   - The Cortex-M `cortex-m-rt` runtime (`#[no_main]`) skips the traditional `main()` and jumps into the reset vector.
   - The `__cortex_m_rt_main_trampoline` is called automatically at startup.
   - `__cortex_m_rt_main_trampoline` calls `__cortex_m_rt_main`, which manually initializes and starts the async executor.

2. **Executor Initialization**
   - A static instance of `Executor` is created by `transmute`-ing a stack object to `'static` lifetime.
   - `executor.run()` is called, entering Embassy’s async runtime loop.
   - A `Spawner` is provided by the executor, allowing you to spawn tasks.

3. **Task Spawning (Enqueue Operation)**
   - `Spawner::must_spawn(__embassy_main(spawner))` is called.
   - The `__embassy_main_task` future is created and wrapped into a `Task`.
   - **Enqueue**:
     - The Task is added to the **Task Queue**, a statically allocated double-ended queue (deque) implemented internally by the Executor.
     - This is a lock-free queue; in single-core systems like RP2040, no locks are needed.
     - Enqueue happens at the **tail** (back) of the queue — FIFO behavior is preserved.

4. **Executor Main Loop (Deque + Polling)**
   - The Executor enters its main loop:
     - **Dequeue**:
       - A Task is popped from the **head** (front) of the queue.
     - The Executor **polls** the Task’s future by calling its `poll()` method.
       - If the Future returns `Poll::Pending`, it means it cannot complete immediately:
         - The Task registers a `Waker` — when an awaited event (e.g., timer, GPIO interrupt) completes, the Waker re-schedules the task.
         - The task is then re-**enqueued** at the tail for future polling.
       - If the Future returns `Poll::Ready`, the Task has completed:
         - The Task is **dropped** and removed permanently from the system.
     - If there are no tasks left in the queue, the Executor executes a `WFI` (Wait-For-Interrupt) instruction, entering low-power sleep until an interrupt occurs.

5. **Peripheral Setup**
   - In the spawned `____embassy_main_task`:
     - `embassy_rp::init()` is called to set up clocks, watchdog, and peripherals.
     - `PIN_16` is configured as an input GPIO with an internal pull-up resistor (for the button).
     - `PIN_25` is configured as an output GPIO (for the onboard LED).

6. **Task Execution**
   - Inside the `loop {}`:
     - `run_cycle` is called and awaited:
       - Waits for a button press (falling edge detected on the input pin).
       - Turns the LED on by driving `PIN_25` high.
       - Waits for a button release (rising edge).
       - Turns the LED off by driving `PIN_25` low.
       - Waits 10ms for debounce using `Timer::after_millis`.
     - `run_cycle` returns `Poll::Ready`, but since it's inside an infinite loop, a new future is immediately constructed for the next cycle.
   - As `run_cycle` awaits on GPIO events and timers, the task yields control back to the Executor, causing the task to be **re-enqueued** and other pending tasks (if any) to be polled.

7. **Event Handling and Re-Scheduling**
   - When the awaited GPIO or timer event completes:
     - The Task’s registered `Waker` is triggered.
     - The Task is **re-enqueued** into the Task Queue tail.
     - On the next executor loop iteration, it will be **dequeued** and `poll()` will resume where it left off in its await.

8. **Continuous Loop**
   - The `____embassy_main_task` is never terminated due to its infinite loop.
   - This cycle continues indefinitely, reacting to button presses/releases, toggling the LED accordingly.

---

## Embassy Executor Enqueue/Dequeue In Detail

- **Enqueue (Push-Back)**:
  - When a `Future` yields `Poll::Pending`, the Task’s Waker will call `spawn()`.
  - Internally, this pushes the Task to the **back** of the Task Queue.
  - The Task Queue is lock-free, array-backed with bounded capacity.
  - Waker ensures the task is only enqueued if it was not already enqueued (no duplication).

- **Dequeue (Pop-Front)**:
  - Executor pops a Task from the **front** of the queue (FIFO order).
  - Calls `poll()` on the Task.
  - If `Poll::Pending`, the Task will re-enqueue after its awaited event is ready.
  - If `Poll::Ready`, the Task is cleaned up and removed.

- **Task Scheduling**:
  - Tasks are cooperatively scheduled.
  - No preemption — a task must yield (await) to allow others to run.
  - If all tasks are `Pending`, Executor enters WFI (low-power wait).

- **Wakers**:
  - Embassy provides a lightweight Waker implementation tied to the Task.
  - When a peripheral (e.g., Timer or GPIO interrupt) completes, the Waker triggers the task re-enqueue.

- **No Dynamic Memory**:
  - All Tasks are statically allocated.
  - The queue and task structures are baked into flash/ram at compile time.
  - Ensures no heap fragmentation and determinism — critical for embedded systems.

---

### Deep Dive: `run_cycle` Async State Machine & Code Layout

When you declare:
```rust
pub async fn run_cycle(led: &mut Led, button: &mut Input<'_>) { /* your logic */ }
```
Rust lowers this into two distinct functions:

1. **Future Constructor** at `0x10001904` – builds and returns the state machine struct:

```asm
0x10001904 <+0>:  push    {r7, lr}
0x10001906 <+2>:  add     r7, sp, #0
0x10001908 <+4>:  sub     sp, #12
...                   # store arguments into the Future struct
0x10001918 <+20>: movs    r0, #0       ;; initial state = 0
0x1000191a <+22>: strb    r0, [r1,#16] ;; write state tag field
0x1000191c <+24>: add     sp, #12
0x1000191e <+26>: pop     {r7, pc}
```

This tiny function allocates your `RunCycleFuture`, stores the `led` and `button` references into its fields, sets its `.state` to `0`, and returns immediately back to the executor.

2. **Poll Implementation** (closure) at `0x100006a0` – this is where your high-level Rust logic lives:

```asm
0x100006a0 <+0>:   push    {r4, r6, r7, lr}
...                   # prologue, stack frame setup
0x100006ec <+76>:  ldr     r0, [r0,#12]  ;; load Future.state
0x100006ee <+78>:  bl      0x100005d6    ;; Input::is_low()
0x100006f2 <+82>:  cmp     r0, #0
0x100006f4 <+84>:  bne.n   0x10000740    ;; if button.is_low() -> branch
...                   # await wait_for_high
0x100007cc <+300>: bl      0x1000020a    ;; Led::on()
...                   # await wait_for_low, then Led::off()
0x10000834 <+404>: bl      0x100018de    ;; Timer::after_millis()
0x10000880 <+480>: pop     {r4, r6, r7, pc}
```

Each `bne` or `b.n` jump corresponds to one of the `await` suspension points:

- `state = 0` → evaluate `button.is_low()` and possibly await `wait_for_high()`
- `state = 1` → resumed after `wait_for_high().await`
- `state = 2` → await `wait_for_low().await`
- `state = 3` → execute `led.on()`
- `state = 4` → await `wait_for_high().await`
- `state = 5` → execute `led.off()`
- `state = 6` → await `Timer::after_millis().await`
- `state = DONE` → return `Poll::Ready` and exit

By placing a breakpoint at the closure entry (`0x100006a0`), you dive directly into your button-press/LED logic, skipping the trivial constructor at `0x10001904`.

---

## Building and Flashing

Make sure you have:
- Rust toolchain with thumbv6m-none-eabi target installed.
- Probe-rs or OpenOCD for flashing.

Build:

`cargo build`

Flash:

`cargo flash`

---

## Requirements
- Rust nightly (for async embedded features).
- Embassy (embassy-rp crate) for async HAL support.
- A Raspberry Pi Pico board.

---

## License
Apache-2.0 License

---

## References
- Embassy: Embedded async executor — https://embassy.dev/
- Raspberry Pi Pico Datasheet — https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf