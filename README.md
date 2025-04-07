# rust-ant-plus
High level ANT+ protocal implementation.

## Table of contents
1. [Installation](#installation)
2. [Examples](#examples)
2. [Contribution](#contribution)
3. [Thanks to](#thanks-to)

## Installation
<i>Package not available as a crate.</i>

## Examples
1. Opening the Garming Stick 3.
    ```rust
    let stick = GarminStick3::new();

    if !stick.is_present() {
        println!("Garmin Stick 3 is not present.");
        return;
    }

    if !stick.open() {
        println!("Failed to open Garmin Stick 3.");
        return;
    }

    println!("Succesfully opened Garmin Stick 3!");
    ```

## Contribution
Contributions are always welcome through pr's.

## Thanks to...
- [Loghorn/ant-plus](https://github.com/Loghorn/ant-plus) (Typescript)