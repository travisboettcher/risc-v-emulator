li t0, 0
li t1, 0
bge t1, a1, 24
mv t2, t1
add t2, a0, t2
lw t2, 0(t2)
add t0, t0, t2
addi t1, t1, 1
j -28
mv a0, t0
ret
