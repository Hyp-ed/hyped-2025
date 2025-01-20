# HYPED 2025

&nbsp;

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/Hyp-ed/hyped-2024/assets/43144010/12892983-b036-4ec3-b624-1c997f85bf94">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/Hyp-ed/hyped-2024/assets/43144010/54f3db17-be2b-4473-a963-b7d7d8c24a9a">
  <img alt="The HYPED logo." src="[https://user-images.githubusercontent.com/25423296/163456779-a8556205-d0a5-45e2-ac17-42d089e3c3f8.png](https://github.com/Hyp-ed/hyped-2024/assets/43144010/54f3db17-be2b-4473-a963-b7d7d8c24a9a)">
</picture>

&nbsp;

![Build Shield](https://github.com/Hyp-ed/hyped-2025/actions/workflows/ci.yml/badge.svg) ![TODO Shield](https://img.shields.io/github/search/hyp-ed/hyped-2025/TODOLater?color=red&label=TODO%20counter)

## Software Architecture

This repository contains the software for the HYPED 2025 pod. The software is divided into two main components:

### 1. Pod-side Code

The pod-side code is responsible for controlling the pod's systems, including the motor & levitation controllers, sensors, localisation system, and communication with the base station. The pod-side code is written in Rust using [Embassy](https://embassy.dev/) and runs on the pod's STM32 microcontrollers.

We use the following microcontrollers on our pod:

- [STM32F767ZI](https://www.st.com/en/microcontrollers-microprocessors/stm32f767zi.html)
- [STM32L432](https://www.st.com/en/microcontrollers-microprocessors/stm32l432kc.html)

| Sub-system         | Microcontroller | Number |
| ------------------ | --------------- | ------ |
| Localisation       | STM32F767       | 1      |
| Telemetry          | STM32F767       | 1      |
| Levitation control | STM32L432       | 6      |
| Motor control      | TBD             | 1      |

All microcontrollers on our pod will communicate with each other (for sending sensor data, commands, logs, etc.) using [CAN](https://en.wikipedia.org/wiki/CAN_bus) (Controller Area Network).

### 2. Base Station (Telemetry)

The base station (aka Telemetry) is responsible for monitoring the pod's systems, visualising data, and sending commands to the pod. The Telemetry system is written in TypeScript and runs on a base station computer connected to the pod via 2.4GHz radio antennas ([Ubiquiti Rocket M2s](https://techspecs.ui.com/uisp/wireless/rocketm2)). (On the pod-side, the Telemetry board is connected to one of the Rockets and relays messages onto the other microcontrollers over CAN.) The [MQTT](https://mqtt.org/) IoT messaging protocol is used to transfer messages between the pod and the base station.

The base station consists of two main components:

- The Telemetry server, which communicates with the pod and serves the GUI.
- The Telemetry GUI, which visualises data and allows the user to send commands to the pod.

You can learn more about our Telemetry system on our Wiki [here](https://github.com/Hyp-ed/hyped-2025/wiki/What-is-Telemetry).

## Dependencies

### Pod-side Code (Rust)

- [Rust](https://www.rust-lang.org/tools/install)
- [probe-rs](https://probe.rs/guide/getting-started/)

Our Wiki contains a guide on how to get started with Rust and `probe-rs` [here](https://github.com/Hyp-ed/hyped-2025/wiki/Getting-Started-with-Rust).

### Base Station (Node.js)

- [Node.js](https://nodejs.org/en/download/)
- [pnpm](https://pnpm.io/installation)

Or if running with Docker (recommended, see usage section):

- [Docker](https://docs.docker.com/get-docker/)

For more details on how to set up a development environment for the Telemetry system, see our Wiki [here](https://github.com/Hyp-ed/hyped-2025/wiki/Telemetry-Development)

## Usage

### Pod-side

To flash some code to an STM32 microcontroller, first navigate to the `boards` directory and then to the board-specific directory. For example, to run code on the `stm32f767zi` board you would first change directory into `boards/stm32f767zi`.

Then, to flash some particular code to the microcontroller, `bin/{your_code}.rs`, run the following command:

```bash
cargo run --bin {your_code}
```

### Base Station (Telemetry)

> Note: The following should be run in the `telemetry` directory.

To run the telemetry system, run the following command:

```
./telemetry.sh <pnpm_script> -m
```

where `<pnpm_script>` is one of the scripts defined in `package.json`. E.g. `dev` or `build`. For a full list of possible arguments, run `./telemetry.sh --help`.

> The GUI will now be available on `http://localhost:5173`

## Repository Structure

Our repository is structured as follows:

- `telemetry/`: Contains the base station code for the Telemetry server and GUI.
- `lib/`: Contains the shared library code for the pod-side code and the base station.
  - `lib/core/`: Contains the core library code.
  - `lib/io/`: Contains our abstracted IO implementations and procedural macros for creating board-specific implementations.
  - `lib/sensors/`: Contains our sensor implementations.
- `boards/`: Contains the board-specific code for the pod-side code. These implementations are specific to each of the STM32 microcontrollers we are using due to differences in the peripherals available on each microcontroller. The board-specific code includes hardware implementations of the IO traits defined in `lib/io/`, tasks that can run on the board, as well as binaries that can be flashed to the board.
  - `boards/{board_name}/src/tasks`: The tasks that can run on the board.
  - `boards/{board_name}/src/bin`: The binaries that can be flashed to the board.

## Contributing

### Rust

We have a GitHub Actions workflow that runs on every pull request to ensure that the code compiles and passes tests. We also lint all of our code using `clippy`, which is a collection of lints to catch common mistakes, and check that the code is formatted correctly using `rustfmt`. We also check that everything is spelled correctly using `typos`.

### TypeScript

TypeScript code is linted using `ESLint` and formatted using `Prettier`. The configuration for these tools can be found in the `eslint-config` package and the `.prettierrc` file respectively. You can also find our global TypeScript configuration in the `tsconfig` package.

To run the linter and formatter, use the following commands:

- `pnpm lint` to check for lint errors, or `pnpm lint:fix` to automatically fix some issues.
- `pnpm format` to format the code, or `pnpm format:check` to check for formatting errors but not format.

Like our Rust code, we have a GitHub Actions workflow to check that these tools pass on every pull request.
