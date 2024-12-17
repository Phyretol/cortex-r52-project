//! A uart-driver example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

use core::fmt::Write;

use kernel::dual_timer::*;
use kernel::uart_driver_solution::*;
use kernel::PERIPHERAL_CLOCK;

/// The entry-point to the Rust application.
///
/// It is called by the start-up assembly code in [`lib.rs`](./lib.rs) and thus
/// exported as a C-compatible symbol.
#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    if let Err(e) = main() {
        panic!("main returned {:?}", e);
    }
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> Result<(), core::fmt::Error> {
    let mut uart0 = unsafe { Uart::new_uart0() };
    uart0.enable(115200, PERIPHERAL_CLOCK);
    writeln!(uart0, "Hello, this is Rust!")?;

    let timer_value: u32 = 250_000_000;
    let print_interval= 50_000_000;
    let mut dual_timer = unsafe { DualTimer::new_dual_timer() };
    
    dual_timer.set_timer1(timer_value);
    writeln!(uart0, "Timer set to {timer_value}")?;
    dual_timer.enable_timer1();
    
    let mut print_value = timer_value - print_interval;
    loop {
        let current_value = dual_timer.get_timer1();
        if current_value < print_value {
            writeln!(uart0, "Current value {print_value}")?;
            print_value -= print_interval;
        }
        if current_value <= 0 {
            break;
        }
    } 

    writeln!(uart0, "Time's up!")?;

    // Print a multiplication square, using floating point
    for x in 1..=10 {
        for y in 1..=10 {
            let z = f64::from(x) * f64::from(y);
            write!(uart0, "{z:>8.2} ")?;
        }
        writeln!(uart0)?;
    }

    // Now crash the program
    panic!("I am a panic");
}

// End of file
