    jsr $1c254                    ; Vanilla
    move.b #$0A, $FF99C5          ; Open up gate to Purapril from the get go
    move.b #$60, $FF9997          ; Queen Elnaora already talked to and Ocarina item spot is spawned
    lea.l bought_spells_arr, a0
reset_store_flags:
    move.w #$00, (a0)
    move.w #$00, (a0)+
    move.l #$00, (a0)+
    move.b (a0), d0
    tst.b d0
    beq done ; Loop until 0x00 is found
    bra reset_store_flags
done:
    rts
    include "ram-map.asm"
