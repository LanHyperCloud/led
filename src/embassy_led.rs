//! Embassy executor integration for LED control

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;
use esp_println::println;

/// LED blinking task that runs continuously
#[embassy_executor::task]
pub async fn led_blink_task(mut led: Output<'static>) {
    loop {
        led.set_high();
        println!("LED ON");
        Timer::after(Duration::from_millis(500)).await;
        
        led.set_low();
        println!("LED OFF");
        Timer::after(Duration::from_millis(500)).await;
    }
}

/// LED blinking task with customizable interval
#[embassy_executor::task]
pub async fn led_blink_task_custom(mut led: Output<'static>, interval_ms: u64) {
    loop {
        led.set_high();
        println!("LED ON");
        Timer::after(Duration::from_millis(interval_ms)).await;
        
        led.set_low();
        println!("LED OFF");
        Timer::after(Duration::from_millis(interval_ms)).await;
    }
}

/// LED blinking task with different on/off intervals
#[embassy_executor::task]
pub async fn led_blink_task_advanced(
    mut led: Output<'static>, 
    on_duration_ms: u64, 
    off_duration_ms: u64
) {
    loop {
        led.set_high();
        println!("LED ON");
        Timer::after(Duration::from_millis(on_duration_ms)).await;
        
        led.set_low();
        println!("LED OFF");
        Timer::after(Duration::from_millis(off_duration_ms)).await;
    }
}

/// LED control functions
pub struct LedController {
    led: Output<'static>,
}

impl LedController {
    /// Create a new LED controller
    pub fn new(led: Output<'static>) -> Self {
        Self { led }
    }

    /// Turn LED on
    pub fn turn_on(&mut self) {
        self.led.set_high();
        println!("LED turned ON");
    }

    /// Turn LED off
    pub fn turn_off(&mut self) {
        self.led.set_low();
        println!("LED turned OFF");
    }

    /// Toggle LED state
    pub fn toggle(&mut self) {
        if self.led.is_set_high() {
            self.led.set_low();
            println!("LED toggled OFF");
        } else {
            self.led.set_high();
            println!("LED toggled ON");
        }
    }

    /// Blink LED once with specified duration
    pub async fn blink_once(&mut self, duration_ms: u64) {
        self.turn_on();
        Timer::after(Duration::from_millis(duration_ms)).await;
        self.turn_off();
    }

    /// Blink LED multiple times
    pub async fn blink_multiple(&mut self, count: u32, duration_ms: u64) {
        for _ in 0..count {
            self.blink_once(duration_ms).await;
            Timer::after(Duration::from_millis(duration_ms)).await;
        }
    }
}

/// Spawn LED blinking task with default settings
pub fn spawn_led_blink_task(spawner: &Spawner, led: Output<'static>) -> Result<(), SpawnError> {
    spawner.spawn(led_blink_task(led))?;
    println!("LED blink task started");
    Ok(())
}

/// Spawn LED blinking task with custom interval
pub fn spawn_led_blink_task_custom(
    spawner: &Spawner, 
    led: Output<'static>, 
    interval_ms: u64
) -> Result<(), SpawnError> {
    spawner.spawn(led_blink_task_custom(led, interval_ms))?;
    println!("LED blink task with custom interval started");
    Ok(())
}

/// Spawn LED blinking task with advanced settings
pub fn spawn_led_blink_task_advanced(
    spawner: &Spawner, 
    led: Output<'static>, 
    on_duration_ms: u64, 
    off_duration_ms: u64
) -> Result<(), SpawnError> {
    spawner.spawn(led_blink_task_advanced(led, on_duration_ms, off_duration_ms))?;
    println!("LED blink task with advanced settings started");
    Ok(())
}

/// Error type for LED operations
#[derive(Debug)]
pub enum SpawnError {
    SpawnFailed,
}

impl From<embassy_executor::SpawnError> for SpawnError {
    fn from(_: embassy_executor::SpawnError) -> Self {
        SpawnError::SpawnFailed
    }
}