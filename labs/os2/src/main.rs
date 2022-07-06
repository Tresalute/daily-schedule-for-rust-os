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
mod trap;
mod sync;
mod batch;

// 导入外部的“entry.asm”文件，并作为内联汇编嵌入
// RISC-V会汇编指令
// http://www.360doc.com/content/21/0307/22/36367108_965698214.shtml
core::arch::global_asm!(include_str!("entry.asm"));

// no_mangle 关闭rust的名称修饰功能，不然名字不确定，entry中无法调用
#[no_mangle]
pub fn rust_main(){
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trap::init();
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
