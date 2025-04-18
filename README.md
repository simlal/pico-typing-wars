
# Project: Pico Button Wars

Course project for _IFT-769 - Real-Time Systems_.

Using the Raspberry Pi Pico platform, we'll implement a reaction time game (and possibly a typing game)
with electronic components (buttons, LEDs, LCD screen, USB keyboard).

The project is divided into 2 parts:

1. **Part 1**: Primary implementation of a reaction time game with buttons and LEDs
2. **Part 2 (optional)**: Implementation of a typing speed game with a USB keyboard and LCD display

**TLDR:** See the [demonstration for part 1 here](#demo-camera)

## Table of Contents

<!--toc:start-->
- [Project: Pico Button Wars](#project-pico-button-wars)
  - [Table of Contents](#table-of-contents)
  - [Introduction :book:](#introduction-book)
    - [Overview](#overview)
      - [Part 1](#part-1)
      - [Part 2 (TODO)](#part-2-todo)
    - [Why Rust](#why-rust)
  - [Schedule and Deliverables :calendar:](#schedule-and-deliverables-calendar)
    - [Personal Objectives](#personal-objectives)
  - [Development Environment :hammer_and_wrench:](#development-environment-hammer-and-wrench)
    - [Standard Rust Toolchain](#standard-rust-toolchain)
    - [Resources, Libraries and Tools for Rust Development on Embedded Systems](#resources-libraries-and-tools-for-rust-development-on-embedded-systems)
    - [Debugging with `probe-rs` and Build System with `cargo`](#debugging-with-probe-rs-and-build-system-with-cargo)
      - [Debugging with a 2nd Raspberry Pi Pico](#debugging-with-a-2nd-raspberry-pi-pico)
      - [Build System](#build-system)
      - [Project `blinky`](#project-blinky)
  - [Development Environment Instructions](#development-environment-instructions)
    - [Rust Installation](#rust-installation)
    - [Toolchain Selection](#toolchain-selection)
    - [Building and/or Running the `blinky` Project](#building-andor-running-the-blinky-project)
    - [Building and/or Running the `pico-button-wars` Project](#building-andor-running-the-pico-button-wars-project)
  - [Hardware Setup :rocket:](#hardware-setup-rocket)
    - [Required Materials](#required-materials)
  - [Pico Button Wars :video_game:](#pico-button-wars-video-game)
    - [Code Structure](#code-structure)
      - [Overview](#overview-1)
      - [`common.rs`](#commonrs)
      - [`game.rs`](#gamers)
      - [`led.rs`](#ledrs)
      - [`button.rs`](#buttonrs)
        - [Debounce Test](#debounce-test)
        - [Asynchronous Task for Button Monitoring (reset)](#asynchronous-task-for-button-monitoring-reset)
      - [`main.rs`](#mainrs)
        - [Initialization and Launching Asynchronous Tasks](#initialization-and-launching-asynchronous-tasks)
        - [Main Game Loop](#main-game-loop)
  - [Demo :camera:](#demo-camera)
    - [Game Example](#game-example)
    - [Reset with Both Buttons](#reset-with-both-buttons)
  - [Conclusion :checkered_flag:](#conclusion-checkered-flag)
  - [References :books:](#references-books)
<!--toc:end-->

## Introduction :book:

### Overview

The project involves creating a reaction time and typing speed game using the Raspberry Pi Pico platform. The game is divided into two parts:

1. **Part 1**: Primary implementation of a reaction time game with buttons and LEDs
2. **Part 2 (TODO)**: Implementation of a typing speed game with a USB keyboard and LCD display

**TLDR:** See the [demonstration for part 1 here](#demo-camera)

#### Part 1

The game is designed to test players' reaction time and speed. Part 1 primarily consists of pressing a button as soon as the LED lights up after a random delay following game initiation. There will be 2 buttons and 2 LEDs to allow 2 players to play simultaneously. The game loop consists of:

1. Waiting for a random delay
2. Turning on both LEDs
3. Waiting for players to press their respective buttons
4. Measuring reaction time and declaring the winner
5. Flashing the winner's LED
6. Repeating the game.

#### Part 2 (TODO)

Part 2 is an extension of Part 1, where the game tests players' typing speed. The game takes the winner from Part 1 and has them play a typing speed game. The player must repeat a sequence of characters displayed on an LCD screen. These sequences will be random lines of code (possibly from the source code?). The player must type the sequence as quickly as possible within a time limit (based on the number of characters).

### Why Rust

Rust is a modern programming language that emphasizes memory safety, speed, and concurrency. It supports multiple programming paradigms and can be used for various purposes (e.g., system programming, backend/server development, CLI tools, etc.).

Rust is also known for its ownership and lifetime systems, as well as its strict type checker, which ensures memory safety without requiring garbage collection, thanks to the compiler's borrowing verification mechanism. Additionally, it's an increasingly popular language in both industry and academia.

Since this course focuses on real-time systems, Rust is a wise choice for this project due to its performance, memory safety, and suitability for real-time constraints and the limited resources of the Raspberry Pi Pico platform.

## Schedule and Deliverables :calendar:

- [x] _L00_: Introduction and planning :calendar:
- [ ] _L01_: Progress with demonstration of part of the project (part 1 desired!) :video_game:
- [ ] _L02_: Presentation with demonstration :rocket: of the final project and report

| Deliverable | Deadline | Description |
| -------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| L00      | 2025-01-27  | Introduction and planning. GitHub repo, documentation, required materials, etc. |
| L01      | 2025-03-10  | Progress with demonstration of part of the project. Execution of part 1 desired, otherwise in finalization. Source code, documentation, demonstration video, etc. |
| L02      | 2025-04-07  | Final project presentation with demonstration. Source code, documentation, completed demonstration video with final report. |

### Personal Objectives

- Introduction to embedded systems programming, the Raspberry Pi Pico platform, and Rust programming.
- Introduction to using electronic peripherals, communication protocols (I2C, SPI, USB).
- Development of real-time programming skills, resource management, and time constraints.

## Development Environment :hammer_and_wrench:

After initially trying to use the Raspberry Pi Pico C/C++ SDK, I decided to switch to Rust for this project. Although C is the preferred language for embedded systems, my initial impression of the toolchain with _CMake_ via the Pico SDK, the semi-functional debugging via a 2nd Pico with _OpenOCD_ and _GDB_, and the lack of native SDK support for _multi-threading_ pushed me to explore an alternative.

I also encountered an issue with peripheral initialization via the Pico SDK, where the `blinky` example didn't work. Using the 2nd Pico debugger, I could see that the code was executing but was blocking in the initialization code in the following loop:

```c
while (!time_reached(t_before)) {
    uint32_t save = spin_lock_blocking(sleep_notifier.spin_lock);
    lock_internal_spin_unlock_with_wait(&sleep_notifier, save);
}
```

A trace of this approach remains on the `c-version-sdk` branch. I therefore decided to switch to Rust for this project.

After briefly exploring the introduction to Rust via **The Rust Programming Language**, I decided that it seemed to be a judicious choice for this project (as well as for personal learning). The Rust toolchain is well integrated with embedded systems development.

### Standard Rust Toolchain

The Rust toolchain managed via `rustup` contains:

- `rustc`: Rust compiler
- `cargo`: Package manager and build system
- `rustup`: Version and toolchain manager
- `rls`: Rust Language Server for integration with text editors
- `rustfmt`: Formatter for Rust
- `clippy`: Linter for Rust
- `rust-analyzer`: Code analyzer for Rust (Interface with editors)

### Resources, Libraries and Tools for Rust Development on Embedded Systems

First, a key resource is **The Embedded Rust Book** which is a comprehensive resource for Rust development on embedded systems. Additionally, there is a [Rust Embedded Working Group](https://github.com/rust-embedded) that provides tools, libraries, and resources for Rust development on embedded systems.

Next, there are several libraries and tools for Rust development on embedded systems:

- `svd2rust`: Rust code generator from SVD (System View Description) files for ARM peripherals. [svd2rust executable](https://docs.rs/svd2rust/latest/svd2rust/)
- `probe-rs`: Programming and debugging tool for ARM Cortex-M microcontrollers. [Official Website](https://probe.rs/)
- `cortex-m`: Library for ARM Cortex-M development in Rust. Includes interrupt routines, error handling, etc. [GitHub](https://github.com/rust-embedded/cortex-m)
- `embedded-hal`: Peripheral abstraction for embedded systems. [GitHub](https://github.com/rust-embedded/embedded-hal)
- `rp2040-pac`: ARM Cortex-M0+ peripherals for the Raspberry Pi Pico. [GitHub](https://github.com/rp-rs/rp2040-pac)
- `rp-rs/rp-hal`: HAL for the Raspberry Pi Pico. [GitHub](https://github.com/rp-rs/rp-hal)
- `embassy-rs`: Asynchronous framework for embedded systems. [Official Website](https://embassy.dev/) [5]

### Debugging with `probe-rs` and Build System with `cargo`

#### Debugging with a 2nd Raspberry Pi Pico

As found in the [Raspberry Pi Pico documentation](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf), it's possible to use a 2nd Pico as a debugger for the main Pico. The connection diagram can be seen here where the debugger Pico is connected to the computer and the main Pico is connected to the debugger Pico via the `SWD` pins:

<img src="./media/pico-debug.png" alt="pico-debugging" width="400">

This allows us to both simplify loading executables onto the main Pico and also debug the code using `probe-rs` and `gdb`.

#### Build System

In the context of embedded systems, the `blinky` project is a demonstration project that consists of making an LED blink. It's the equivalent of `hello world!` for embedded systems. I was inspired by the example of the `blinky` project with `embassy-rs` to set up the development environment.

This framework will allow us to manage peripherals and interrupts asynchronously without resorting to an RTOS (Real-Time Operating System). By minimizing dependencies, a build system with `cargo` and `probe-rs`, we have a good starting point with [blinky found in this repository](./blinky/).

Here are the necessary elements to establish a build system with `cargo` and a Pi Pico:

**`build.rs`**:

- Facilitates integration of the memory address map for the Pico with the `memory.x` file. Used by crates related to peripheral access (_PAC_) and hardware abstractions (_HAL_).
- Passes compilation flags to the linker and compiler. i.e., `--nmagic` allows disabling page alignment since we don't use such a memory pagination system in an embedded system like the Pico.

**`memory.x`**:

- Memory configuration file for the linker. Defines the memory sections for the bootloader, flash memory, and RAM of the Pico.

**`Cargo.toml`**:

- Configuration file for `cargo` for the project. Contains dependencies, build configurations for compilation.
- Also contains information about our project (name, version, author, etc.)

The `release` and `dev` profiles are configured here, meaning that when we compile our project with `cargo build --release`, the production version compilation options are used. In our case, we have the following options:

```toml
# Build configuration for the production version
[profile.release]
debug = 2  # Full debug level
lto = true  # Link Time Optimization active, so code optimization at compilation
opt-level = 'z'  # Optimization level for minimizing binary size
```

**`rust-toolchain.toml`**:

- Configuration file for `rustup` that allows specifying the Rust toolchain (version and components).

**`.cargo/config.toml`**:

- Configuration file for `cargo` that allows specifying build options for the project.
- In our case, we specify `probe-rs` as the `runner` for debugging and the `thumbv6m-none-eabi` target for compilation.

`thumbv6m-none-eabi` is the target for ARM Cortex-M0 and M0+ microcontrollers (the Pico's processor).

**`main.rs`**:

- Main source file of the project.
- The `#![no_std]` attribute indicates that we're not using the Rust standard library.
- The `#![no_main]` attribute indicates that we're not using the Rust `main` function, but rather the `embassy_executor::main` function provided by the `embassy-rs` framework.

There will be more details on how `embassy-rs` works with `async/await` later.

#### Project `blinky`

The `blinky` project is a demonstration project that consists of making an LED blink on the Raspberry Pi Pico. Using the `embassy-rs` framework, this allows us to easily make an LED blink by taking advantage of asynchronous features (especially for the timer).

We see the LED on the Pico (corresponding to pin 25) blinking at a frequency of 1Hz:
<img src="./media/pico-blinky-live.gif" alt="blinky-live" width="300">

On the console:

<img src="./media/pico-blinky-console.gif" alt="blinky-console" width="800">

## Development Environment Instructions

### Rust Installation

Using rustup.sh, we install the Rust version manager `rustup` which allows managing Rust versions and toolchains.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or also with your distribution's package manager:

```bash
# Ubuntu/debian
sudo apt install rustup

# Fedora
sudo dnf install rustup
rustup-init
```

We can verify the installation with:

```bash
rustup --version
rustc --version
```

We need `probe-rs` for debugging and programming the Pico. We install it with `cargo` or with a package manager:

```bash
# with cargo (from source)
cargo install probe-rs

# Installation script for Linux
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

probe-rs --version
```

### Toolchain Selection

To facilitate the installation of the toolchain for development on the Pi Pico, we can first clone the repository:

```bash
git clone git@github.com:simlal/pico-button-wars.git
cd pico-button-wars
```

For developing with Rust on the Raspberry Pi Pico, we need the `nightly-2024-12-10` Rust toolchain to compile for the `thumbv6m-none-eabi` target.

Being in the project directory, the toolchain should be automatically detected by `rustup`.

This is specified in the `rust-toolchain.toml` file:

```toml
[toolchain]
channel = "nightly-2024-12-10"
targets = [
    "thumbv6m-none-eabi",
]
```

We can also add components like `rust-analyzer` and `rustfmt` for our IDE:

```bash
rustup component add rust-analyzer
rustup component add rustfmt
```

### Building and/or Running the `blinky` Project

To run the `blinky` project, we can use `cargo` to compile and run the project:

```bash
cd blinky
# To build only 
cargo build --release
```

The _release_ mode is used here out of habit because we use the 'z' optimization _flag_ to minimize the binary size. Here's the compilation profile in the `Cargo.toml` file:

```toml
[profile.release]
debug = 2
lto = true
opt-level = 'z'
```

We're still in debug mode with `debug = 2` to have debugging information and the environment variable for our log mode:

```toml
[env]
DEFMT_LOG = "debug"
```

With the debugger Pico connected, we can directly compile and run `blinky`:

```bash
cargo run --release

# We should see the flash and the INFO log messages 
Finished `release` profile [optimized + debuginfo] target(s) in 0.16s
     Running `probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/blinky`
      Erasing ✔ 100% [####################]  12.00 KiB @  52.45 KiB/s (took 0s)
  Programming ✔ 100% [####################]  12.00 KiB @  41.28 KiB/s (took 0s)                                                            Finished in 0.53s
0.000350 INFO  Turning onboard led pin output to high...
└─ blinky::____embassy_main_task::{async_fn#0} @ src/main.rs:22
0.000379 INFO  led on!
└─ blinky::____embassy_main_task::{async_fn#0} @ src/main.rs:24
1.000426 INFO  Turning onboard led pin output to low...
└─ blinky::____embassy_main_task::{async_fn#0} @ src/main.rs:27
1.000446 INFO  led off!
...
```

### Building and/or Running the `pico-button-wars` Project

Same principle for the `pico-button-wars` project:

```bash
cd pico-button-wars
cargo build --release
```

## Hardware Setup :rocket:

As seen in the connection diagram for debugging, we need to connect the debugger Pico to the computer. Thus, the main Pico is easily connected to the debugger Pico via the `SWD` pins to flash the code and debug.

In addition to the basic setup, for our project, we'll need to connect the following components:

- 2 LEDs (red and green)
- 2 differently colored buttons
- LCD screen
- USB keyboard (Optional)
- USB adapters, _breadboard_, connection wires, etc.

We use different colors for the LEDs and buttons to distinguish the players.

### Required Materials

For part 1, we need the following components:

| Component(s) | Quantity | Description | Price |
| ------------- | -------- | ---------- | ---- |
| Raspberry Pi Pico-H | 2 | ARM Cortex-M0+ microcontroller with pre-installed headers | $7 each |
| Breadboards | 2 | 400-point breadboards | $5 each |
| Jumper wires | TODO | For connecting components | < $3 |
| LEDs | 2 | Red and green | < $3 |
| Resistors | ??? | LEDs=1kOhm, Buttons=10kOhm, ??? | < $3 |

For part 2, we add a USB keyboard to test players' typing speed and an adapter.

| Component(s) | Quantity | Description | Price |
| ------------- | -------- | ---------- | ---- |
| LCD Screen | 1 | 3.2 Inch 320x240 Touch LCD | $20 |
| USB Keyboard | 1 | USB Keyboard | ~$10 |
| OTG Adapter | 1 | USB-A to micro USB conversion | ~$10 |

## Pico Button Wars :video_game:

The project involves creating a reaction time and button game on a Raspberry Pi Pico.
(I didn't have time to do part 2 so here's part 1 only.)

### Code Structure

#### Overview

The code is divided into several modules to facilitate reading and understanding. The different modules are:

- `common.rs`: Module containing project utility functions (i.e., Rng, console formatting trait, etc.)
- `game.rs`: Module containing the game logic. Contains the main game object to handle transitions between game states.
- `led.rs`: Module containing the logic to control LEDs. Contains functions to turn LEDs on and off.
- `button.rs`: Module containing the logic to control buttons. Contains functions to read the button state, handle debounce, etc.
- `main.rs`: Main source file of the project. Contains peripheral initialization and the main game loop.

**NOTE**:

#### `common.rs`

Possibly a poor choice of module name (could have contained generic types, common traits, etc.), but mainly contains the simplistic Rng:

```rust
pub struct SimpleRngU64 {
    seed: u64,
}
impl SimpleRngU64 {
    pub fn new() -> Self {
        // Use the current time as initial seed
        let now = Instant::now();
        let seed = now.as_micros();
        Self { seed }
    }

    // Seed update
    pub fn next_u64(&mut self) -> u64 {
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        self.seed = self.seed.wrapping_mul(A).wrapping_add(C);
        self.seed
    }

    // Linear congruential generator implementation
    pub fn generate_from_range(&mut self, from: u64, to: u64) -> u64 {
        if from >= to {
            return from;
        }
        from + (self.next_u64() % (to - from + 1))
    }
}
```

Useful for having a random light time for the LED before turning it off and thus making the game less predictable.

#### `game.rs`

The `game.rs` module contains the game logic. It contains the different game stages and transitions between game states. It's used as a static singleton to manage the game state. It also contains functions to manage transitions between game states.

```rust

type GameMutex = Mutex<CriticalSectionRawMutex, Option<Game>>;
static GAME: GameMutex = Mutex::new(None);

#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum GameState {
    Waiting,
    Playing,
    ComputingResults,
    Finished,
}

// Singleton game instance
#[derive(Format)]
struct Game {
    state: GameState,
    state_start: Instant,
    state_duration: Duration,
}

// With state transition management methods
fn update_state_duration(&mut self) {...} 
fn transition(&mut self, next_state: GameState) {...}
...
// We access the singleton with GAME.lock()
// Example access to gamestate and reset with watchdog
pub async fn get_current_game_state_or_reset(
    wd: &'static Mutex<ThreadModeRawMutex, Option<Watchdog>>,
) -> GameState {
    let game_lock = GAME.lock().await;
    match game_lock.as_ref() {
        Some(game) => game.state,
        None => {
            async {
                warn!(
                    "Attempted to get game state but GAME singleton not initialized. Resetting..."
                );
                // Lock the watchdog to prevent feeding
                let _lock_forever = wd.lock().await;
                loop {
                    Timer::after_secs(10).await; // Keep the lock forever
                }
            }
            .await;
            // HACK: Should not be reached, but fallback
            GameState::Waiting
        }
    }
}
```

#### `led.rs`

The `led.rs` module contains the logic to control the LEDs. It contains functions to turn LEDs on and off, both for basic routines and abstractions to reflect the game state.

```rust
 
#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum LedRole {
    Onboard,
    Player1,
    Player2,
}

// A simple abstraction over an output pin with a role
pub struct Led<'a> {
    output: Output<'a>,
    role: LedRole,
}
impl Led<'_> {
    pub fn new<P: Pin>(pin: P, role: LedRole) -> Self {
        Self {
            output: Output::new(pin, Level::Low), // Initialize Output with the pin
            role,
        }
    }

    pub fn turn_on(&mut self) {
        self.output.set_high();
    }

    pub fn turn_off(&mut self) {
        self.output.set_low();
    }

  // others ...

}

// Example used for the game's on/off lighting routine:

pub async fn round_playing_leds_routine_on_off(
    leds: &'_ mut [Led<'_>; 3],
    current_round: usize,
) -> Instant {
    // Signal that round 'i' is about to start then quick blinky
    info!("Players get ready for round {}", current_round);
    for _ in 0..current_round + 1 {
        for led in leds.iter_mut() {
            led.turn_on();
        }
        Timer::after_millis(750).await;
        for led in leds.iter_mut() {
            led.turn_off();
        }
        Timer::after_millis(750).await;
    }
    Timer::after_millis(500).await;
    for _ in 0..4 {
        for led in leds.iter_mut() {
            led.turn_on();
        }
        Timer::after_millis(150).await;
        for led in leds.iter_mut() {
            led.turn_off();
        }
        Timer::after_millis(150).await;
    }

    // Generate random time in ms between 2000-5000 ms for led signal to press button
    let mut rng = SimpleRngU64::new();
    let leds_duration = rng.generate_from_range(2000, 5000);
    info!(
        "Rng time for LED ON until shutoff for current game round: {} ms. ",
        leds_duration
    );
    for led in leds.iter_mut() {
        led.turn_on();
    }
    Timer::after_millis(leds_duration).await;

    for led in leds.iter_mut() {
        led.turn_off();
    }
    info!("GO!");
    Instant::now()
}
```

#### `button.rs`

The `button.rs` module contains the logic to control the buttons. It contains functions to read the button state, have a task for game 'reset' (via watchdog starvation), evaluate debounce value, wait for and measure button press actions.

This is therefore an important module that needs to be well tested to avoid false positives and false negatives, ensure nothing is blocked during execution for proper game execution.

The general structure of the module is as follows:

```rust
// Debounce time with prior tests from measure_minimal_debounce()
const MINIMAL_DEBOUNCE_TIME: u64 = 50;

#[derive(PartialEq, Eq, Format, Clone, Copy, Debug, Hash)]
pub enum ButtonRole {
    Player1,
    Player2,
}

pub struct Button<'a> {
    input: Input<'a>,
    role: ButtonRole,
    debounce: Duration,
}
```

And methods for both waiting for a button press, debounce, or simply monitoring its state:

```rust

impl Button<'_> {
    pub fn new<P: Pin>(pin: P, role: ButtonRole) -> Self {
        Self {
            input: Input::new(pin, Pull::Up), // Initialize input with pull up
            role,
            debounce: Duration::from_millis(MINIMAL_DEBOUNCE_TIME),
        }
    }

    pub fn role(&self) -> ButtonRole {
        self.role
    }

    async fn wait_for_press(&mut self) -> Instant {
        loop {
            self.input.wait_for_falling_edge().await;
            let press_instant = Instant::now();
            Timer::after(self.debounce).await;
            // safety in case debounce not enough
            if self.input.get_level() == Level::Low {
                info!("{} button pressed.", self.role);
                return press_instant;
            }
        }
    }

    async fn wait_for_release(&mut self) -> Instant {
        loop {
            self.input.wait_for_low().await;
            let release_instant = Instant::now();
            Timer::after(self.debounce).await;
            // safety in case debounce not enough
            if self.input.get_level() == Level::High {
                info!("{} button released.", self.role);
                return release_instant;
            }
        }
    }

    pub async fn measure_full_press_release(&mut self) -> Instant {
        self.wait_for_press().await;
        return self.wait_for_release().await;
    }

  // Other methods...
}
```

##### Debounce Test

I first performed tests to measure our _worst case debounce time_ for the button, while also evaluating if the button behavior was acceptable.

Using this test routine, we evaluate the minimal debounce time for the button. We can use it to adjust the debounce value in the game code afterward as a `const` variable when constructing a `Button` instance:

```rust

pub async fn measure_minimal_debounce(&mut self, ms_test_range: u64, iterations: usize) -> u64 {
    const MIN_DEBOUNCE_DEFAULT_IN_TEST: u64 = 50;
    info!(
        "Measuring debounce for {} Button with {} ms max and averaging over {}",
        self.role, ms_test_range, iterations
    );
    let mut total_transitions = 0;
    let mut max_debounce_time = 0;
    for i in 0..iterations {
        // Wait for an initial press
        self.input.wait_for_low().await;
        info!("Button pressed! Measuring minimal debounce time");

        // Debounce
        let mut transitions = 0;
        let mut last_level = Level::Low; // We just checked its low

        let start_time = Instant::now();
        let mut last_transition_time = start_time;
        let mut longest_debounce = Duration::from_millis(0);
        // Fix: Add duration to start_time instead of subtracting
        let end_time = start_time + Duration::from_millis(ms_test_range);

        // Evaluate max transition time
        while Instant::now() < end_time {
            let current_level = self.input.get_level();
            if current_level != last_level {
                transitions += 1;
                let now = Instant::now();

                // No need to debounce if no transitions
                if transitions > 1 {
                    let bounce_duration = now - last_transition_time;
                    if bounce_duration > longest_debounce {
                        longest_debounce = bounce_duration;
                        debug!("New longest debounce: {} ms", bounce_duration.as_millis());
                    }
                }

                last_transition_time = now;
                debug!(
                    "Transition #{} detected from {} to {} at {} ms from test start.",
                    transitions,
                    self.level_to_str(&last_level),
                    self.level_to_str(&current_level),
                    (last_transition_time - start_time).as_millis()
                );
                last_level = current_level;
            }

            // Small delay to prevent tight CPU looping
            Timer::after_micros(50).await;
        }

        info!(
            "Detected {} transitions in iteration {}",
            transitions,
            i + 1
        );
        if transitions > 0 {
            info!(
                "Longest debounce interval: {}ms",
                longest_debounce.as_millis()
            );
            max_debounce_time = max_debounce_time.max(longest_debounce.as_millis());
        }

        total_transitions += transitions;

        info!(
            "Found {} transitions with longest_debounce time of {} ms for test iteration i={}",
            transitions,
            longest_debounce.as_millis(),
            i
        );

        // Wait for button release before next iteration
        if i < iterations - 1 {
            self.input.wait_for_high().await;
            // Add delay between tests
            Timer::after_millis(500).await;
        }
    }
    // Compute summary
    let avg_transitions = if iterations > 0 {
        total_transitions / iterations as u64
    } else {
        0
    };
    info!(
        "Summary: Avg transitions={}, longest_debounce_time={} ms over {} iterations.",
        avg_transitions, max_debounce_time, iterations
    );
    info!(
        "Returning 10% over maximum debounce time or default {}",
        MIN_DEBOUNCE_DEFAULT_IN_TEST
    );
    (max_debounce_time + (max_debounce_time / 10)).max(MIN_DEBOUNCE_DEFAULT_IN_TEST)
}

```

Thus, by calling the function on each of the buttons, multiple times with at least 10 iterations, we can evaluate the minimal debounce time for the button. We can then use it to adjust the debounce value in the game code afterward as a `const` variable when constructing a `Button` instance.

I therefore evaluated that in the worst case generally, a value of 50 ms would be adequate (the majority of tests were below 50ms, but there were a few cases of ~100ms, so I decided to take the value of 50ms for the debounce.

##### Asynchronous Task for Button Monitoring (reset)

Since we want to _spawn_ an asynchronous task for the buttons with _Embassy_, we need to have a static reference for each of the buttons (so _lifetime_ of the entire program). Thus, when any resource wants to access a button, it must do so via a _Mutex_.

```rust

// Could be subject to interrupt but OK for now
pub type ButtonMutex = Mutex<ThreadModeRawMutex, Option<Button<'static>>>;

// In main.rs we have an initialization where we crash if there's an issue!
static BUTTON_P1: ButtonMutex = Mutex::new(None);
static BUTTON_P2: ButtonMutex = Mutex::new(None);
{
  let mut button_p1_unlocked = BUTTON_P1.lock().await;
  *button_p1_unlocked = Some(Button::new(p.PIN_10, ButtonRole::Player1));

  let mut button_p2_unlocked = BUTTON_P2.lock().await;
  *button_p2_unlocked = Some(Button::new(p.PIN_11, ButtonRole::Player2));

  // Making sure we panic if unproperly init
  match *button_p1_unlocked {
      None => crate::panic!("Could not initialize player 1 button."),
      Some(_) => info!("Initialized 'BUTTON_P1'  as static shareable thread-safe ref",),
  }
  match *button_p2_unlocked {
      None => crate::panic!("Could not initialize player 2 button."),
      Some(_) => info!("Initialized 'BUTTON_P2'  as static shareable thread-safe ref",),
  }
}
```

**Task to monitor simultaneous press on both buttons to trigger a reset via the _Watchdog_:**

Here, we have an asynchronous task that monitors both buttons and triggers a reset if both buttons are pressed simultaneously for more than 3 seconds. We try not to block the mutex for too long to avoid blocking the game. We use a `select` to monitor both buttons simultaneously, which allows seeing which of the _futures_ completes first (and otherwise releasing the mutex quickly by exiting the scope).

By evaluating state changes and updating press times, we can monitor if both buttons are pressed simultaneously for more than 3 seconds. If so, we trigger a reset via the _Watchdog_.

```rust

#[embassy_executor::task(pool_size = 1)]
pub async fn monitor_double_longpress(
    b1: &'static ButtonMutex,
    b2: &'static ButtonMutex,
    wd: &'static Mutex<ThreadModeRawMutex, Option<Watchdog>>,
) {
    // Less-blocking approach with 50 ms polling on button mutex
    let mut ticker = Ticker::every(Duration::from_millis(50));

    // Track long press state
    let mut b1_pressed_time: Option<Instant> = None;
    let mut b2_pressed_time: Option<Instant> = None;
    let mut reset_countdown_active = false;

    loop {
        // Check both buttons without holding locks for too long
        let b1_pressed = {
            // Only try to lock for a short 10ms time before giving up
            match select(b1.lock(), Timer::after(Duration::from_millis(10))).await {
                Either::First(button_lock) => {
                    if let Some(button) = button_lock.as_ref() {
                        button.input.get_level() == Level::Low
                    } else {
                        false // Could not acquire lock
                    }
                }
                Either::Second(_) => {
                    // Couldn't get lock, maintain previous state
                    b1_pressed_time.is_some()
                }
            }
        };

        let b2_pressed = {
            // Only try to lock for a short time before giving up
            match select(b2.lock(), Timer::after(Duration::from_millis(10))).await {
                Either::First(button_lock) => {
                    if let Some(button) = button_lock.as_ref() {
                        button.input.get_level() == Level::Low
                    } else {
                        false // Could not acquire lock
                    }
                }
                Either::Second(_) => {
                    // Couldn't get lock, maintain previous state
                    b2_pressed_time.is_some()
                }
            }
        };

        // Update press times
        if b1_pressed && b1_pressed_time.is_none() {
            b1_pressed_time = Some(Instant::now());
            debug!("Button 1 pressed");
        }

        if b2_pressed && b2_pressed_time.is_none() {
            b2_pressed_time = Some(Instant::now());
            debug!("Button 2 pressed");
        }

        // Check for button releases
        if !b1_pressed && b1_pressed_time.is_some() {
            let duration = b1_pressed_time.unwrap().elapsed();
            debug!("Button 1 released after {} ms", duration.as_millis());
            b1_pressed_time = None;
            reset_countdown_active = false;
        }

        if !b2_pressed && b2_pressed_time.is_some() {
            let duration = b2_pressed_time.unwrap().elapsed();
            debug!("Button 2 released after {} ms", duration.as_millis());
            b2_pressed_time = None;
            reset_countdown_active = false;
        }

        // Check for longpress condition
        if let (Some(t1), Some(t2)) = (b1_pressed_time, b2_pressed_time) {
            let b1_duration = t1.elapsed();
            let b2_duration = t2.elapsed();

            if !reset_countdown_active
                && b1_duration.as_millis() >= 1000
                && b2_duration.as_millis() >= 1000
            {
                reset_countdown_active = true;
                info!(
                    "Both buttons held for 1+ second. Continuing to monitor for reset threshold..."
                );
            }

            // Check if press duration is enough to trigger reset
            if b1_duration.as_millis() >= 3000 && b2_duration.as_millis() >= 3000 {
                info!(
                    "Long press detected on both buttons (b1={} ms, b2={} ms). Resetting via watchdog...",
                    b1_duration.as_millis(),
                    b2_duration.as_millis()
                );

                // Lock the watchdog to prevent feeding
                let _lock_forever = wd.lock().await;
                loop {
                    Timer::after_secs(10).await; // Keep the lock forever
                }
            }
        }

        ticker.next().await;
    }
}
```

#### `main.rs`

The main logic of the project is in the `main.rs` file. It contains peripheral initialization, the main game loop, and asynchronous tasks.

Using the `embassy` executor, we can easily create asynchronous tasks to manage buttons and the watchdog, use timers, and wait for responses to _futures_. This API is compatible with the `rp-rs` and `embedded-hal` HALs and PACs, libraries for stack containers like `heapless`, and other resources in embedded programming.

The main is decorated with the `#[embassy_executor::main]` feature flag which allows specifying the main function of the program. It's important to note that we're not using the Rust `main` function, but rather the `embassy_executor::main` function provided by the `embassy-rs` framework.

We therefore have access to the `embassy` executor's spawner to create asynchronous tasks.

```rust

#[embassy_executor::main]
async fn main(spawner: Spawner) {...}
```

##### Initialization and Launching Asynchronous Tasks

We must first initialize the Pico's peripherals. We need:

- 3 LEDS: (onboard, player1, player2)
- 2 buttons: (player1, player2)

And our global instances:

- 1 watchdog: (for game reset)
- 1 Game: (the game logic)

Finally our containers for rules and scores (on the stack obviously):

```rust
const TOTAL_ROUNDS: usize = 5;
const WIN_THRESHOLD: usize = TOTAL_ROUNDS.div_ceil(2);
let mut round_winner_times: [(Option<ButtonRole>, u64); TOTAL_ROUNDS] =
    [(None::<ButtonRole>, u64::MIN); TOTAL_ROUNDS];

let mut players_scores = FnvIndexMap::<ButtonRole, usize, 2>::new();
players_scores.insert(ButtonRole::Player1, 0).unwrap();
players_scores.insert(ButtonRole::Player2, 0).unwrap();
```

This is executed before the main game loop. We make the program panic if initialization fails.

We can thus launch our watchdog fed every 500 ms with a starvation time of 3s, which allows time for a reset if both buttons are pressed simultaneously for more than 3 seconds, reset in case it would be blocked elsewhere:

```rust
#[embassy_executor::task(pool_size = 1)]
pub async fn feed_watchdog(
    wd: &'static Mutex<ThreadModeRawMutex, Option<Watchdog>>,
    feed_schedule: Duration,
) {
    let mut ticker = Ticker::every(feed_schedule);
    loop {
        {
            let mut wd_unlocked = wd.lock().await;
            if let Some(wd) = wd_unlocked.as_mut() {
                wd.feed();
                // info!("watchdog fed")
            }
        } // watchdog lock dropped here
        ticker.next().await;
    }
}
```

As mentioned above, we also have an asynchronous task to monitor the buttons and trigger a reset if both buttons are pressed simultaneously for more than 3 seconds, which will block access to the watchdog and naturally starve it, thus triggering a reset.

##### Main Game Loop

We have 4 main states according to the `GameState` enum:

- `Waiting`: Waiting for button press to start the game
- `Playing`: Playing
- `ComputingResults`: Computing results
- `Finished`: The game is finished. We start over

**_Waiting_ Mode**:

The first step is to evaluate the GameState at the beginning of each game loop and transition to the next state based on the `match`.

```rust
GameState::Waiting => {
  info!("We are waiting! Resetting scores before next game");
  // Resetting scores in case we are coming in from a previous game
  for (role, time) in round_winner_times.iter_mut() {
    *role = None;
    *time = 0;
  }

  if let Entry::Occupied(mut o) = players_scores.entry(ButtonRole::Player1) {
    *o.get_mut() = 0;
  }
  if let Entry::Occupied(mut o) = players_scores.entry(ButtonRole::Player2) {
    *o.get_mut() = 0;
  }
  // Wait for single press on each
  loop {
    waiting_state_leds(&mut leds).await;

    info!("Press any button within the next 2 seconds to start the game...");
    let mut b1_unlocked = BUTTON_P1.lock().await;
    let mut b2_unlocked = BUTTON_P2.lock().await;
    if let (Some(b1_ref), Some(b2_ref)) =
    (b1_unlocked.as_mut(), b2_unlocked.as_mut())
    {
      match select3(
        b1_ref.wait_for_full_press(),
        b2_ref.wait_for_full_press(),
        Timer::after_secs(2),
      )
      .await
      {
        Either3::First(_) => {
          info!("Player 1 button pressed, we can start the game!");
          break;
        }
        Either3::Second(_) => {
          info!("Player 2 button pressed, we can start the game!");
          break;
        }
        Either3::Third(_) => {
          info!("Timeout! going through routine again.")
        }
      }
    }
  }
  transition_game_state(GameState::Playing).await;
}
```

The Waiting mode prepares the game for a new round and waits for the game start trigger. This mode:

- Completely resets previous players' scores and times
- Resets counters for both players
- Activates a distinctive light pattern on the LEDs indicating the waiting state
- Enters an invitation sequence where:
  - A message invites players to press a button within 2 seconds
  - The system simultaneously monitors three possible events:
    - Full button press by player 1
    - Full button press by player 2
    - 2-second timeout expiration
  - If a player presses their button, the game can start and transitions to Playing mode
  - If no button is pressed within the time limit, the invitation sequence restarts

This waiting phase ensures the game starts under fair conditions with a complete score reset, while offering an intuitive user interface to start a new game.

**_Playing_ Mode**:

The Playing mode is the heart of the game where players compete against each other. This mode:

1. Indicates that the game is in progress and prepares players for the first round.
2. For each round, it executes a preparation sequence:
   - The LEDs flash to signal the start of the round.
   - A random time is generated to turn on the LED.
   - Players must press their button as soon as the LED turns off.
3. The system monitors both buttons to determine which player reacted the fastest.
4. The round winner is determined based on reaction time and is recorded.
5. Players' scores are updated and displayed.
6. If we have a winner (best of 5, but modifiable in the code), the game transitions to the _ComputingResults_ state.

We allow 2 seconds between each round to let players prepare and call reset if needed.

We block the button mutex to avoid interfering with the game, but we ensure to release it quickly after each round.

```rust
GameState::Playing => {
  info!("We are playing!");
  'rounds: for (i, round) in round_winner_times.iter_mut().enumerate() {
    info!("Players get ready for round #{}", i);

    // Insure we have both button mutex
    let mut b1_unlocked = BUTTON_P1.lock().await;
    let mut b2_unlocked = BUTTON_P2.lock().await;
    if let (Some(b1_ref), Some(b2_ref)) =
    (b1_unlocked.as_mut(), b2_unlocked.as_mut())
    {
      // Randomized time w/ light ON then OFF + pick first to full press w/ time
      let target_time_press =
      round_playing_leds_routine_on_off(&mut leds, i).await;
      let winner_timepress = select(
        b1_ref.measure_full_press_release(),
        b2_ref.measure_full_press_release(),
      )
      .await;

      // Use the button to match the winner led and add it to scores container
      let winner = match winner_timepress {
        Either::First(p1_release) => {
          info!("B1 was faster!");
          let p1_score = (p1_release - target_time_press).as_millis();
          (b1_ref.role(), p1_score)
        }
        Either::Second(p2_release) => {
          info!("B2 was faster!");
          let p2_score = (p2_release - target_time_press).as_millis();
          (b2_ref.role(), p2_score)
        }
      };
      // Update the player scores
      if let Entry::Occupied(mut o) = players_scores.entry(winner.0) {
        *o.get_mut() += 1;
      }

      // Save score and highlight round winner
      highlight_round_winner(
        &mut leds,
        winner.0,
        *players_scores.get(&winner.0).unwrap(),
      )
      .await;
      *round = (Some(winner.0), winner.1);
      info!(
        "DINGINGINGING! Congratulations for {} with a response time of {} ms",
        winner.0, winner.1
      );
      // If we have a winner (best of 5), transition to Computing Results
      info!("Current scores: ");
      for (player, score) in &players_scores {
        info!("{}: {}", player, score);
        if *score == WIN_THRESHOLD {
          transition_game_state(GameState::ComputingResults).await;
          break 'rounds;
        }
      }
    }
    info!(
    "Target window for ressetting game with long button double press of 2s..."
  );
    drop(b1_unlocked);
    drop(b2_unlocked);
    Timer::after_secs(2).await; // Just before starting next round
  }
}
```

**_ComputingResults_ Mode**:

We calculate the game results and display the winner. We use the players' response times to determine the best player and display the game statistics. The LED flashing routine is used to highlight the winner.

```rust
GameState::ComputingResults => {
  info!("Computing results for current game...");
  let highest_scorer = players_scores
    .iter()
    .max_by_key(|&(_, score)| score)
    .map(|(player, _)| *player)
    .unwrap();

  let mut best_response_time = u64::MAX;
  let mut worst_response_time = u64::MIN;
  let mut avg_response_time: u64 = 0;

  for (role, time) in &round_winner_times {
    if let Some(r) = role {
      if *r == highest_scorer {
        // Compute stats for winner
        avg_response_time += *time;
        if *time < best_response_time {
          best_response_time = *time;
        }
        if *time > worst_response_time {
          worst_response_time = *time;
        }
      }
    }
  }
  avg_response_time /= WIN_THRESHOLD as u64;

  // Log stats and celebrate winner
  info!("Winner {} had an avg response time of {} ms (best time {} ms, worst time {} ms",
    highest_scorer,
    avg_response_time,
    best_response_time,
    worst_response_time
  );
  Timer::after_secs(1).await; // Let us read before transition!
  highlight_game_winner(&mut leds, highest_scorer).await;
  game::transition_game_state(GameState::Finished).await;
}
```

**_Finished_ Mode**:

A simple reset to Waiting mode.

```rust
GameState::Finished => {
  info!("Finished the game. Going back into waiting mode.");
  game::transition_game_state(GameState::Waiting).await;
}
```

## Demo :camera:

### Game Example

[Demo Video](https://youtube.com/shorts/mJzi2ivcp6k?feature=share)

And the corresponding logs:

```text
pico-button-wars/pico-button-wars on  main is 󰏗 v0.1.0 via 󱘗 v1.85.0-nightly took 5s
❯ cargo run --release
    Finished `release` profile [optimized + debuginfo] target(s) in 0.14s
     Running `probe-rs run --chip RP2040 --log-format '{t} - {f} [{L:<4}]  {s}
' target/thumbv6m-none-eabi/release/pico-button-wars`
      Erasing ✔ 100% [####################]  24.00 KiB @  61.93 KiB/s (took 0s)
  Programming ✔ 100% [####################]  24.00 KiB @  44.36 KiB/s (took 1s)                                                                                                    Finished in 0.94s
0.000597 - main.rs [INFO]  Raspberry Pi Pico init in main executor...

0.000314 - main.rs [INFO]  Initialized 'WATCHDOG'  as static shareable thread-safe ref

0.000338 - main.rs [INFO]  Started watchdog on feed scheduale of 3 s

0.000409 - main.rs [INFO]  Initializing Led { role: Onboard, output_level=Low }...

0.000471 - main.rs [INFO]  Initializing Led { role: Player1, output_level=Low }...

0.000513 - main.rs [INFO]  Initializing Led { role: Player2, output_level=Low }...

0.000583 - main.rs [INFO]  Initialized 'BUTTON_P1'  as static shareable thread-safe ref

0.000600 - main.rs [INFO]  Initialized 'BUTTON_P2'  as static shareable thread-safe ref

0.000631 - game.rs [INFO]  GAME mutex init.

0.000655 - main.rs [INFO]  OK for Game Singleton.

################ WAITING ################

0.000682 - game.rs [DEBUG]  Current GameState=Waiting, started=0 ms from boot with current-duration=0 ms

0.000819 - main.rs [INFO]  We are waiting! Resetting scores before next game

6.126485 - main.rs [INFO]  Press any button within the next 2 seconds to start the game...

8.126529 - main.rs [INFO]  Timeout! going through routine again.

12.600970 - button.rs [DEBUG]  Button 1 pressed

12.750974 - button.rs [DEBUG]  Button 1 released after 150 ms

14.252103 - main.rs [INFO]  Press any button within the next 2 seconds to start the game...

14.532480 - button.rs [INFO]  Player1 button pressed.

14.532510 - main.rs [INFO]  Player 1 button pressed, we can start the game!

14.532548 - game.rs [DEBUG]  Current GameState=Waiting, started=0 ms from boot with current-duration=14531 ms

14.532606 - game.rs [INFO]  Current state duration before transition=Waiting->Playing: 14531 ms

################ PLAYING ################

14.532656 - game.rs [INFO]  Transition finished: Game { state: Playing, state_start: Instant { ticks: 14532650 }, state_duration: Duration { ticks: 14531919 } }

14.532727 - main.rs [INFO]  We are playing!

14.532752 - main.rs [INFO]  Players get ready for round #0

14.532793 - led.rs [INFO]  Players get ready for round 0

17.732975 - led.rs [INFO]  Rng time for LED ON until shutoff for current game round: 4042 ms.

21.775029 - led.rs [INFO]  GO!

22.254500 - button.rs [INFO]  Player1 button pressed.

22.304557 - button.rs [INFO]  Player1 button released.

22.304586 - main.rs [INFO]  B1 was faster!

23.704803 - led.rs [DEBUG]  Blinking winner Led { role: Player1, output_level=Low } for current_score: 1

24.704890 - main.rs [INFO]  DINGINGINGING! Congratulations for Player1 with a response time of 479 ms

24.704929 - main.rs [INFO]  Current scores:

24.704955 - main.rs [INFO]  Player1: 1

24.704991 - main.rs [INFO]  Player2: 0

24.705023 - main.rs [INFO]  Target window for ressetting game with long button double press of 2s...

26.705060 - main.rs [INFO]  Players get ready for round #1

26.705089 - led.rs [INFO]  Players get ready for round 1

31.405226 - led.rs [INFO]  Rng time for LED ON until shutoff for current game round: 2871 ms.

34.276266 - led.rs [INFO]  GO!

34.825871 - button.rs [INFO]  Player2 button pressed.

34.875916 - button.rs [INFO]  Player2 button released.

34.875943 - main.rs [INFO]  B2 was faster!

36.276117 - led.rs [DEBUG]  Blinking winner Led { role: Player2, output_level=Low } for current_score: 1

37.276189 - main.rs [INFO]  DINGINGINGING! Congratulations for Player2 with a response time of 549 ms

37.276227 - main.rs [INFO]  Current scores:

37.276248 - main.rs [INFO]  Player1: 1

37.276281 - main.rs [INFO]  Player2: 1

37.276313 - main.rs [INFO]  Target window for ressetting game with long button double press of 2s...

39.276343 - main.rs [INFO]  Players get ready for round #2

39.276372 - led.rs [INFO]  Players get ready for round 2

45.476515 - led.rs [INFO]  Rng time for LED ON until shutoff for current game round: 4675 ms.

50.151552 - led.rs [INFO]  GO!

50.549276 - button.rs [INFO]  Player1 button pressed.

50.649336 - button.rs [INFO]  Player1 button released.

50.649361 - main.rs [INFO]  B1 was faster!

52.049532 - led.rs [DEBUG]  Blinking winner Led { role: Player1, output_level=Low } for current_score: 2

54.049618 - main.rs [INFO]  DINGINGINGING! Congratulations for Player1 with a response time of 447 ms

54.049655 - main.rs [INFO]  Current scores:

54.049675 - main.rs [INFO]  Player1: 2

54.049706 - main.rs [INFO]  Player2: 1

54.049739 - main.rs [INFO]  Target window for ressetting game with long button double press of 2s...

56.049769 - main.rs [INFO]  Players get ready for round #3

56.049798 - led.rs [INFO]  Players get ready for round 3

63.749952 - led.rs [INFO]  Rng time for LED ON until shutoff for current game round: 4295 ms.

68.044989 - led.rs [INFO]  GO!

68.612813 - button.rs [INFO]  Player1 button pressed.

68.712872 - button.rs [INFO]  Player1 button released.

68.712896 - main.rs [INFO]  B1 was faster!

70.113065 - led.rs [DEBUG]  Blinking winner Led { role: Player1, output_level=Low } for current_score: 3

73.113164 - main.rs [INFO]  DINGINGINGING! Congratulations for Player1 with a response time of 617 ms

73.113200 - main.rs [INFO]  Current scores:

73.113220 - main.rs [INFO]  Player1: 3

73.113260 - game.rs [DEBUG]  Current GameState=Playing, started=14532 ms from boot with current-duration=58580 ms

73.113314 - game.rs [INFO]  Current state duration before transition=Playing->ComputingResults: 58580 ms

73.113361 - game.rs [INFO]  Transition finished: Game { state: ComputingResults, state_start: Instant { ticks: 73113357 }, state_duration: Duration { ticks: 58580607 } }

################ COMPUTING RESULTS ################

73.113429 - main.rs [INFO]  Computing results for current game...

73.113481 - main.rs [INFO]  Winner Player1 had an avg response time of 514 ms (best time 447 ms, worst time 617 ms

82.114188 - game.rs [DEBUG]  Current GameState=ComputingResults, started=73113 ms from boot with current-duration=9000 ms

82.114247 - game.rs [INFO]  Current state duration before transition=ComputingResults->Finished: 9000 ms

82.114293 - game.rs [INFO]  Transition finished: Game { state: Finished, state_start: Instant { ticks: 82114290 }, state_duration: Duration { ticks: 9000824 } }

################ FINISH + RESET TO WAIT ################

82.114354 - main.rs [INFO]  Finished the game. Going back into waiting mode.

82.114377 - game.rs [DEBUG]  Current GameState=Finished, started=82114 ms from boot with current-duration=0 ms

82.114428 - game.rs [INFO]  Current state duration before transition=Finished->Waiting: 0 ms

82.114468 - game.rs [INFO]  Transition finished: Game { state: Waiting, state_start: Instant { ticks: 82114464 }, state_duration: Duration { ticks: 84 } }

82.114522 - main.rs [INFO]  We are waiting! Resetting scores before next game

88.240100 - main.rs [INFO]  Press any button within the next 2 seconds to start the game...

90.240133 - main.rs [INFO]  Timeout! going through routine again.

96.365704 - main.rs [INFO]  Press any button within the next 2 seconds to start the game...
```

### Reset with Both Buttons

[Demo of reset with both buttons](https://youtube.com/shorts/U9RWrXn6WUA?feature=share)

We lose the connection with `probe-rs` because of hardware reset:

```text
7.056322 - game.rs [INFO]  Transition finished: Game { state: Playing, state_start: Instant { ticks: 7056317 }, state_duration: Duration { ticks: 7055562 } }

7.056392 - main.rs [INFO]  We are playing!

7.056417 - main.rs [INFO]  Players get ready for round #0

7.056417 - main.rs [INFO]  Players get ready for round #0

7.056457 - led.rs [INFO]  Players get ready for round 0

10.256638 - led.rs [INFO]  Rng time for LED ON until shutoff for current game round: 3161 ms.

13.417690 - led.rs [INFO]  GO!

13.825521 - button.rs [INFO]  Player1 button pressed.

13.925594 - button.rs [INFO]  Player1 button released.

13.925621 - main.rs [INFO]  B1 was faster!

15.325841 - led.rs [DEBUG]  Blinking winner Led { role: Player1, output_level=Low } for current_score: 1

16.325924 - main.rs [INFO]  DINGINGINGING! Congratulations for Player1 with a response time of 457 ms

16.325961 - main.rs [INFO]  Current scores:

16.325985 - main.rs [INFO]  Player1: 1

16.326019 - main.rs [INFO]  Player2: 0

16.326050 - main.rs [INFO]  Target window for ressetting game with long button double press of 2s...

17.550975 - button.rs [DEBUG]  Button 1 pressed

17.600975 - button.rs [DEBUG]  Button 2 pressed

18.326083 - main.rs [INFO]  Players get ready for round #1

18.326112 - led.rs [INFO]  Players get ready for round 1

18.620985 - button.rs [INFO]  Both buttons held for 1+ second. Continuing to monitor for reset threshold...

20.620987 - button.rs [INFO]  Long press detected on both buttons (b1=3070 ms, b2=3020 ms). Resetting via watchdog...

23.026244 - led.rs [INFO]  Rng time for LED ON until shutoff for current game round: 2476 ms.

 WARN probe_rs::session: Could not clear all hardware breakpoints: An ARM specific error occurred.

Caused by:
    0: An error occurred in the communication with an access port or debug port.
    1: Target device did not respond to request.
 WARN probe_rs::session: Failed to deconfigure device during shutdown: Arm(Dap(NoAcknowledge))
 WARN probe_rs::architecture::arm::communication_interface: Failed to stop DP Multidrop(11002927)
Error: An ARM specific error occurred.

Caused by:
    0: Error using access port FullyQualifiedApAddress { dp: Multidrop(16787751), ap: V1(0) }.
    1: Failed to read register DRW at address 0xd0c
    2: An error occurred in the communication with an access port or debug port.
    3: Target device did not respond to request.
```

## Conclusion :checkered_flag:

All things considered, this project is a good example of using Rust for embedded programming. It utilizes Rust's core concepts such as traits, enumerations and structures, error handling and null values management (`Option` and `Result` enums) to create a simple yet fun game. The project also employs advanced concepts like asynchronous tasks and mutexes to manage concurrency and synchronization between different parts of the code.

I could have created more sophisticated abstractions to manage buttons and LEDs, particularly by using an interrupt system and implementing prioritization between tasks with finer control of mutexes. However, I favored simplicity and code readability for this project. The constants used for debounce times and wait delays could also be made configurable in a future version, thus offering more flexibility without sacrificing system robustness.
