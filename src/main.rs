//! A blinky-LED example application
//! This example uses Primer, a library for simple bare-metal ARM programming.

#![no_std]
#![no_main]
#![feature(alloc, collections)]
#![crate_type="staticlib"]
extern crate alloc;
#[macro_use] extern crate collections;

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate primer;

use primer::board::launchpad;
use primer::lm4f120h5qr::uart;
use core::fmt::Write;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

#[no_mangle]
pub extern "C" fn primer_start() {
    use alloc::boxed::Box;
    let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::SwapLFtoCRLF);
    launchpad::init();
    let mut loops = 0;
    loop {
        writeln!(uart, "Hello, world! Loops = {}", loops).unwrap();
        let heap_test = Box::new(42);
        writeln!(uart, "Heap test says {}", heap_test).unwrap();
        loops = loops + 1;
        launchpad::led_on(launchpad::Led::Red);
        primer::delay(250);
        launchpad::led_off(launchpad::Led::Red);
        primer::delay(250);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
