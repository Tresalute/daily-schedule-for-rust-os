#![allow(unused)]


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
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
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