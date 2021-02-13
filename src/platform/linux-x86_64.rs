#[inline(always)]
pub unsafe fn sys_call0(mut num: isize) -> isize {
    llvm_asm!("syscall"
         : "+{rax}"(num)
         :
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call1(mut num: isize, arg1: isize) -> isize {
    llvm_asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call2(mut num: isize, arg1: isize, arg2: isize) -> isize {
    llvm_asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call3(mut num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    llvm_asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call4(
    mut num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
) -> isize {
    llvm_asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3) "{r10}"(arg4)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call5(
    mut num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
) -> isize {
    llvm_asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3) "{r10}"(arg4) "{r8}"(arg5)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call6(
    mut num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    arg6: isize,
) -> isize {
    llvm_asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3) "{r10}"(arg4) "{r8}"(arg5) "{r9}"(arg6)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}
