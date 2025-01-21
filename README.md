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
This is an assembly language for the AFG virtual machine. The machine consists of;
* A stack, used for function calls or for saving various data.
* 4 general purpose registers
* The `FRP` register, which holds the data returns from function calls
* `PC` the program counter
* `RP` the return pointer, set by the machine upon `call` invocations, for the function to know where to return to

#### Syntax
* Registers are prefixed with `'`
* Literals are prefixed with `#`
* Special variables (memory addresses) are prefixed with `$`
* Comments start with `;` must be on their own lines and don't count in the jumps offsets

#### Basic instructions
| Instruction | operand 1 | operand 2 | Description |
|-------------|-----------|-----------|-------------|
| `mov`       | reg       | reg/imm   | Moves data from one register or an immediate value to a register. |
| `add`       | reg       | reg/imm   | Adds op2 to op1 in place |
| `sub`       | reg       | reg/imm   | Subtracts op2 from op1 in place |
| `mul`       | reg       | reg/imm   | Multiplies op1 with op2 in place |
| `div`       | reg       | reg/imm   | Divides op1 by op2 in place |
| `load`      | reg       | reg/imm   | loads address of op1 into register op1 |
| `store`     | reg/imm   | reg/imm   | stores value of op2 into memory address op1 |
| `push`      | reg/imm   |     /     | Pushes the value of op1 onto the stack. |
| `pop`       | reg       |     /     | Pops a value from the stack into op1. |
| `call`      | imm       |     /     | Calls the function at the given offset |
| `ret`       |     /     |     /     | Returns from a function call using the address in the `RP` register. |
| `jmp`       | reg/imm   |     /     | Jumps of the operand's offset |
| `cmp`       | reg       | reg/imm   | sub op2 from op1 and changes machine's flags accordingly |
| `jz`        | reg/imm   |     /     | Jumps of the operand's offset if the zero flag is set |
| `jnz`       | reg/imm   |     /     | Jumps of the operand's offset if the zero flag is not set |
| `jn`        | reg/imm   |     /     | Jumps of the operand's offset if the negtive flag is set |
| `jp`        | reg/imm   |     /     | Jumps of the operand's offset if the positive flag is set |

### afg
Details about the programming language `afg` and its compilation process to `asmfg` will be provided here.

## Bot Sensors
Information about the different sensors available to bots and how they can be used to gather information about the game environment will be provided here.

## Contributing
Guidelines for contributing to the project will be provided here.

## License
Information about the project's license will be provided here.
