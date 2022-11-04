# Sega Controller Driver
![crates.io](https://img.shields.io/crates/v/sega-controller.svg)

Embedded driver for reading input from Sega controllers in Rust.

This library utilizes [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits as a platform-agnostic driver.

## Features

- `mega-drive` - includes Sega Mega Drive controllers
- `all` - includes all available features

## Example

```rust
use sega_controller::mega_drive::{MegaDriveButton, MegaDriveController};
use sega_controller::Error;

// Using some kind of hal like `arduino_hal`
// NOTE: You should have pull-up resistors on these pins (10k ohm)
let controller = MegaDriveController::from_pins(
    pins.d8.into_output(),          // select pin
    pins.d2.into_floating_input(),  // data pin 0
    pins.d3.into_floating_input(),  // data pin 1
    pins.d4.into_floating_input(),  // data pin 2
    pins.d5.into_floating_input(),  // data pin 3
    pins.d6.into_floating_input(),  // data pin 4
    pins.d7.into_floating_input(),  // data pin 5
);

// Only do this once every frame (16ms)
match controller.read_state() {
    Ok(state) => {
        if state.is_six_button {
            // do something special for six-button controllers if you like
        }
        
        if state.is_pressed(MegaDriveButton::Start) {
            // start button is currently held down
        }
    }
    Err(Error::NotPresent) => {} // controller is not connected
    _ => {}
}

```

## Hardware Reference

The Sega Mega Drive uses a standard DB9 serial port connector for controllers.

### Controller Connectors

```
      CONSOLE PORT (MALE)
 ,---------------------------,
 \  (1)  (2)  (3)  (4)  (5)  /
  \   (6)  (7)  (8)  (9)    /
   `-----------------------'

   CONTROLLER CABLE (FEMALE)
 ,---------------------------,
 \  (5)  (4)  (3)  (2)  (1)  /
  \   (9)  (8)  (7)  (6)    /
   `-----------------------'
```

**NOTE:** The controller cable uses a **female** connector, where the console has **male** connectors.

### Pin Mapping

| Pin | Description |    Mode    |
|:---:|:------------|:----------:|
|  1  | Data Bit 0  |   Input    |
|  2  | Data Bit 1  |   Input    | 
|  3  | Data Bit 2  |   Input    | 
|  4  | Data Bit 3  |   Input    | 
|  5  | **+5V VDC** |     --     | 
|  6  | Data Bit 4  |   Input    | 
|  7  | **Select**  | **Output** |
|  8  | **Ground**  |     --     | 
|  9  | Data Bit 5  |   Input    |

**NOTE:** The `Mode` is from the perspective of the console or microcontroller reading the controller.

## Credits

Thanks to [PlutieDev](https://plutiedev.com/controllers) documentation on how the Mega Drive controller works.
Especially useful for polling six-button controllers.

## TODO:

- [ ] Library Documentation via `mdbook` + GitHub Actions
- [ ] Hardware Documentation via `mdbook` + GitHub Actions
- [ ] Shift Register Support (Parallel-In)
- [ ] Sega Mouse Support
- [ ] Sega Multi-Tap Support
- [ ] Master System Support
- [ ] Unit Tests