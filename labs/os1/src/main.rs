// 不使用标准库std,去除相关库的依赖，而使用原生的core库（底层可没有这么多库支持）
#![no_std]
// 不使用传统的main函数
#![no_main]
// 这里如果使用的toolchain不是nightly的版本会报错，通过rust-tiikchain文件可以指定toolchain
#![feature(panic_info_message)] 

use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

// 导入外部的“entry.asm”文件，并作为内联汇编嵌入
// RISC-V会汇编指令
// http://www.360doc.com/content/21/0307/22/36367108_965698214.shtml
core::arch::global_asm!(include_str!("entry.asm"));

// no_mangle 关闭rust的名称修饰功能，不然名字不确定，entry中无法调用
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

// 对bss段进行清零操作
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe { (a as *mut u8).write_volatile(0)}
    });
}
