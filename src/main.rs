#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mOS::println;

// this function doesn't return because it is the entry point and is called by the os or the bootloader
// , and thus should invoke the exit() system call of the os.

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");

    mOS::init();


    // trigger a page fault for a page which is not mapped to a physical page 
    // if double fault handler is not present it will trigger a triple fault 
    // which causes a system reset in most architectures

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };


    // x86_64::instructions::interrupts::int3();


    // trigger a stack overflow exception
    fn stackoverflow() {
        stackoverflow();
    }
    
    stackoverflow();

    #[cfg(test)]
    test_main();

    println!("It didn't crash...");

    loop {}
}


// we have to define our own panic handler, PanicInfo parameter has the file and the line number where panic has occured
// the function should never return, so it is marked as a diverging function return never "!" type
/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mOS::test_panic_handler(info)
}

/*
linker is a program that combines the generated code into an executable
the linker thinks that our program depends on the C runtime which is doesnt
To solve the errors, we need to tell the linker that it should not include the C runtime. We can do this either by passing a certain set of arguments to the linker or by building for a bare metal target.
*/

/*
after computer is turned on BIOS is loaded from some special flash memory in the motherboard -> Power on self test -> transfer control to bootloader
bootloader determines the location of the kernel image on disk
it also needs to switch the CPU from 16 bit real mode into 32 bit protected mode and then to 64 bit long mode
it then queries some info such as memory map from BIOS and passes it to OS kernel 
*/

