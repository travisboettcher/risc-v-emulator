lb t0, 0(a1)
sb t0, 0(a0)
beqz t0, 12
addi a0, a0, 1
addi a1, a1, 1
j -24
