use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use std::fs;
use rom::text::Dialogue;

mod rom;

static SET_PRICE_ENTRY_POINT: usize = 0x1da6;
static CHECK_STORE_ITEM_ENTRY_POINT: usize = 0x243e;
static START_ADDRESS: usize = 0xa5000;
static END_ADDRESS: usize = 0xbfff0;
static SET_SPELL_BOUGHT_ENTRY_POINT: usize = 0x1888;
static DIALOGUE_PICK_UP_ENTRY_POINT: usize = 0x1888;
static NEW_GAME_ENTRY_POINT: usize = 0x8eb4;

fn main() {
    let mut rom_data = match fs::read("./Wonder Boy in Monster World (USA, Europe).md") {
        Ok(v) => v,
        Err(e) => panic!("Could not read rom! Error: {}", e),
    };
    let address = START_ADDRESS;


    // SRAM patch (Better compability then EEPROM saving)
    rom_data[0x1b2] = 0xf8;
    rom_data[0x1b3] = 0x20;
    rom_data[0x1ba] = 0x03;
    rom_data[0x1bb] = 0xff;

    // Code section header
    let (_, address) = patch(&mut rom_data, address, vec![0x4d,0x57,0x33,0x52,0x41,0x4e,0x44,0x4f,0x20,0x43,0x4f,0x44,0x45,0x20,0x2d,0x20], vec![]);

    // TODO: Make sure store spot memory is persisted to sram on save
    // TODO: Make sure it's reset on new game
    let (start_addr, address) = patch(&mut rom_data, address, read_obj("item-bought-check"), vec![]);
    patch(&mut rom_data, CHECK_STORE_ITEM_ENTRY_POINT, read_obj("item-bought-check-entry"), vec![start_addr as u32]);

    //let (start_addr, address) = patch(&mut rom_data, address, read_obj("price-list"), vec![]);
    let (start_addr, address) = patch(&mut rom_data, address, read_obj("price-1-gold"), vec![]);
    patch(&mut rom_data, SET_PRICE_ENTRY_POINT, read_obj("jsr"), vec![start_addr as u32]);

    // NOTE: There can only be one kind of each spell in a store, so you can't have two fire storm in one store
    let (start_addr, address) = patch(&mut rom_data, address, read_obj("spell-bought-set"), vec![]);
    patch(&mut rom_data, SET_SPELL_BOUGHT_ENTRY_POINT, read_obj("jsr"), vec![start_addr as u32]);

    let (start_addr, address) = patch(&mut rom_data, address, read_obj("new-game"), vec![]);
    patch(&mut rom_data, NEW_GAME_ENTRY_POINT, read_obj("jsr"), vec![start_addr as u32]);

    // Starting area
    let elder_elixir: u8 = 0x22;
    let elder_fire_storm: u8 = 0x23;
    rom_data[0x1e07f] = elder_elixir;
    rom_data[0x1e08b] = elder_fire_storm;

    let leather_boots: u8 = 0x25;
    rom_data[0x1e239] = leather_boots;
    rom_data[0x22b08] = leather_boots << 1;

    // Fairy village
    let medicine: u8 = 0x40;
    rom_data[0x1e292] = medicine;
    rom_data[0x22b62] = medicine << 1;

    let spear: u8 = 0x3A;
    rom_data[0x1e1c1] = spear;
    rom_data[0x22b36] = spear << 1;

    let armor: u8 = 0x23;
    rom_data[0x1e1e5] = armor;
    rom_data[0x22b3c] = armor << 1;

    let shield: u8 = 0x39;
    rom_data[0x1e215] = shield;
    rom_data[0x22b42] = shield << 1;

    let cave_boss_chest: u16 = 0x02;
    rom_data[0xa752] = (((0x1000 + cave_boss_chest) - 0xf80) & 0xff) as u8;

    // Fairy Cave
    let ocarina: u8 = 0x28; // NOTE: Can't be spell or heart!
    rom_data[0x1e277] = ocarina; // String used in "Oh, it's X"
    rom_data[0x1e27c] = ocarina; // Item rewarded
    rom_data[0x27946] = ocarina << 1; // Which sprite is rendered and also which item to check in inventory

    // Purapril
    let excalibur: u8 = 0x20;
    rom_data[0x1e1af] = excalibur;
    rom_data[0x22b8a] = excalibur << 1;

    let steel_shield: u8 = 0x23;
    rom_data[0x1e203] = steel_shield;
    rom_data[0x22b84] = steel_shield << 1;


    // Jungle Cave
    let bat_reward: u8 = 0x33; // NOTE: Can't be spell or heart!
    rom_data[0x2ca09] = (((0x1000 + bat_reward as u16) - 0xf80) & 0xff) as u8; // Bat item drop
    rom_data[0x2ca05] = bat_reward; // Make sure bat doesn't respawn if you have item
    rom_data[0x2ccad] = bat_reward; // Make sure that bat door doesn't close behind you if you have item

    // Don't remove items from inventory (Stop them from re-appearing in stores etc)
    // TODO: Make sure this doesn't break anything
    patch(&mut rom_data, 0x24fa, read_obj("nop"), vec![]);

    // Remove demo timer check
    patch(&mut rom_data, 0x719e, read_obj("nop"), vec![]);


    let word_list = rom::text::build_word_list(&rom_data);

    // Intro text
    let text = Dialogue::new(&word_list).text("Let's go").finish();
    println!("{:x?} ({} bytes)", text.clone(), text.len());
    patch(&mut rom_data, 0x1df29, text.clone(), Vec::new());

    // Sonia item check
    // TODO: Make it refer to the actual item in the cave
    let text = Dialogue::new(&word_list).text("I lost my").eol().word("Oasis Boots").eol().text("in the cave").finish();
    patch(&mut rom_data, 0x1e731, text.clone(), Vec::new());

    // Purapril weapon store text
    let text = Dialogue::new(&word_list).indent(2).text("Please return with\nthe Oasis Boots").finish();
    patch(&mut rom_data, 0x1faaa, text, Vec::new());

    // Darkworld seal text
    let text = Dialogue::new(&word_list).text("Bracelet seal").finish();
    patch(&mut rom_data, 0x20f48, text.clone(), Vec::new());

    //rom::text::read_dialogue(&rom_data, 0x1f1ce, 0);

    match fs::write("mw3rando.smd", rom_data) {
        Ok(v) => v,
        Err(e) => panic!("Could not write file! Error: {}", e)
    };
}

fn read_obj(obj_name: &str) -> Vec<u8> {
    match fs::read(format!("./asm/obj/{}.o", obj_name)) {
        Ok(v) => v,
        Err(e) => panic!("Could not read obj file {}! Error: {}", obj_name, e),
    }
}

fn patch(rom_data: &mut Vec<u8>, address: usize, data: Vec<u8>, args: Vec<u32>) -> (usize, usize) {
    let mut skip = 0;
    for (i, byte) in data.iter().enumerate() {
        skip -= 1;
        if skip > 0 { continue; }

        let pos = address+i;

        // Look for magic hex value (feeded00) where 00 is the index in the args array to replace it with
        if data.len() >= i+3 && data[i] == 0xfe && data[i+1] == 0xed && data[i+2] == 0xed {
            let mut wtr = vec![];
            let arg = args[data[i+3] as usize];
            wtr.write_u32::<BigEndian>(arg).unwrap();
            rom_data[pos] = wtr[0];
            rom_data[pos+1] = wtr[1];
            rom_data[pos+2] = wtr[2];
            rom_data[pos+3] = wtr[3];
            skip = 4;
        } else {
            rom_data[pos] = *byte;
        }
    }

    (address, address+(data.len() as usize))
}
