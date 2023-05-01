# RaspTank Plain

RaspTank software for basic demonstration of the Artefacts platform on a real albeit toyish robot.

The software is "plain" as it is self-contained and relies on no framework like ROS or YARP.

## Status

* Basic coverage of LEDs.
* Forward caterpillar motion.

## Setup

The project requires Rust and clang available on the host RPi:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    sudo apt-get install libclang-dev

The installation of `rustup` guides into installing a Rust version. A stable version is fine at this point.

### Build and run

The project is a plain Cargo project. Build and run achieved with:

    cargo run

Current settings do not tune permissions, so accessing GPIOs can require extra privilegies, requiring:

    sudo -E $HOME/.cargo/bin/cargo run

So the root user can execute the progra under proper conditions.
