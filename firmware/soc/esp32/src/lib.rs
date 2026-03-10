#![no_std]

// re-export shared crates
pub use esp_hal;
pub use esp_println;
pub use esp_backtrace;
pub use esp_rtos;
pub use esp_storage;
pub use esp_radio;
pub use esp_alloc;
pub use esp_bootloader_esp_idf;


use common::log::*;

/// provide a less verbose panic handler
#[cfg(not(feature="esp-backtrace-panic"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // display location
    if let Some(location) = info.location() {
        error!(
            "PANIC at {} {}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }
    // display message
    error!("{}", info.message());

    loop {
        // wait for logging message to publish
        let delay = esp_hal::delay::Delay::new();
        delay.delay_millis(1000);

        // if release build, reset to resume mission
        #[cfg(not(debug_assertions))]
        esp_hal::system::software_reset()
    }
}

// provide 64K heap (reclaimed from bootloader)
pub fn init_heap() {
    use esp_hal::ram;
    esp_alloc::heap_allocator!(#[ram(reclaimed)] size: 64 * 1024);
}

// /// * initialize logging
// /// * initialize scheduler
// pub fn init(peripherals: esp_hal)
// {
//     // initialize logging
//     esp_println::logger::init_logger_from_env();
//     info!("initializing...");

//     // initialize the rtos scheduler
//     use esp_hal::timer::timg::TimerGroup;
//     let timg0 = TimerGroup::new(peripherals.TIMG0);
//     use esp_hal::interrupt::software::SoftwareInterruptControl;
//     let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
//     esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);
// }
