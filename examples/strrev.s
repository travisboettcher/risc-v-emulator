strrev:
    # s1 = str
    # a0 = sz
    # t0 = sz / 2
    # t1 = i
    # Enter stack frame
    addi    sp, sp, -16
    sw      ra, 0(sp)
    sw      s1, 8(sp)

    # Get the size of the string
    mv      s1, a0
    call    strlen
    srai    t0, a0, 1     # Divide sz by 2
    li      t1, 0         # i = 0
loop1:  # for loop
    bge     t1, t0, end1
    add     t2, s1, t1    # str + i
    sub     t3, a0, t1    # sz - i
    addi    t3, t3, -1    # sz - i - 1
    add     t3, t3, s1    # str + sz - i - 1
    lb      t4, 0(t2)     # str[i]
    lb      t5, 0(t3)     # str[sz - i - 1]
    sb      t4, 0(t3)     # swap
    sb      t5, 0(t2)
    addi    t1, t1, 1
    j       loop1
end1:
    # Leave stack frame
    lw      s1, 8(sp)
    lw      ra, 0(sp)
    addi    sp, sp, 16
    ret

strlen:
    li t0, 0
loop2:
    add t1, t0, a0
    lb t1, 0(t1)
    beqz t1, end2
    addi t0, t0, 1
    j loop2
end2:
    mv a0, t0
    ret
