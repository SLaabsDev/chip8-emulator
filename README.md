Chip8 Emulator
======
**chip8-emulator** is a Rust implementation of the chip8 created to learn about the creation of emulators.

## Screenshots
![Pong](https://raw.githubusercontent.com/StevenLaabs/chip8-emulator/master/images/pong.png)
![UFO](https://raw.githubusercontent.com/StevenLaabs/chip8-emulator/master/images/ufo.png)

## Download
* [Current](https://github.com/StevenLaabs/chip8-emulator/archive/master.zip)
* No complete versions are available. This project is a work in progress.

## Build and Run
You can get the project using the download available above or by cloning with git using the following command:
```bash
$ git clone https://github.com/StevenLaabs/chip8-emulator.git
```

#### Dependencies
* [Rust](https://www.rust-lang.org/en-US/downloads.html)
* Cargo (comes with Rust)
* [SDL2.0 development libraries](https://www.libsdl.org/) (cargo will install rust-sdl2 automatically)

#### Build with Cargo
From the project directory run the following:
```bash
$ cargo build
$ cargo run
```

## Structure
This section explains the purpose of each file in the project.
```
chip8-emulator
│   README.md  - General information about the project.
│   Cargo.toml - Cargo file for project info and dependencies
│   Cargo.lock - Cargo generated file for details on packages and dependencies
│
└───src
│   │   main.rs   - Entry point for the program. Sets up SDL and runs the main loop for the chip8.
│   │   cpu.rs    - Emulates the chip8 cpu with registers and memory. Handles the opcodes.
│   │   keys.rs   - Handles key state and presses.
│   │   screen.rs - Holds the state of the pixels on the screen and handles drawing them.
│
└───rom - holds public domain rom files which can be played on a chip8
│
└───images - screenshots of the emulator
```
