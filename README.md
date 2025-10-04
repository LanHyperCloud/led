# ESP LED Control Library

A Rust library for controlling LEDs on ESP32 microcontrollers with embassy executor support.

## Features

- Simple LED on/off control
- Blinking tasks with customizable intervals
- LED controller with advanced control methods
- Embassy executor integration
- Support for different on/off durations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
esp-led = { git = "https://github.com/LanHyperCloud/led" }
```

## Examples

### Basic LED Blinking

```rust
use esp_led::spawn_led_blink_task;
use embassy_executor::Spawner;
use esp_hal::gpio::{Output, Level, OutputConfig};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let led_pin = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    
    // Start LED blinking task with default 500ms interval
    spawn_led_blink_task(&spawner, led_pin).unwrap();
}
```

### Custom Blinking Interval

```rust
use esp_led::spawn_led_blink_task_custom;
use embassy_executor::Spawner;
use esp_hal::gpio::{Output, Level, OutputConfig};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let led_pin = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    
    // Start LED blinking task with 1000ms interval
    spawn_led_blink_task_custom(&spawner, led_pin, 1000).unwrap();
}
```

### Advanced Blinking (Different On/Off Times)

```rust
use esp_led::spawn_led_blink_task_advanced;
use embassy_executor::Spawner;
use esp_hal::gpio::{Output, Level, OutputConfig};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let led_pin = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    
    // Start LED blinking task with 200ms on, 800ms off
    spawn_led_blink_task_advanced(&spawner, led_pin, 200, 800).unwrap();
}
```

### Manual LED Control

```rust
use esp_led::LedController;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Output, Level, OutputConfig};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let led_pin = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    
    let mut led_controller = LedController::new(led_pin);
    
    // Turn LED on
    led_controller.turn_on();
    Timer::after(Duration::from_millis(1000)).await;
    
    // Blink 3 times with 200ms duration
    led_controller.blink_multiple(3, 200).await;
    
    // Turn LED off
    led_controller.turn_off();
}
```

## API Reference

### Functions

- `spawn_led_blink_task(spawner, led)` - Start LED blinking with default 500ms interval
- `spawn_led_blink_task_custom(spawner, led, interval_ms)` - Start LED blinking with custom interval
- `spawn_led_blink_task_advanced(spawner, led, on_duration_ms, off_duration_ms)` - Start LED blinking with different on/off times

### LedController

- `new(led)` - Create a new LED controller
- `turn_on()` - Turn LED on
- `turn_off()` - Turn LED off
- `toggle()` - Toggle LED state
- `blink_once(duration_ms)` - Blink LED once
- `blink_multiple(count, duration_ms)` - Blink LED multiple times

## License

MIT