use std::fs;
use std::ops::Range;

pub fn load_character_from_file(file_path: &str) -> Character {
    Character {
        bytes: fs::read(file_path).unwrap(),
    }
}

pub fn save_character_to_file(file_path: &str, character: Character) {
    fs::write(file_path, &character.bytes).unwrap()
}

pub struct Character {
    pub bytes: Vec<u8>,
}

const HEADER_SIGNATURE: Range<usize> = 0..4;
const HEADER_VERSION: Range<usize> = 4..8;
const HEADER_SIZE: Range<usize> = 8..12;
const HEADER_CHECKSUM: Range<usize> = 12..16;
const HEADER_STATUS: usize = 36;
const HEADER_CLASS: usize = 40;
const HEADER_NAME: Range<usize> = 267..267+16;

fn CLASSES(class: u8) -> String {
    match class {
        0 => "Amazon".to_string(),
        1 => "Sorceress".to_string(),
        2 => "Necromancer".to_string(),
        3 => "Paladin".to_string(),
        4 => "Barbarian".to_string(),
        5 => "Druid".to_string(),
        6 => "Assassin".to_string(),
        _ => panic!(),
    }
}

impl Character {
    pub fn signature(&self) -> u32 {
        u32::from_le_bytes(self.bytes[HEADER_SIGNATURE].try_into().unwrap())
    }

    pub fn version(&self) -> u32 {
        u32::from_le_bytes(self.bytes[HEADER_VERSION].try_into().unwrap())
    }

    pub fn size(&self) -> u32 {
        u32::from_le_bytes(self.bytes[HEADER_SIZE].try_into().unwrap())
    }

    pub fn checksum(&self) -> u32 {
        u32::from_le_bytes(self.bytes[HEADER_CHECKSUM].try_into().unwrap())
    }

    pub fn calculate_checksum(&self) -> u32 {
        let mut checksum: u32 = 0;
        for (i, b) in self.bytes.iter().enumerate() {
            let mut byte = *b as u32;
            if HEADER_CHECKSUM.contains(&i) {
                byte = 0;
            }
            if checksum & 0x80000000 != 0 {
                byte += 1;
            }
            checksum = (checksum << 1) + byte;
        }

        checksum
    }

    pub fn hardcore(&self) -> bool {
        self._status(0b0100)
    }

    pub fn died(&self) -> bool {
        self._status(0b1000)
    }

    pub fn expansion(&self) -> bool {
        self._status(0b0010_0000)
    }

    pub fn ladder(&self) -> bool {
        self._status(0b0100_0000)
    }

    fn _status(&self, status: u8) -> bool {
        (self.bytes[HEADER_STATUS] & status) != 0
    }

    pub fn class(&self) -> String {
        CLASSES(self.bytes[HEADER_CLASS])
    }

    pub fn name(&self) -> String {
        let mut len = HEADER_NAME.len();
        for (i, c) in self.bytes[HEADER_NAME].iter().enumerate() {
            if *c == 0x0 {
                len = i;
                break;
            }
        }
        String::from_utf8_lossy(&self.bytes[HEADER_NAME.start..HEADER_NAME.start+len]).to_string()
    }
}