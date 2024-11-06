arraysum:
    li t0, 0
    li t1, 0
loop:
    bge t1, a1, end
    mv t2, t1
    add t2, a0, t2
    lw t2, 0(t2)
    add t0, t0, t2
    addi t1, t1, 1
    j loop
end:
    mv a0, t0
    ret
