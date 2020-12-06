       ;;  Input: d0 - Item ID
       ;;  Output: d0 - Price of item in gold
       lsl #$2, d0               ; Calculate price list index
       move.l data(pc, d0.w), d0 ; Fetch price from price list below
       rts
data: 
       dc.l $00004D58, $000009C4, $000000FA, $0000000A, $00002710 ; Legend sword, Excalibur, Knight Sword, Gradius, Battle Spear
       dc.l $00001964, $00000050, $00002134, $00005CF8, $00002710 ; Trident, Small Spear, Pygmy Sword, Legend Armor, Flame Armor
       dc.l $00001388, $00000320, $000000DC, $00000046, $0000001E ; Knight Armor, Steel Armor, Hard Armor, Chain Mail, Leather Armor
       dc.l $00002710, $00004E20, $00001F40, $00000DAC, $000007D0 ; Pygmy Armor, Legend Shield, Flame Shield, Knight Shield, Steel Shield
       dc.l $00000096, $000000C8, $00000032, $00002EE0, $00002710 ; Hard Shield, Shell Shield, Wood Shield, Pygmy Shield, Legend Boots
       dc.l $00000FA0, $000004B0, $0000012C, $00000032, $0000001E ; Ceramic Boots, Oasis Boots, Marine Boots, Ladder Boots, Leather Boots
       dc.l $00000014, $00001388, $000001F4, $000001F4, $0000000A ; Cloth Boots, Pygmy Boots, Fire Storm, Quake, Thunder
       dc.l $000001F4, $000001F4, $000001F4, $FFFFFFFF, $FFFFFFFF ; Power, Shield, Return, NONE, Fire Storm(dup)
       dc.l $0000003E, $0000003D, $00000BB8, $0000000A, $00000032 ; Ocarina, Charmstone, Elixir, Medicine, Potion
       dc.l $0000003C, $0000003B, $FFFFFFFF, $00000039, $00000038 ; Holywater, Hi-potion, Ocarina(dup), Rod, Lamp
       dc.l $00000037, $00000036, $00000035, $00000034, $00000033 ; Amulet, Sun-key, Moon-key, Star-Key, Gold-Gem,
       dc.l $00000032, $00000031, $00000030, $0000002F, $0000002E ; Blue-Gem, Old Axe, Fire-Urn, Bracelet, Rapid Pad
