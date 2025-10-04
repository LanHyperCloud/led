#![no_std]

//! ESP LED Control Library
//! 
//! This library provides LED control functionality for ESP32 microcontrollers
//! with support for embassy executor tasks and various blinking patterns.
//! 
//! ## Features
//! 
//! - Simple LED on/off control
//! - Blinking tasks with customizable intervals
//! - LED controller with advanced control methods
//! - Embassy executor integration
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use esp_led::{LedController, spawn_led_blink_task};
//! use embassy_executor::Spawner;
//! use esp_hal::gpio::{Output, Level, OutputConfig};
//! 
//! #[embassy_executor::main]
//! async fn main(spawner: Spawner) {
//!     let peripherals = esp_hal::init(esp_hal::Config::default());
//!     let led_pin = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
//!     
//!     // Start LED blinking task
//!     spawn_led_blink_task(&spawner, led_pin).unwrap();
//! }
//! ```

#[cfg(feature = "embassy")]
pub mod embassy_led;

#[cfg(feature = "embassy")]
pub use embassy_led::*;