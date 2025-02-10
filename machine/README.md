# AFG's Virtual Machine
![gif demo of afg](../.github/demo_machine.gif)

## Table of Contents
- [AFG's Virtual Machine](#afgs-virtual-machine)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Usage](#usage)
    - [Instruction list](#instruction-list)
    - [Stack](#stack)
    - [Registers](#registers)
    - [Output](#output)
    - [Status](#status)

## Introduction
This is a virtual machine that enable the bots in the game to run. It is a register & stack based machine.
The machine has 4 General purpose registers, 1 instruction pointer, 2 stack pointers (base and top, making a stack frame), 1 return value register.

Details on the assembly language can be found in the [compiler documentation](../compiler/README.md).

## Usage
To run the virtual machine, you can use the following command:
```bash
cargo run -- -i <assembly file> [--no-tui]
```

The `--no-tui` flag is optional and is used to disable the TUI interface. This means that the machine will run until completion, without any user interaction.
The output will be printed to the console.

The tui interface is used to visualize the machine's state at each step. It is useful for debugging and understanding the machine's state. It is divided into 5 sections.

### Instruction list
Shows the list of instructions in the program. You can move up and down the instructions using the arrow keys and the page up and page down keys.
To add a breakpoint, press `b`. To remove a breakpoint, press `b` at the same instruction.

The default mode for the machine is to run one instruction at a time, when the user presses the space bar. You can also run the machine continuously by pressing `c` (In this case, the machine will run until completion or until it hits a breakpoint).

When the cursor is on an instruction that might jump (with a literal offset), a line will appear pointing to the target instruction.

### Stack
The second section is the stack visualization. It shows the stack frame, with the base pointer at the bottom and the top pointer at the top. The stack grows downwards. (meaning the base will be displayed above the top of the stack).
For convenience, up to 4 values below the stack base pointer are labeled. This has no effective meaning (the values pointer might or might not be used in the current context).

### Registers
The third section shows the values of the registers. The registers are labeled as `GPA, `GPB`, `GPC, `GPD`, `CIP`, `TSP`, `SBP`, `FRV`. The values of the registers are displayed in hexadecimal.
This section also shows the current flags of the machine. The flags are `ZF`, `PF`, `NF`, `OF`, representing the zero, positive, negative, and overflow flags respectively.

### Output
This section will print the output of the machine (Those are the results of the `PRINT` instructions).

### Status
This section shows the status of the machine (Ready, Running, Completed, Error).
