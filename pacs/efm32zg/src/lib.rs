//! Peripheral access API for EFM32ZG microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32zg-pacs)
//!
//! This crate supports all EFM32ZG devices; for the complete list please see:
//! [efm32zg](https://github.com/efm32-rs/efm32zg-pacs/pacs/efm32zg)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32zg108")]
pub mod efm32zg108;

#[cfg(feature = "efm32zg110")]
pub mod efm32zg110;

#[cfg(feature = "efm32zg210")]
pub mod efm32zg210;

#[cfg(feature = "efm32zg222")]
pub mod efm32zg222;
