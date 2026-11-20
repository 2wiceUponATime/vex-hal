<!--
SPDX-FileCopyrightText: 2026 Gabe Schrock
SPDX-License-Identifier: MIT
-->

# vex-hal

A Rust runtime for VEX V5 and EXP robots.

## Example

```rust
#![no_std]
#![no_main]

use vex_hal::prelude::*;

#[vex_hal::main]
async fn main(_peripherals: Peripherals) {
    println!("Hello, World!");
}
```

## Resources

[vexide](https://vexide.dev/) is a useful resource to refer to because most of this crate's API is taken from it.

## Attribution

Files containing material derived from [vexide/vex-sdk](https://github.com/vexide/vex-sdk) are marked with a `SPDX-FileCopyrightText: 2025 vexide` header, while files containing material derived from [vexide/vexide](https://github.com/vexide/vexide) are marked with a `SPDX-FileCopyrightText: 2023 Gavin Niederman` header (both use an [MIT license](LICENSES/MIT.txt)).
