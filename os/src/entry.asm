    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top   # 将标签boot_stack_top的地址加载到栈顶指针寄存器sp
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound   # 初始化了一个64KB大小的启动栈
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: