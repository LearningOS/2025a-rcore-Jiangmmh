.altmacro               # 添加这句才能使用.rept
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps   # 声明两个外部符号，将__alltraps和__restore设置为函数
    .globl __restore
    .align 2            # 这里的2表示2的幂次方对齐，即四字节对齐

__alltraps:
    # 原型是csrrw rd, csr, rs 即将csr的值写到rd，然后将rs的值写到csr，这里表示交换sp和sscratch
    # 在执行到这条语句时，sp保存的是用户栈的栈顶，sscratch保存的是内核栈的栈顶
    # 因此这条指令的作用将用户栈切换为内核栈
    csrrw sp, sscratch, sp                      
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8                          # 32个通用寄存器，两个CSR寄存器，每个64位(8字节)
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus        # 将sstauts和sepc读取到t0和t1
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch       # 将用户栈栈顶取出到t2
    sd t2, 2*8(sp)          # 将用户占栈顶保存到内核栈
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp               # 让寄存器a0（trap_handler的第一个参数）指向内核栈，目的是获取用户程序传入的参数
    call trap_handler       # 跳转到trap_handler

# 从trap_handler返回之后，使用__restore恢复保存在内核栈上的Trap上下文
__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    sret
