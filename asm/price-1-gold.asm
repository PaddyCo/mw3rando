       ;;  Input: d0 - Item ID
       ;;  Output: d0 - Price of item in gold
       move.l #$000001, d0      ; Every item is 1 gold!
       rts
