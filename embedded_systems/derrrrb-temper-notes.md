Just notes as I build the derrrb temper
===

Hardware
---
* Microcontroller Board: Arduino Uno R3
* Infrared Sensor: MLX90614

Software
---
* Program I wrote in Rust btw. [source code](https://github.com/antonio-hickey/derrrrb-temper)

Writing Rust for an embedded system
---
A Hardware Abstraction Layer (HAL) is required to run Rust on AVR microcontrollers
and other common boards. To use this we need to use the nightly Rust compiler.

We can't use Rust's standard library or even a main f(x) as we are running on bare metal (no Operating System).
```rust
#![no_std]
#![no_main]
```

Instead of using the `main` f(x) as the entry point for our program, we will use a macro from the Hardware Abstraction Layer:
```rust
#[arduino_hal::entry]
```
This will specify the programs entry point.

Importing rust compiled binary to the microcontroller
---
1. List open USB ports: `lsusb`
2. Set the serial com port for `ravedude`: `export RAVEDUDE_PORT=/dev/ttyUSB1`
3. Upload the program to the board's flash memory: `cargo run`

Soldering the MLX90614 sensor
---
yo0o pretty hard, actually did it so bad it short circuits my board when connected lol.

round 2, bought new sensor and did an ok enough of a job soldering it, so where no longer short circuiting the board :).


Connecting MLX90614 sensor to board
---
Using jumper cables and breadboard I connected the sensor.

Pins:
* MLX90614 SCL ->Microcontroller A5
    * The Serial Clock Line (SCL) is responsible for synchronizing the timing of data transmission between devices on the bus. It carries clock signals that dictate the rate at which data is transmitted and received. All devices connected to the bus follow this clock signal to ensure synchronized communication.
    * Analog 5 (A5) is just a analog input pin on the microcontroller

* MLX90614 SDA -> Microcontroller A4
    * Serial Data-Line Address (SDA) is a communication line used in serial bus protocols, such as I2C (Inter-Integrated Circuit) and SMBus (System Management Bus).
    * Analog 4 (A5) is just a analog input pin on the microcontroller

* MLX90614 GND -> Microcontroller GND
    * Ground (GND) is a reference point in an electrical circuit that is defined as having zero voltage. It serves as a common return path for electric current and is used as a reference for measuring voltage levels in a circuit.

* MLX90614 VCC -> Microcontroller 5V
    * Voltage Common Collector (VCC) refers to the voltage level provided to power the components or integrated circuits in a circuit. It is the positive voltage rail that serves as a reference for the circuit's operation. The VCC voltage is typically specified in the circuit's design or datasheet and is essential for proper functioning of the components.
    * 5 Volts (5V) is just that 5 Volts power supply on the microcontroller


Software abstraction for the MLX90614 sensor
---
* Crate: https://crates.io/crates/mlx9061x

So I found a Rust crate for my exact sensor, but quickly realizing the abstraction layer relies on 32 bit floats, and floating point operations on bare metal are not fun lol. 

Forking the MLX90614 Rust Crate
---
* After a day of hacky attempts to with floating point operations fighting against rust on bare metal, I decided to just fork the crate to make it only use unsigned 16 bit integers which is perfect for my use case.

    * original (using `f32`): 
    ```rust
    /// Read the object 1 temperature in celsius degrees
    pub fn object1_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TOBJ1)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }
    ```

    * using `u16`:
    ```rust
    pub fn object1_temperature(&mut self) -> Result<u16, Error<E>> {
        let t = self.read_u16(Register::TOBJ1)?;
        let t = (t * 2) / 100 - 273;
        Ok(t)
    }
    ```

* Another problem is this crate uses celsius and I'm :us: 'merican, so ya we converting celsius to fahrenheit:
```rust
/// Read the object 1 temperature in fahrenheit degrees
pub fn obj1_temp_f(&mut self) -> Result<u16, Error<E>> {
    let t = self.read_u16(Register::TOBJ1)?;
    let t_c = (t * 2) / 100 - 273);  // temp as celsius
    let t_f = (t_c * 9 / 5) + 32;   // temp as fahrenheit 
    Ok(t_f)
}
```

TODO
---
* Refactor the MLX90614 fork I created to flesh out unneeded features
* Look into using super sonic sensor to detect how far object is from the infrared sensor
* Display tempature using LCD or OLED display
* Build case for it (thinking custom 3d printed shell)
* _mass produce and get rich bro_
