li t1, 0
addi t2, a2, -1
bgt t1, t2, 48
add t0, t1, t2
srai t0, t0, 1
mv t4, t0
add t4, a0, t4
lw t4, 0(t4)
ble a1, t4, 8
addi t1, t0, 1
j -36
bge a1, t4, 8
addi t2, t0, -1
j -48
mv a0, t0
ret
