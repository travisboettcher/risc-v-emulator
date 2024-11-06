bubsort:
loop1:
    li t0, 0
    li t1, 1
loop2:
    bge t1, a1, end1
    addi t3, t1, 0
    add t3, a0, t3
    lw t4, -1(t3)
    lw t5, 0(t3)
    ble t4, t5, end2
    li t0, 1
    sw t4, 0(t3)
    sw t5, -1(t3)
end2:
    addi t1, t1, 1
    j loop2
end1:
    bnez t0, loop1
    ret
