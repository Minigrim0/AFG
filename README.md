# Project Readme

## Overview
This project is a game written in Rust using the Bevy game engine. The primary objective is to program bots to fight in an arena.
These bots can be programmed using either the assembly-like language `asmfg` or the higher-level programming language `afg`,
which compiles to `asmfg` using the provided compiler. The ultimate goal is to develop increasingly sophisticated bots capable
of winning in various situations.

## Running the Game
To run the game, you will need to have Rust and the Bevy game engine installed on your system. Follow these steps to get started:

1. Clone the repository:
   ```sh
   git clone https://github.com/Minigrim0/AFG.git AFG
   cd AFG
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
   cd compiler && cargo run -- -i your_input_file.afg -o your_output_file.asmfg [-s]
   ```

3. Place your bot code in the appropriate directory for the game to load it.

4. Run the game and test your bot's performance.

## Languages

### Machine specifications
The machine consists of;
* A stack, used to save function state, allocation of local variables, ... The stack is 256 integers long. (1KB)
* 2 registers (`GPA`, `GPB`). `GPA` holds the results of arithmetic operations anf `GPB` is used as a temporary register.
* The `FRV` register, which holds the data returns from function calls
* `CIP` the program counter
* `SBP` the base pointer for the stack (Anything higher comes from the caller, anything lower (down to `TSP`) is local to the function)
* `TSP` the top of the stack

### ASMFG
The assembly-like language used by the machine.

#### Syntax
* Registers are prefixed with `'`
* Literals are prefixed with `#`
* Special variables (memory addresses) are prefixed with `$`
* Comments start with `;` must be on their own lines and don't count in the jumps offsets

#### Basic instructions
| Instruction | operand 1 |  operand 2  | Description |
|-------------|-----------|-------------|-------------|
| `mov`       | reg/stk   | reg/imm/stk | Moves data from one register or an immediate value to a register. |
| `store`     | reg/imm   | reg/imm/stk | stores value of op2 into memory address op1 |
| `load`      | reg       | reg/imm/stk | loads address of op2 into register op1 |
| `add`       | reg       | reg/imm     | Adds op2 to op1 in place |
| `sub`       | reg       | reg/imm     | Subtracts op2 from op1 in place |
| `mul`       | reg       | reg/imm     | Multiplies op1 with op2 in place |
| `div`       | reg       | reg/imm     | Divides op1 by op2 in place |
| `cmp`       | reg       | reg/imm     | sub op2 from op1 and changes machine's flags accordingly |
| `jmp`       | reg/imm   |      /      | Jumps of the operand's offset |
| `jz`        | reg/imm   |      /      | Jumps of the operand's offset if the zero flag is set |
| `jnz`       | reg/imm   |      /      | Jumps of the operand's offset if the zero flag is not set |
| `jn`        | reg/imm   |      /      | Jumps of the operand's offset if the negtive flag is set |
| `jp`        | reg/imm   |      /      | Jumps of the operand's offset if the positive flag is set |
| `push`      | reg/imm   |      /      | Pushes the value of op1 onto the stack. |
| `pop`       | reg       |      /      | Pops a value from the stack into op1. |
| `call`      | imm       |      /      | Calls the function at the given offset |
| `ret`       |     /     |      /      | Returns from a function call using the address in the `RP` register. |

> Notes:
> `load` operation can only load data into a register. The address to load from must be in a register, an immediate value or an offset on the stack.
> `store` operation can store data from register, an immediate value or an offset on the stack. The memory address can be a register, an immediate value or an offset on the stack.
> All math operations are done in the registers or with a register and an immediate value. The result is stored in the first register.
> Popping from the stack is done into a register.

### afg
Details on the compiler are provided in the [COMPILER](./compiler/README.md) file.

Details about the programming language `afg` and its compilation process to `asmfg` will be provided here.

## Bot Sensors
Information about the different sensors available to bots and how they can be used to gather information about the game environment will be provided here.
