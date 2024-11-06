strlen:
    li t0, 0
loop:
    add t1, t0, a0
    lb t1, 0(t1)
    beqz t1, end
    addi t0, t0, 1
    j loop
end:
    mv a0, t0
    ret
