//! RustForge OS Kernel
//! 
//! A bare-metal x86_64 kernel with custom memory management,
//! interrupt handling, and process scheduling.

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod gdt;
mod interrupts;
mod memory;
mod scheduler;
mod serial;
mod vga;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("RustForge OS v0.1.0");
    println!("Initializing kernel...");

    // Initialize GDT
    gdt::init();
    println!("[OK] GDT initialized");

    // Initialize IDT
    interrupts::init_idt();
    println!("[OK] IDT initialized");

    // Initialize memory management
    unsafe {
        memory::init(boot_info);
    }
    println!("[OK] Memory management initialized");

    // Initialize scheduler
    scheduler::init();
    println!("[OK] Scheduler initialized");

    println!("Kernel initialization complete!");

    #[cfg(test)]
    test_main();

    println!("Entering idle loop...");
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
