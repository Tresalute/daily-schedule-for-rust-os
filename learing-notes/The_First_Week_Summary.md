# 第一周汇总
## RISC-V汇编
### 寄存器
**通用寄存器：** X0 ~ x31，宽度对应位数（32/64）,总数为32个 
X0      ZERO        永远为0
**X1      RA          函数调用返回地址**
**x2      SP          堆栈指针**
x3      GP          全局指针
X4      TP          线程指针
X5-X7   T0-T2       临时寄存器
X8      S0/FP       保存寄存器/帧指针
X9      S1          保存寄存器
**X10-X11 A0-A1       函数参数/返回值**
**X12-X17 A2-A7       函数参数**
X18-X27 S2-S11      保存寄存器
X28-X31 T3-T6       临时寄存器

7个临时寄存器
12个保存寄存器
8个参数寄存器

**PC寄存器：**在RISC-V 中 PC 寄存器是单独存在的

### 汇编指令
**大致分为6种**
1. R类：寄存器与寄存器操作 
2. I类：立即数与访存load操作
    addi -> add + i(立即数)
    lw -> load word(加载 一个字节)
3. S类: 访存操作store
    sw -> store word(存储 一个字节)
4. B类：用于条件跳转
    beq -> b + eq (跳转 相等)
    bne -> b + ne (跳转 不相等)
5. U类：操作长立即数
    lbu -> l + b + u(加载 一个bit 无符号)
6. J类：无条件跳转

## Rust 代码技巧
### lazy_static
问题：需要一个全局的静态变量，又想在运行时初始化它
方案示例
```rust
#[macro_use]
extern crate lazy_static;

lazy_static!{
    static ref STATICVAR:usize = static_init_func();
}

fn static_init_func() -> usize{
    1
}

```
### Cell、RefCell
**Cell:**
**RefCell:**
### 指针
**Box:**
**Rc:**
**Arc:**
**Cow:**


## 系统 