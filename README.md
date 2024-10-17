# HYPED 2025

&nbsp;

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/Hyp-ed/hyped-2024/assets/43144010/12892983-b036-4ec3-b624-1c997f85bf94">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/Hyp-ed/hyped-2024/assets/43144010/54f3db17-be2b-4473-a963-b7d7d8c24a9a">
  <img alt="The HYPED logo." style="margin:100px" src="[https://user-images.githubusercontent.com/25423296/163456779-a8556205-d0a5-45e2-ac17-42d089e3c3f8.png](https://github.com/Hyp-ed/hyped-2024/assets/43144010/54f3db17-be2b-4473-a963-b7d7d8c24a9a)">
</picture>

## HYPED Software 2025

This repository contains the software for the HYPED 2025 pod. The software is divided into two main components:

### 1. Pod-side code

The pod-side code is responsible for controlling the pod's systems, including the motor controllers, sensors, and communication with the base station. The pod-side code is written in Rust and runs on the pod's STM32 microcontrollers.

### 2. Base station

The base station is responsible for monitoring the pod's systems, visualising data, and sending commands to the pod. The base station is written in TypeScript and runs on a computer connected to the pod via 2.4GHz radio antennas (Ubiquiti Rocket M2s).

## Dependencies

### Pod-side Code (Rust)

- [Rust](https://www.rust-lang.org/tools/install)
- [probe-rs](https://probe.rs/guide/getting-started/)

### Base Station (TypeScript)

- [Node.js](https://nodejs.org/en/download/)
- [pnpm](https://pnpm.io/installation)

Or run with Docker (recommended, see usage section):

- [Docker](https://docs.docker.com/get-docker/)

## Usage

### Pod-side Code

To flash some code to an STM32 microcontroller, first navigate to the `boards` directory and then to the board-specific directory. For example, to flash code to the `stm32f476rg` board, run the following commands:

```bash
cd boards/stm32f476rg
```

Then, to flash some particular code to the microcontroller, `bin/{your_code}.rs`, run the following command:

```bash
cargo run --bin {your_code}
```

### Base Station

## Repository Structure

Our repository is structured as follows:

- `telemetry/`: Contains the base station code for the Telemetry server and GUI.
- `lib/`: Contains the shared library code for the pod-side code and the base station.
  - `lib/core/`: Contains the core library code.
  - `lib/io/`: Contains our abstracted IO implementations.
  - `lib/sensors/`: Contains our sensor implementations.
- `boards/`: Contains the board-specific code for the pod-side code. These implementations are specific to each of the STM32 microcontrollers we are using due to differences in the peripherals available on each microcontroller. The board-specific code includes hardware implementations of the IO traits defined in `lib/io/`, tasks that can run on the board, as well as binaries that can be flashed to the board.
  - `boards/{board_name}/src/io`: The hardware implementations of the IO traits.
  - `boards/{board_name}/src/tasks`: The tasks that can run on the board.
  - `boards/{board_name}/src/bin`: The binaries that can be flashed to the board.

## Contributing

We have a GitHub Actions workflow that runs on every pull request to ensure that the code compiles and passes the tests. We also lint all of our code using `clippy` (Rust) and `eslint` (TypeScript) to ensure that the code style is consistent.
