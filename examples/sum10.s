x:
    .word 10

start:
    lw a0, x
    addi a0, a0, 10
    ret
