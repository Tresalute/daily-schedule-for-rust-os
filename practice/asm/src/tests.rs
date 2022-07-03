#[test]
fn test_asm_one(){
    use core::arch::asm;
    let x:u64;
    unsafe{
        asm!("nop");
        asm!("mov {}, 5", out(reg) x);
    }

    assert_eq!(x, 5);
}

#[test]
fn test_asm_two(){
    use core::arch::asm;
    let i:u64 = 3;
    let o:u64;
    unsafe{
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );   
    }
    assert_eq!(o, 8);
}

#[test]
fn test_asm_thr(){
    use core::arch::asm;
    let mut i:u64 = 3;

    unsafe{
        asm!(
            "add {0}, 5",
            inout(reg) i,
        );
    }
    assert_eq!(i, 8);
}


#[test]
fn test_asm_four(){
    use core::arch::asm;
    let i:u64 = 3;
    let mut o:u64;
    unsafe{
        asm!(
            "add {0}, 5",
            inout(reg) i => o,
        );
    }
    assert_eq!(o, 8);
}

#[test]
fn test_asm_five(){
    use core::arch::asm;
    let cmd:u64 = 0xd1;
    unsafe{
        asm!(
            "out 0x64, eax",
            in("eax") cmd,
        )
    }
}