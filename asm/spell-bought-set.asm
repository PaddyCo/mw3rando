    move.l d0, temp
    move.l d1, temp2
    move.l a1, temp3
    move.l a0, temp4
    move.b curr_shop_item, d0
    cmpi.b #$20, d0             ; Make sure item is >= 0x20, as that means its a spell
    beq is_spell
    bpl is_spell
after:
    ;; Reset values
    move.l temp, d0
    move.l temp2, d1
    lea.l temp3, a1
    lea.l temp4, a0
    ;; Vanilla
    jsr $000085a8
    rts
is_spell:
    cmpi.b #$26, d0             ; Make sure item is < 0x26, as that means its a spell
    bpl after
    ;: Get last entry in bought spells array
    lea.l bought_spells_arr, a1
find_last_entry_loop:
    move.l (a1), d1
    tst.b d1
    beq set_item_as_bought    ; Loop until 0x00 is found
    add.l #$06, a1
    bra find_last_entry_loop
set_item_as_bought:
    ;; Set item as bought
    move.b (curr_shop_item), (a1)+
    move.b #$FF, (a1)+
    ;; Get current Store
    include "find-current-store.asm"
    move.l d0, (a1)
    bra after

    include "ram-map.asm"
