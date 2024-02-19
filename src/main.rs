#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
// we have to define our own panic handler, PanicInfo parameter has the file and the line number where panic has occured
// the function should never return, so it is marked as a diverging function return never "!" type
#[cfg(not(test))] 
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}


// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", _info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

// this function doesn't return because it is the entry point and is called by the os or the bootloader
// , and thus should invoke the exit() system call of the os.

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");


    #[cfg(test)]
    test_main();

    loop {}
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




// Test runner
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }



    exit_qemu(QemuExitCode::Success);

}


// Trivial test case
#[test_case]
fn trivial_assertion() {
    serial_println!("trivial assertion ....");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]

pub  enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}


pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
/* 
The functionality of the isa-debug-exit device is very simple. When a value is written to the I/O port specified by iobase, 
it causes QEMU to exit with exit status (value << 1) | 1. So when we write 0 to the port,QEMU will exit with exit status 
(0 << 1) | 1 = 1, and when we write 1 to the port, it will exit with exit status (1 << 1) | 1 = 3.
*/