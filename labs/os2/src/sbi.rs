#![allow(unused)]

// 对应着RISC-V的调用号
const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;
const SBI_SHUTDOWN: usize = 8;
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;

#[inline(always)]
fn syscall(id: usize, args: [usize;3]) -> usize{
    let mut ret;

    unsafe{
        // 关于内联汇编的使用
        // https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html
        core::arch::asm!(
            "ecall",                                    // 这里类似x86的syscall,进行系统调用，中断，然后陷入内核
            inlateout("x10") args[0] => ret,            // 这里是固定的函数传参，规定 x10放返回值 x11-x12放参数
            in("x11") args[1],                          // 详细的查看： https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md
            in("x12") args[2],                              
            in("x17") id,                               // x17寄存器放调用号
                                                        // 关于调用号在RISC-V找到了些资料
                                                        // https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#legacy-sbi-extension-extension-ids-0x00-through-0x0f
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> usize{
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}


pub fn sys_write(fd: usize, buffer: &[u8]) -> usize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn shutdown() -> !{
    syscall(SBI_SHUTDOWN, [0, 0, 0]);
    panic!("It should shutdown!");
}

pub fn console_putchar(c: usize){
    syscall(SBI_CONSOLE_PUTCHAR, [c, 0, 0]);
}

pub fn console_getchar() -> usize {
    syscall(SBI_CONSOLE_GETCHAR, [0, 0, 0])
}