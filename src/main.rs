#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(romulos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use romulos::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    romulos::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at {:?}", level_4_page_table.start_address());
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    romulos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    romulos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    romulos::test_panic_handler(info);
}