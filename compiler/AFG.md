# AFG Bot Programming Language

AFG is a simple, domain-specific programming language designed for controlling bots in a 2D game environment. It provides direct access to bot sensors and actuators through a straightforward syntax that's easy to learn and use.

## Table of Contents

- [Quick Start](#quick-start)
- [Language Basics](#language-basics)
- [Reserved Keywords](#reserved-keywords)
- [System Variables](#system-variables)
- [Control Structures](#control-structures)
- [Functions](#functions)
- [Bot Programming Patterns](#bot-programming-patterns)
- [Language Reference](#language-reference)

## Quick Start

Every AFG program starts with a `main()` function. Here's the simplest possible bot:

```afg
fn main() {
    set $Velocity[0] = 100;  // Move forward at speed 100
}
```

Most bots need continuous behavior, so you'll typically use an infinite loop:

```afg
fn main() {
    loop {
        set $Velocity[0] = 200;  // Keep moving forward
    }
}
```

## Language Basics

### Data Types

AFG keeps things simple with only **integers**. All values must be whole numbers, including negative numbers.

```afg
fn example() {
    set speed = 100;
    set distance = -50;     // Negative numbers work
    set total = speed + distance;
}
```

### Variables

All variables are **local to their function**. There are no global variables except system variables (those starting with `$`).

```afg
fn my_function() {
    set local_var = 42;     // Only exists in this function
    set $Velocity[0] = 100; // System variable - always accessible
}
```

### Arrays

Create and use arrays with square bracket notation (zero-indexed):

```afg
fn array_example() {
    set my_data[0] = 10;    // First element
    set my_data[1] = 20;    // Second element
    set value = my_data[0]; // Read from array
}
```

### Comments

Use `//` for single-line comments:

```afg
fn main() {
    // This is a comment
    set $Velocity[0] = 100;  // Move forward
}
```

## Reserved Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `fn` | Function definition | `fn move_forward() { ... }` |
| `set` | Variable assignment | `set speed = 100;` |
| `if` | Conditional statement | `if distance < 50 { ... }` |
| `while` | While loop | `while moving { ... }` |
| `loop` | Infinite loop | `loop { ... }` |
| `call` | Function call | `call turn_around();` |
| `return` | Return from function | `return angle;` |
| `print` | Debug output | `print value;` |

## System Variables

System variables give you access to your bot's sensors and controls. All start with `$`:

### Motion Control

| Variable | Description | Example |
|----------|-------------|---------|
| `$Velocity` | Forward/backward speed array | `set $Velocity[0] = 200;` |
| `$Moment` | Rotation speed (+ = left, - = right) | `set $Moment = 10;` |

### State Information

| Variable | Description | Example |
|----------|-------------|---------|
| `$Rotation` | Current rotation in degrees | `if $Rotation > 180 { ... }` |
| `$Position` | Current X,Y coordinates array | `set x = $Position[0];` |

### Sensors

| Variable | Description | Example |
|----------|-------------|---------|
| `$RayDist` | Distance to detected objects array | `if $RayDist[0] < 100 { ... }` |
| `$RayType` | Type of detected objects array (0 = nothing) | `if $RayType[0] != 0 { ... }` |

**Note**: The number of sensors depends on your bot class. Index 0 is typically the front-center sensor.

## Control Structures

### Conditional Statements

```afg
fn avoid_obstacle() {
    if $RayType[0] != 0 {
        // Something detected ahead
        set $Moment = -15;  // Turn right
    }
}
```

### While Loops

```afg
fn turn_until_clear() {
    set $Moment = 10;  // Start turning

    while $RayType[0] != 0 {
        // Keep turning while obstacle detected
    }

    set $Moment = 0;  // Stop turning
}
```

### Infinite Loops

```afg
fn main() {
    loop {
        // This runs forever
        call check_sensors();
        call move_forward();
    }
}
```

## Functions

### Defining Functions

```afg
fn calculate_distance(x1, y1, x2, y2) {
    set dx = x2 - x1;
    set dy = y2 - y1;
    return dx * dx + dy * dy;  // Simplified distance
}
```

### Calling Functions

Two ways to call functions:

```afg
// Using call keyword
call setup_bot();
call test_function(-1);  // Negative numbers work

// Direct assignment (captures return value)
set distance = get_sensor_reading();
set result = calculate_angle(90, 45);
```

### Return Values and Recursion

```afg
fn fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);  // Recursion works
}

fn main() {
    set result = fibonacci(5);
    print result;  // Prints 5
}
```

## Bot Programming Patterns

### Basic Obstacle Avoidance

```afg
fn main() {
    set $Velocity[0] = 200;

    loop {
        if $RayType[0] != 0 && $RayDist[0] < 150 {
            call avoid_obstacle();
        }
    }
}

fn avoid_obstacle() {
    set $Velocity[0] = 50;   // Slow down
    set $Moment = -20;       // Turn right

    while $RayType[0] != 0 {
        // Keep turning while obstacle detected
    }

    set $Moment = 0;         // Stop turning
    set $Velocity[0] = 200;  // Resume speed
}
```

### Sensor Scanning

```afg
fn find_closest_obstacle() {
    set closest_distance = 9999;
    set closest_sensor = -1;
    set i = 0;

    while i < 5 {  // Check 5 sensors
        if $RayType[i] != 0 && $RayDist[i] < closest_distance {
            set closest_distance = $RayDist[i];
            set closest_sensor = i;
        }
        set i = i + 1;
    }

    return closest_sensor;
}
```

### Precise Turning

```afg
fn turn_to_angle(target_angle) {
    set angle_diff = target_angle - $Rotation;

    // Normalize to -180 to 180 range
    while angle_diff > 180 {
        set angle_diff = angle_diff - 360;
    }
    while angle_diff < -180 {
        set angle_diff = angle_diff + 360;
    }

    // Turn in correct direction
    if angle_diff > 0 {
        set $Moment = 15;
    } else {
        set $Moment = -15;
    }

    // Turn until close to target
    while angle_diff > 5 || angle_diff < -5 {
        set angle_diff = target_angle - $Rotation;
    }

    set $Moment = 0;  // Stop turning
}
```

## Language Reference

### Syntax Summary

```afg
// Function definition
fn function_name(param1, param2) { ... }

// Variable assignment
set variable = expression;
set array[index] = value;

// Control structures
if condition { ... }
while condition { ... }
loop { ... }

// Function calls
call function_name(args);
set result = function_name(args);

// Return statement
return;
return value;

// Debug output
print value;
```

### Operators

| Operator | Type | Description | Example |
|----------|------|-------------|---------|
| `+` | Arithmetic | Addition | `set sum = a + b;` |
| `-` | Arithmetic | Subtraction | `set diff = a - b;` |
| `*` | Arithmetic | Multiplication | `set product = a * b;` |
| `/` | Arithmetic | Division | `set quotient = a / b;` |
| `%` | Arithmetic | Modulo | `set remainder = a % b;` |
| `<` | Comparison | Less than | `if distance < 100 { ... }` |
| `>` | Comparison | Greater than | `if speed > 200 { ... }` |
| `<=` | Comparison | Less than or equal | `if angle <= 90 { ... }` |
| `>=` | Comparison | Greater than or equal | `if distance >= 50 { ... }` |
| `==` | Comparison | Equal to | `if type == 0 { ... }` |
| `!=` | Comparison | Not equal to | `if type != 0 { ... }` |

### Coordinate System

- **Angles**: Measured in degrees, with 0° pointing "up" (north)
- **Distance**: Uses Bevy engine's coordinate system
- **Velocity**: Positive values move forward, negative move backward
- **Moment**: Positive values turn left, negative values turn right

### Language Features

- **Function calls**: Both `call function_name();` and `set result = function_name();` syntax
- **Recursion**: Functions can call themselves
- **Arrays**: User-defined arrays with bracket notation
- **Negative numbers**: Full support for negative integer literals and variables
- **Debug output**: `print` statement for development and testing

### Limitations

- **Integer-only**: No floating-point numbers or strings
- **Local variables only**: Except system variables starting with `$`
- **No error handling**: Programs halt on invalid operations
- **No built-in functions**: Only language constructs and system variables
