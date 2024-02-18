#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
// we have to define our own panic handler, PanicInfo parameter has the file and the line number where panic has occured
// the function should never return, so it is marked as a diverging function return never "!" type
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}

// this function doesn't return because it is the entry point and is called by the os or the bootloader
// , and thus should invoke the exit() system call of the os.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");

    loop {}
}

// linker is a program that combines the generated code into an executable
// the linker thinks that our program depends on the C runtime which is doesnt
// To solve the errors, we need to tell the linker that it should not include the C runtime. We can do this either by passing a certain set of arguments to the linker or by building for a bare metal target.


// after computer is turned on BIOS is loaded from some special flash memory in the motherboard -> Power on self test -> transfer control to bootloader
// bootloader determines the location of the kernel image on disk
// it also needs to switch the CPU from 16 bit real mode into 32 bit protected mode and then to 64 bit long mode
// it then queries some info such as memory map from BIOS and passes it to OS kernel