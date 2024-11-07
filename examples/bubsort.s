bubsort:
    # a0 = long *list
    # a1 = size
    # t0 = swapped
    # t1 = i
do: # do loop
    li t0, 0          # swapped = false
    li t1, 1          # i = 1
for: # for loop
    bge t1, a1, od    # break if i >= size
    slli t3, t1, 2    # scale i by 8 (for long)
    add t3, a0, t3    # new scaled memory address
    lw  t4, -8(t3)    # load list[i-1] into t4
    lw  t5, 0(t3)     # load list[i] into t5
    ble t4, t5, rof    # if list[i-1] < list[i], it's in position
    # if we get here, we need to swap
    li  t0, 1         # swapped = true
    sw  t4, 0(t3)     # list[i] = list[i-1]
    sw  t5, -8(t3)    # list[i-1] = list[i]
rof: # bottom of for loop body
    addi t1, t1, 1    # i++
    j    for           # loop again
od: # bottom of do loop body
    bnez t0, do       # loop if swapped = true
    ret               # return via return address register
