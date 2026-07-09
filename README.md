# embedded-flex-pin

[![Crates.io](https://img.shields.io/crates/v/embedded-flex-pin.svg)](https://crates.io/crates/embedded-flex-pin)
[![docs.rs](https://docs.rs/embedded-flex-pin/badge.svg)](https://docs.rs/embedded-flex-pin)

A portable trait abstraction for GPIO pins that can switch between input and output
modes at runtime. Built on top of [`embedded-hal`](https://github.com/rust-embedded/embedded-hal).

## Traits

| Trait | Capability |
|-------|------------|
| `FlexPin` | Switch between input (with pull config) and push-pull output |
| `FlexInputOutput` | Open-drain input/output mode |
| `FlexInputOutputPull` | Open-drain I/O with internal pull resistors |

Each trait provides convenience variants (`_pull_up`, `_pull_down`, `_no_pull`,
`_default_speed`) so you don't need to pass configuration values when you know
exactly what you want.

## Example

```rust
use embedded_flex_pin::FlexPin;
use embassy_stm32::gpio::{Flex, Pull, Speed};

fn configure_led(pin: &mut impl FlexPin) {
    // Input with pull-down, then output at low speed
    pin.set_as_input_pull_down();
    pin.set_high();
    pin.set_as_output_default_speed();
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
