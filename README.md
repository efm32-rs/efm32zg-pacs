# EFM32ZG (Zero Gecko) support for Rust

[![PACs](https://github.com/efm32-rs/efm32zg-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32zg-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32 series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32ZG chip has its own PAC, listed below:

| Crate            | Docs                                                                                   | crates.io                                                                                                   | target               |
|------------------|----------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|----------------------|
| `efm32zg108-pac` | [![docs.rs](https://docs.rs/efm32zg108-pac/badge.svg)](https://docs.rs/efm32zg108-pac) | [![crates.io](https://img.shields.io/crates/d/efm32zg108-pac.svg)](https://crates.io/crates/efm32zg108-pac) | `thumbv6m-none-eabi` |
 | `efm32zg110-pac` | [![docs.rs](https://docs.rs/efm32zg110-pac/badge.svg)](https://docs.rs/efm32zg110-pac) | [![crates.io](https://img.shields.io/crates/d/efm32zg110-pac.svg)](https://crates.io/crates/efm32zg110-pac) | `thumbv6m-none-eabi` |
 | `efm32zg210-pac` | [![docs.rs](https://docs.rs/efm32zg210-pac/badge.svg)](https://docs.rs/efm32zg210-pac) | [![crates.io](https://img.shields.io/crates/d/efm32zg210-pac.svg)](https://crates.io/crates/efm32zg210-pac) | `thumbv6m-none-eabi` |
 | `efm32zg222-pac` | [![docs.rs](https://docs.rs/efm32zg222-pac/badge.svg)](https://docs.rs/efm32zg222-pac) | [![crates.io](https://img.shields.io/crates/d/efm32zg222-pac.svg)](https://crates.io/crates/efm32zg222-pac) | `thumbv6m-none-eabi` |

## Device Reference Manuals from Silabs

**WIP**

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.