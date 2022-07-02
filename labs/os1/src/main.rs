// 不使用标准库std,去除相关库的依赖
#![no_std]
#![no_main]
// 这里如果使用的toolchain不是nightly的版本会报错，通过rust-tiikchain文件可以指定toolchain
#![feature(panic_info_message)] 

use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main(){
    // 这里的函数都是从linker.ld中导出的
    // 用于后面测试输出使用
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("Hello, World!");
    trace!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, 
        boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe { (a as *mut u8).write_volatile(0)}
    });
}
