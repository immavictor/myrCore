// // os/src/main.rs
// #![no_std]
// #![no_main]
// #![feature(global_asm)]
// #![feature(asm)]
// #![feature(panic_info_message)]
// #[macro_use]
// mod console;
// mod lang_items;
// mod sbi;
// mod comlog;
// use log::{*};
// use core::arch::global_asm;

// global_asm!(include_str!("entry.asm"));

// fn clear_bss() {
//     extern "C" {
//         fn sbss();
//         fn ebss();
//     }
//     (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
// }
// #[no_mangle]
// pub fn rust_main() -> ! {
//     clear_bss();
//     comlog::init();
//     // log::set_max_level(LevelFilter::Info);
//     extern "C" {
//         fn stext();
//         fn etext();
//         fn srodata();
//         fn erodata();
//         fn sdata();
//         fn edata();
//         fn sbss();
//         fn ebss();
//         fn boot_stack();
//         fn boot_stack_top();
//     }    
//     error!("Hello, world!");
//     warn!("Hello, world!");
//     info!("Hello, world!");
//     debug!("Hello, world!");
//     trace!("Hello, world!");
//     println!("Hello, world!");
//     error!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
//     warn!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
//     info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
//     info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
//     error!(
//         "boot_stack [{:#x}, {:#x})",
//         boot_stack as usize, boot_stack_top as usize
//     );
//     println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
//     panic!("Shutdown machine!");
// }

#![feature(panic_info_message)]
#![no_std]
#![no_main]

#[macro_use]//用其他模块的宏
mod console;
mod lang_items;
mod logging;
mod sbi;

use core::arch::global_asm;
use log::{error,warn,info,debug,trace};
global_asm!(include_str!("entry.asm"));


#[no_mangle]
pub fn rust_main() -> ! {
    extern  "C" {
        fn stext();
        fn etext();
        
        fn srodata();
        fn erodata();


        fn sdata();
        fn edata();

        fn sbss();
        fn ebss();

        fn skernel();
        fn ekernel();
    }
    clear_bss();
    println!("Hello, world!");

    logging::init();
    // 打印内存布局
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    warn!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    debug!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    trace!(".kernel [{:#x}, {:#x})", skernel as usize, ekernel as usize);
    
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}





