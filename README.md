# Project Readme

## Overview
This project is a game written in Rust using the Bevy game engine. It draws inspiration from classical multiplayer FPS games but introduces a unique twist: the primary objective is to program bots. These bots can be programmed using either the assembly-like language `asmfg` or the higher-level programming language `afg`, which compiles to `asmfg` using the provided compiler. The ultimate goal is to develop increasingly sophisticated bots capable of winning in various situations.

## Getting Started
## Running the Game
To run the game, you will need to have Rust and the Bevy game engine installed on your system. Follow these steps to get started:

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/yourproject.git
   cd yourproject
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the game:
   ```sh
   cargo run --release
   ```

## Programming
To program the bots, you can use either the assembly-like language `asmfg` or the higher-level programming language `afg`. For more details on these languages, refer to the [Languages](#languages) section below.

1. Write your bot code in either `asmfg` or `afg`.

2. If you are using `afg`, compile it to `asmfg` using the provided compiler:
   ```sh
   cargo run --bin compiler -- -i your_input_file.afg
   ```

3. Place your bot code in the appropriate directory for the game to load it.

4. Run the game and test your bot's performance.

## Languages

### asmfg
Details about the assembly-like language `asmfg` will be provided here.

### afg
Details about the programming language `afg` and its compilation process to `asmfg` will be provided here.

## Bot Sensors
Information about the different sensors available to bots and how they can be used to gather information about the game environment will be provided here.

## Contributing
Guidelines for contributing to the project will be provided here.

## License
Information about the project's license will be provided here.
