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

## Attribution

Files containing material derived from [vexide/vex-sdk](https://github.com/vexide/vex-sdk) are marked with a `SPDX-FileCopyrightText: 2025 vexide` header ([license](LICENSES/MIT.txt)).

Files containing material derived from [vexide/vexide](https://github.com/vexide/vexide) are marked with a `SPDX-FileCopyrightText: 2023 Gavin Niederman` header ([license](LICENSES/MIT.txt)).
