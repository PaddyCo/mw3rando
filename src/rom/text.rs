use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::{Cursor, Seek, SeekFrom};
use std::str::FromStr;
use std::collections::HashMap;

static JUMP_START_ADDRESS: usize = 0x211da;
static JUMP_END_ADDRESS: usize = 0x213a9;
static DIALOGUE_START_ADDRESS: usize = 0x1dd94;
static WORD_START_ADDRESS: usize = 0x22026;
static WORD_END_ADDRESS: usize = 0x2245f;

#[derive(Debug)]
pub enum TextCommand {
    Text(String),
    Word(String),
    NewLine,
    WaitForInput,
    GiveItem(u8),
    SetItem(u8),
    SetTextSpeed(u8),
    SetTextIndent(u8),
    RawBytes(Vec<u8>),
    End,
}

use TextCommand::*;

#[derive(Debug)]
pub struct DialogueData {
    commands: Vec<TextCommand>,
}

#[derive(Debug)]
pub struct Dialogue<'a> {
    commands: Vec<TextCommand>,
    word_list: &'a HashMap<String, u8>
}

impl<'a> Dialogue<'a> {
    pub fn new(word_list: &'a HashMap<String, u8>) -> Dialogue<'a> {
        Dialogue {
            word_list: &word_list,
            commands: vec![],
        }
    }

    pub fn add(&mut self, command: TextCommand) -> &mut Dialogue<'a> {
        self.commands.push(command);
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Dialogue<'a> {
        // Sort words in word list by length
        let mut words: Vec<String> = self.word_list.keys().cloned().collect();
        words.sort_by(|a, b| Ord::cmp(&b.len(), &a.len()));

        let text = String::from_str(text).unwrap();
        let mut reader = Cursor::new(&text);
        let mut curr_str = String::new();

        while reader.position() < text.len() as u64 {
            let mut word: Option<String> = None;
            for w in &words {
                if text[reader.position() as usize..].starts_with(w) {
                    word = Some(w.to_string());
                    break;
                }
            }

            match word {
                Some(w) => {
                    if !curr_str.is_empty() {
                        self.add(Text(curr_str));
                        curr_str = String::new();
                    }
                    self.add(Word(w.clone()));
                    reader.seek(SeekFrom::Current(w.len() as i64));
                },
                None => match reader.read_u8().unwrap() as char {
                    '\n' => { self.add(NewLine); },
                    ch => { curr_str.push(ch); }
                }
            }
        }

        if !curr_str.is_empty() {
            self.add(Text(curr_str));
            curr_str = String::new();
        }

        self
    }

    pub fn word(&mut self, word: &str) -> &mut Dialogue<'a> {
        self.add(Word(word.to_string()));
        self
    }

    pub fn eol(&mut self) -> &mut Dialogue<'a> {
        self.add(NewLine);
        self
    }

    pub fn wait_for_input(&mut self) -> &mut Dialogue<'a> {
        self.add(WaitForInput);
        self
    }

    pub fn end(&mut self) -> &mut Dialogue<'a> {
        self.add(End);
        self
    }

    pub fn indent(&mut self, indent: u8) -> &mut Dialogue<'a> {
        self.add(SetTextIndent(indent));
        self
    }

    pub fn speed(&mut self, speed: u8) -> &mut Dialogue<'a> {
        self.add(SetTextSpeed(speed));
        self
    }

    pub fn set_item(&mut self, item: u8) -> &mut Dialogue<'a> {
        self.add(SetItem(item));
        self
    }

    pub fn give_item(&mut self, item: u8) -> &mut Dialogue<'a> {
        self.add(GiveItem(item));
        self
    }

    /// Add raw bytes to dialogue
    pub fn bytes(&mut self, bytes: Vec<u8>) -> &mut Dialogue<'a> {
        self.add(RawBytes(bytes));
        self
    }

    /// Shortcut for `.wait_for_input().end().build()`
    pub fn finish(&mut self) -> Vec<u8> {
        self.wait_for_input().end().build()
    }

    /// Returns the byte array for the dialogue
    pub fn build(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        for com in &self.commands {
            match com {
                Text(s) => {
                    for ch in s.bytes() {
                        bytes.push(ch as u8);
                    }
                },
                Word(s) => {
                    if self.word_list.contains_key(s) {
                        bytes.push(0x0c);
                        bytes.push(self.word_list[s]);
                    } else {
                        panic!("Tried to find word '{}' in word list, but it does not exist.", s);
                    }
                },
                NewLine => bytes.push(0x09),
                WaitForInput => bytes.push(0x05),
                SetItem(item) => bytes.append(&mut vec![0x0b, 0x0a, *item]),
                GiveItem(item) => bytes.append(&mut vec![0x0b, 0x08, *item]),
                RawBytes(b) => bytes.append(&mut b.clone()),
                SetTextSpeed(s) => bytes.append(&mut vec![0x03, *s]),
                SetTextIndent(i) => bytes.append(&mut vec![0x04, *i]),
                End => bytes.push(0x00),
            }
        }

        bytes
    }

    pub fn print(&mut self) -> &mut Dialogue<'a> {
        println!("{:#?}", &self.commands);

        self
    }
}

pub fn build_word_list(rom_data: &Vec<u8>) -> HashMap<String, u8> {
    let mut word_list: HashMap<String, u8> = HashMap::new();

    let mut reader = Cursor::new(&rom_data[JUMP_START_ADDRESS..JUMP_END_ADDRESS]);

    for i in 0..(JUMP_END_ADDRESS-JUMP_START_ADDRESS)/2 {
        let addr = (reader.read_u16::<BigEndian>().unwrap() as usize) + JUMP_START_ADDRESS;
        let mut word_reader = Cursor::new(&rom_data[addr..]);

        let mut word = String::new();
        loop {
            match word_reader.read_u8().unwrap() {
                0x00 => {
                    word_list.insert(word, i as u8);
                    break;
                }
                c => {
                    word.push(c as char);
                }
            }
        }
    }

    word_list
}

pub fn read_dialogue(rom_data: &Vec<u8>, start_address: usize, level: u8) {
    // Just trying to figure out how this stuff works
    let mut reader = Cursor::new(&rom_data[start_address..]);

    let mut indent = "".to_string();
    for i in 0..level {
        indent += "  ";
    }

    println!("{}ADDRESS: 0x{:x}", indent, start_address);
    loop {
        match reader.read_u8().unwrap() {
            0x00 => {
                println!("{}END", indent);
                if level == 0 {
                    println!("\n");
                }
                return;
            },
            0x03 => {
                let x = reader.read_u8().unwrap();
                println!("{}SET TEXT SPEED: {}", indent, x);
            },
            0x04 => {
                let x = reader.read_u8().unwrap();
                println!("{}INDENT: {}", indent, x);
            },
            0x05 => {
                println!("{}WAIT FOR INPUT", indent);
            },
            0x09 => {
                println!("{}NEW LINE", indent);
            },
            0x0a => {
                let x = reader.read_u8().unwrap();
                println!("{}SET INDENT FOR LINES: {}", indent, x)
            },
            0x0b => {
                let x = reader.read_u8().unwrap();
                match x {
                    0x0a => {
                        let y = reader.read_u8().unwrap();
                        println!("{}SET CURRENT ITEM {:x}", indent, y);
                    },
                    0x08 => {
                        let y = reader.read_u8().unwrap();
                        println!("{}REWARD ITEM {:x}", indent, y);
                    },
                    _ => {
                        println!("{}UNKNOWN: 0x0b, jumps to function at 15c8+{:x}", indent, x*2);
                    }
                }
            }
            0x0c => {
                let x = reader.read_u8().unwrap();
                let addr = get_jump_address(rom_data, x);
                println!("{}JUMP TO: 0x{:x}", indent, addr);
                read_dialogue(rom_data, addr, level+1);
            },
            0x20 => {
                println!("{}SPACE", indent);
            },
            v => {
                if v.is_ascii_alphabetic() || v.is_ascii_punctuation() {
                    println!("{}CHAR: {}", indent, v as char)
                } else {
                    println!("{}UNKNOWN: 0x{:x}", indent, v)
                }
            }
        }
    }
}

fn get_jump_address(rom_data: &Vec<u8>, offset: u8) -> usize {
    let offset = offset as usize;
    let offset = JUMP_START_ADDRESS + (offset*2);
    let mut reader = Cursor::new(&rom_data[offset..offset+2]);
    let addr = (reader.read_u16::<BigEndian>().unwrap() as usize) + JUMP_START_ADDRESS;

    addr
}

fn get_word(rom_data: &Vec<u8>, offset: u8) -> String {
    let offset = offset as usize;
    let offset = JUMP_START_ADDRESS + (offset*2);
    let mut reader = Cursor::new(&rom_data[offset..offset+2]);
    let mut word = String::new();

    let addr = (reader.read_u16::<BigEndian>().unwrap() as usize) + JUMP_START_ADDRESS;
    let mut reader = Cursor::new(&rom_data[addr..]);
    loop {
        match reader.read_u8().unwrap() {
            0x00 => break,
            c => {
                word.push(c as char)
            }
        }
    }

    word
}
