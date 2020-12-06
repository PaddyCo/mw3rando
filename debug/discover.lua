item_names = {}
item_names[0x00] = "Legend Sword"
item_names[0x01] = "Excalibur"
item_names[0x02] = "Knight Sword"
item_names[0x03] = "Gradius"
item_names[0x04] = "Battle Spear"
item_names[0x05] = "Trident"
item_names[0x06] = "Small Spear"
item_names[0x07] = "Pygmy Sword"
item_names[0x08] = "Legend Armor"
item_names[0x09] = "Flame Armor"
item_names[0x0A] = "Knight Armor"
item_names[0x0B] = "Steel Armor"
item_names[0x0C] = "Hard Armor"
item_names[0x0D] = "Chain Mail"
item_names[0x0E] = "Leather Armor"
item_names[0x0F] = "Pygmy Armor"
item_names[0x10] = "Legend Shield"
item_names[0x11] = "Flame Shield"
item_names[0x12] = "Knight Shield"
item_names[0x13] = "Steel Shield"
item_names[0x14] = "Hard Shield"
item_names[0x15] = "Shell Shield"
item_names[0x16] = "Wood Shield"
item_names[0x17] = "Pygmy Shield"
item_names[0x18] = "Legend Boots"
item_names[0x19] = "Ceramic Boots"
item_names[0x1A] = "Oasis Boots"
item_names[0x1B] = "Marine Boots"
item_names[0x1C] = "Ladder Boots"
item_names[0x1D] = "Leather Boots"
item_names[0x1E] = "Cloth Boots"
item_names[0x1F] = "Pygmy Boots"
item_names[0x20] = "Fire Storm"
item_names[0x21] = "Quake"
item_names[0x22] = "Thunder"
item_names[0x23] = "Power"
item_names[0x24] = "Shield"
item_names[0x25] = "Return"
item_names[0x26] = "NONE"
item_names[0x27] = "Fire Storm?"
item_names[0x28] = "Ocarina"
item_names[0x29] = "Charm Stone"
item_names[0x2A] = "Elixir"
item_names[0x2B] = "Medicine"
item_names[0x2C] = "Potion"
item_names[0x2D] = "Holywater"
item_names[0x2E] = "Hi-Potion"
item_names[0x2F] = "Ocarina?"
item_names[0x30] = "Rod"
item_names[0x31] = "Lamp"
item_names[0x32] = "Amulet"
item_names[0x33] = "Sun-Key"
item_names[0x34] = "Moon-Key"
item_names[0x35] = "Star-Key"
item_names[0x36] = "Gold-Gem"
item_names[0x37] = "Blue-Gem"
item_names[0x38] = "Old Axe"
item_names[0x39] = "Fire-Urn"
item_names[0x3A] = "Bracelet"
item_names[0x3B] = "Rapid Pad"


s = manager:machine().screens[":megadriv"]
cpu = manager:machine().devices[":maincpu"]
mem = cpu.spaces["program"]

function rshift(a,disp)
  if disp < 0 then return lshift(a,-disp) end
  return math.floor(a % 2^32 / 2^disp)
end

function lshift(a,disp)
  if disp < 0 then return rshift(a,-disp) end
  return (a * 2^disp) % 2^32
end

CURRENT_ITEM_ADDR = 0xFF8CAE
START_SHOP_ADDR_RANGE = 0xFFA300
END_SHOP_ADDR_RANGE = 0xFFA338

START_ITEM_ADDR_RANGE = 0x1E1B0
END_ITEM_ADDR_RANGE = 0x1E258

SECOND_START_ITEM_ADDR_RANGE = 0x1E28D
SECOND_END_ITEM_ADDR_RANGE = 0x1E2B1

CURRENT_CHEST_ADDR = 0xFF9A02

last_item = 0
last_shop_addr = {}
last_chest_addr = {}
current_shop_addr = 0
current_chest_addr = 0
item_addr = {}

function set_item_addr(i)
  item = mem:read_u8(i+5)
  item_addr[item] = i+5
end

-- Get all item addresses
for i=START_ITEM_ADDR_RANGE-6, END_ITEM_ADDR_RANGE, 6 do
  set_item_addr(i)
end

for i=SECOND_START_ITEM_ADDR_RANGE-6, SECOND_END_ITEM_ADDR_RANGE, 6 do
  set_item_addr(i)
end

function check_item(item, a)
  return mem:read_u8(a) == lshift(item, 1)
end

function find_shop_item_offset(addr, item)
  for i=addr, addr+16, 1 do
    if i==addr and check_item(item, i) then
      return i-addr
    end

    if mem:read_u8(i) == 0x68 and check_item(item, i+1) then
      return (i+1)-addr
    end
  end

  return 0
end

function utf8_from(t)
  local bytearr = {}
  for _, v in ipairs(t) do
    local utf8byte = v < 0 and (0xff + v + 1) or v
    table.insert(bytearr, string.char(utf8byte))
  end
  return table.concat(bytearr)
end

function draw_hud()
  item = mem:read_u8(CURRENT_ITEM_ADDR)
  chest_addr = mem:read_u32(CURRENT_CHEST_ADDR)-2

  for i=START_SHOP_ADDR_RANGE, END_SHOP_ADDR_RANGE, 4 do
    shop_addr = mem:read_u32(i)

    if last_shop_addr[i] ~= shop_addr then
      last_shop_addr[i] = shop_addr

      if shop_addr ~= 0 then
        current_shop_addr = shop_addr
      end
    end
  end

  if current_shop_addr ~= 0 then
    current_shop_item_addr = current_shop_addr + find_shop_item_offset(current_shop_addr, item)
  end

  item_string = item_names[item]

  if current_shop_item_addr == nil then
    return
  end

  area = mem:read_u8(0xFF9668)
  room = mem:read_u8(0xFF9669)


  if item ~= last_item then
    print(string.format("[SHOP ITEM] %s | Item: [%x] %x | Shop: [Base: %x] [%x] %x | Area/Room: %x/%x", item_string or "Unknown", item_addr[item] or 0, item, current_shop_addr, current_shop_item_addr, lshift(item, 1), area, room))
  end

  if chest_addr ~= last_chest_addr then
    chest_item = mem:read_u8(chest_addr)
    print(chest_item);
    chest_item_string = item_names[(chest_item + 0xF80) - 0x1000]
    print(string.format("[CHEST ITEM] %s | [%x] %x | Area/Room: %x/%x", chest_item_string, chest_addr, chest_item, area, room))
  end


  last_item = item
  last_chest_addr = chest_addr
end

emu.register_frame_done(draw_hud, "frame")
