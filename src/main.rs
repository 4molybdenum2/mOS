#![no_std]
#![no_main]

use core::panic::PanicInfo;
// we have to define our own panic handler, PanicInfo parameter has the file and the line number where panic has occured
// the function should never return, so it is marked as a diverging function return never "!" type
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}

// this function doesn't return because it is the entry point and is called by the os or the bootloader
// , and thus should invoke the exit() system call of the os.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// linker is a program that combines the generated code into an executable
// the linker thinks that our program depends on the C runtime which is doesnt
// To solve the errors, we need to tell the linker that it should not include the C runtime. We can do this either by passing a certain set of arguments to the linker or by building for a bare metal target.
