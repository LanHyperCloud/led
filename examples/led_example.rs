#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Output, Level, OutputConfig};
use esp_hal::timer::systimer::SystemTimer;
use esp_println as _;

#[cfg(feature = "led")]
use esp_led;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    // 配置GPIO8引脚为输出模式，用于控制板载LED蓝灯
    let led_pin = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    info!("GPIO8 LED configured");

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    #[cfg(feature = "led")]
    {
        // 示例1: 使用默认LED闪烁任务
        info!("Starting default LED blink task");
        esp_led::spawn_led_blink_task(&spawner, led_pin).unwrap();

        // 示例2: 使用自定义间隔的LED闪烁任务
        // let led_pin2 = Output::new(peripherals.GPIO9, Level::Low, OutputConfig::default());
        // esp_led::spawn_led_blink_task_custom(&spawner, led_pin2, 1000).unwrap();

        // 示例3: 使用高级LED闪烁任务（不同的开/关时间）
        // let led_pin3 = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());
        // esp_led::spawn_led_blink_task_advanced(&spawner, led_pin3, 200, 800).unwrap();

        // 示例4: 使用LED控制器进行手动控制
        // let mut led_controller = esp_led::LedController::new(led_pin);
        // led_controller.turn_on();
        // Timer::after(Duration::from_millis(1000)).await;
        // led_controller.blink_multiple(3, 200).await;
    }

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}