//! Constants in the kernel

#[allow(unused)]

/// user app's stack size
pub const USER_STACK_SIZE: usize = 4096 * 2;        // 8KB
/// kernel stack size
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;      // 8KB
/// kernel heap size
pub const KERNEL_HEAP_SIZE: usize = 0x200_0000;     // 32MB

/// page size : 4KB
pub const PAGE_SIZE: usize = 0x1000;
/// page size bits: 12
pub const PAGE_SIZE_BITS: usize = 0xc;              // 对应4K大小的bits数量
/// the max number of syscall
pub const MAX_SYSCALL_NUM: usize = 500;
/// the virtual addr of trapoline
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;       // 地址空间最高页，内核和用户都有
/// the virtual addr of trap context
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;    // 用户地址空间次高页，存放TrapContex的页帧
/// clock frequency
pub const CLOCK_FREQ: usize = 12500000;
/// the physical memory end
pub const MEMORY_END: usize = 0x88000000;                       
