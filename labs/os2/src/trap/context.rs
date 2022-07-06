use riscv::register::sstatus::{self, Sstatus, SPP};

// 如C一般进行内存对齐
#[repr(C)]
pub struct TrapContext{
    pub x: [usize;32], // 对应着32个寄存器
    pub sstatus: Sstatus, // 主要是为了获取|设置SSP字段
    pub sepc: usize,// 异常发生时的指令地址
}

impl TrapContext{
    pub fn set_sp(&mut self, sp: usize){
        self.x[2] = sp;
    }

    /// 这里主要是在初始化用户层堆栈
    /// 并且设置sepc
    /// 设置sepc后执行sret指令是便可以到指定地址
    /// sp指定所属堆栈，用于后面执行堆栈的恢复动作
    /// 这样达到了一个切换堆栈的功能
    pub fn app_init_context(entry: usize, sp: usize) -> Self{
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self{
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}