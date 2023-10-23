addi sp, sp, -16
sw ra, 0(sp)
sw s1, 8(sp)
mv s1, a0
call 72
srai t0, a0, 1
li t1, 0
bge t1, t0, 40
add t2, s1, t1
sub t3, a0, t1
addi t3, t3, -1
add t3, t3, s1
lb t4, 0(t2)
lb t5, 0(t3)
sb t4, 0(t3)
sb t5, 0(t2)
addi t1, t1, 1
j -44
lw s1, 8(sp)
lw ra, 0(sp)
addi sp, sp, 16
ret
li t0, 0
add t1, t0, a0
lb t1, 0(t1)
beqz t1, 8
addi t0, t0, 1
j -20
mv a0, t0
ret
