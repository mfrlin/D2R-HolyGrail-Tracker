use std::fs;
use std::ops::Range;

pub fn load_character(file_path: &str) -> Character {
    Character {
        raw: fs::read(file_path).unwrap(),
    }

}

pub struct Character {
    pub raw: Vec<u8>,
}

const HEADER_SIGNATURE: Range<usize> = 0..4;
const HEADER_VERSION: Range<usize> = 4..8;
const HEADER_SIZE: Range<usize> = 8..12;
const HEADER_CHECKSUM: Range<usize> = 12..16;

const HEADER_NAME: Range<usize> = 267..267+16;

impl Character {
    pub fn signature(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_SIGNATURE].try_into().unwrap())
    }

    pub fn version(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_VERSION].try_into().unwrap())
    }

    pub fn size(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_SIZE].try_into().unwrap())
    }

    pub fn checksum(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_CHECKSUM].try_into().unwrap())
    }

    pub fn calculate_checksum(&self) -> u32 {
        let mut checksum: u32 = 0;
        for (i, b) in self.raw.iter().enumerate() {
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

    pub fn name(&self) -> String {
        let mut len = HEADER_NAME.len();
        for (i, c) in self.raw[HEADER_NAME].iter().enumerate() {
            if *c == 0x0 {
                len = i;
                break;
            }
        }
        
        String::from_utf8_lossy(&self.raw[HEADER_NAME.start..HEADER_NAME.start+len]).to_string()
    }
}