.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch

# 保存curren_task的上下文，恢复next_task的上下文
# 在执行ret前，ra和sp指向的是next_task的返回指令地址和内核栈
__switch:
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,    该参数放在a0寄存器中
    #     next_task_cx_ptr: *const TaskContext      该参数放在a1寄存器中
    # )
    # save kernel stack of current task
    sd sp, 8(a0)   
    # save ra & s0~s11 of current execution
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    # restore ra & s0~s11 of next execution
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a1)
    ret

