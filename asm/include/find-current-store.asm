    lea.l curr_shop_addr_array, a0
find_store_loop:
    move.l (a0)+, d0
    tst.l d0
    beq find_store_loop
;; d0 is set to current store address
