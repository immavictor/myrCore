// os/src/sbi.rs
// pub fn console_putchar(c: usize) {
//     #[allow(deprecated)]
//     sbi_rt::legacy::console_putchar(c);
// }
// pub fn shutdown(failure: bool) -> ! {
//     use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
//     if !failure {
//         system_reset(Shutdown, NoReason);
//     } else {
//         system_reset(Shutdown, SystemFailure);
//     }
//     unreachable!()
// }
// use core::arch::asm;
// const SBI_CONSOLE_PUTCHAR: usize = 1;
// const SBI_CONSOLE_GETCHAR: usize = 2;
// const SBI_SHUTDOWN: usize = 8;

// pub fn console_putchar(c: usize) {
//     sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
// }

// pub fn console_getchar() -> usize {
//     sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
// }

// pub fn shutdown() -> ! {
//     sbi_call(SBI_SHUTDOWN, 0, 0, 0);
//     panic!("It should shutdown!");
// }


// #![feature(global_asm)]
// #![feature(asm)]
// #![allow(unused)]
// #![allow(dead_code)]
// const SBI_SET_TIMER: usize = 0;
// const SBI_CONSOLE_PUTCHAR: usize = 1;
// const SBI_CONSOLE_GETCHAR: usize = 2;
// const SBI_CLEAR_IPI: usize = 3;
// const SBI_SEND_IPI: usize = 4;
// const SBI_REMOTE_FENCE_I: usize = 5;
// const SBI_REMOTE_SFENCE_VMA: usize = 6;
// const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
// const SBI_SHUTDOWN: usize = 8;
// use core::arch::asm;

// #[inline(always)]

// fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
//     let mut ret:usize;
//     unsafe {
//         // asm!( "ecall" : "={x10}" (ret) : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which) : "memory" : "volatile");
//         asm!(
//             "ecall",
//             inlateout("x10") arg0 => ret,
//             in("x11") arg1,
//             in("x12") arg2,
//             in("x17") which,
//         );
//     }
//     ret
// }

// pub fn console_putchar(c: usize) {
//     sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
// }

// pub fn console_getchar() -> usize {
//     sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
// }

// pub fn shutdown() -> ! {
//     sbi_call(SBI_SHUTDOWN, 0, 0, 0);
//     panic!("It should shutdown!");
// }
// os/src/sbi.rs
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}