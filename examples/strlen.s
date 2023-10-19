li t0, 0
add t1, t0, a0
lb t1, 0(t1)
beqz t1, 8
addi t0, t0, 1
j -20
mv a0, t0
