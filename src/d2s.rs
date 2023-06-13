use std::fs;
use std::ops::Range;

pub fn load_character(file_path: &str) -> Character {
    Character {
        raw: fs::read(file_path).unwrap(),
    }

}

pub struct Character {
    raw: Vec<u8>,
}

struct Field {
    offset: usize,
    size: usize,
}

impl Field {
    fn range(&self) -> Range<usize> {
        self.offset..self.offset+self.size
    }
}

const HEADER_SIGNATURE: Field = Field {
    offset: 0,
    size: 4,
};

const HEADER_VERSION: Field = Field {
    offset: 4,
    size: 4,
};

const HEADER_CHECKSUM: Field = Field {
    offset: 12,
    size: 4,
};

const HEADER_NAME: Field = Field {
    offset: 267,
    size: 16,
};

impl Character {
    pub fn signature(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_SIGNATURE.range()].try_into().unwrap())
    }

    pub fn version(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_VERSION.range()].try_into().unwrap())
    }

    pub fn name(&self) -> String {
        let mut len = HEADER_NAME.size;
        for (i, c) in self.raw[HEADER_NAME.range()].iter().enumerate() {
            if *c == 0x0 {
                len = i;
                break;
            }
        }
        
        String::from_utf8_lossy(&self.raw[HEADER_NAME.offset..HEADER_NAME.offset+len]).to_string()
    }

    pub fn checksum(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_CHECKSUM.range()].try_into().unwrap())
    }

    pub fn calculate_checksum(&self) -> u32 {
        let mut checksum: u32 = 0;
        for (i, b) in self.raw.iter().enumerate() {
            let mut byte = *b as u32;
            if HEADER_CHECKSUM.range().contains(&i) {
                byte = 0;
            }
            if checksum & 0x80000000 != 0 {
                byte += 1;
            }
            checksum = (checksum << 1) + byte;
        }

        checksum
    }
}