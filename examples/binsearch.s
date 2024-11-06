binsearch:
    li t1, 0
    addi t2, a2, -1
loop:
    bgt t1, t2, end
    add t0, t1, t2
    srai t0, t0, 1
    mv t4, t0
    add t4, a0, t4
    lw t4, 0(t4)
    ble a1, t4, if
    addi t1, t0, 1
    j loop
if:
    bge a1, t4, endif
    addi t2, t0, -1
    j loop
endif:
    mv a0, t0
end:
    ret
