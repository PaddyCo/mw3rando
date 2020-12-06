    move.b d0, temp ; Temp
    move.l a0, temp2
    move.b (a0), d0    ; Get item from spot addr
    lsr #$01, d0
    cmpi.b #$20, d0             ; Make sure item is >= 0x20, as that means its a spell
    beq check_spot
    bpl check_spot
vanilla:
    move.b temp, d0              ; Retrieve temp
    move.l temp2, a0
    lea.l ($95be), a0
    cmp.b (a0, d0.w), d1
    rts
check_spot:
    cmpi.b #$26, d0             ; Make sure item is < 0x26, as that means its a spell
    bpl vanilla
    lea.l bought_spells_arr, a0
find_entry_loop:
    ;; d0 = item ID, d1 == current entry item id
    move.b (a0)+, d1
    cmp.b d0, d1
    beq check_shop
    tst.b d1             ; If 0x00, its the end of the array so we know item is not bought
    beq not_bought
next_entry_loop:
    add.l #$05, a0              ; Go to next entry
    bra find_entry_loop
check_shop:
    move.l a0, temp4
    include "find-current-store.asm"
    move.l temp4, a0
    add.l #$01, a0
    move.l (a0), d1
    cmp.l d0, d1              ; Check if base store address is the same
    bne next_entry_loop
    move.b #$01, d0
    tst.b d0
    rts
not_bought:
    move.b #$00, d0
    tst.b d0
    rts
    include "ram-map.asm"
