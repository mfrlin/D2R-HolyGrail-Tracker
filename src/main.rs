use std::fs;
use std::str;
use std::ops::Range;
use std::{thread, time};
use std::time::SystemTime;

fn main() {
    let file_path = r"C:\Users\Martin\Saved Games\Diablo II Resurrected\Grail.d2s";
    let mut modified = get_modified_time(file_path);
    loop {
        let new_modified = get_modified_time(file_path);
        if new_modified != modified {
            modified = new_modified;
            println!("{new_modified:?}");
        }
        thread::sleep(time::Duration::from_millis(100));
    }
    let character = load_character(file_path);
    let s = character.signature();
    println!("Signature {:?}", s);
    let v = character.version();
    println!("Version {:?}", v);
    let n = character.name();
    println!("Name {:?}", n);
}

fn get_modified_time(file_path: &str) -> SystemTime {
    let metadata = fs::metadata(file_path).unwrap();
    metadata.modified().unwrap()
}

fn load_character(file_path: &str) -> Character {
    Character {
        raw: fs::read(file_path).unwrap(),
    }

}

struct Character {
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

const HEADER_NAME: Field = Field {
    offset: 267,
    size: 16,
};

impl Character {
    fn signature(&self) -> String {
        format!("{:x}", u32::from_le_bytes(self.raw[HEADER_SIGNATURE.range()].try_into().unwrap()))
    }

    fn version(&self) -> u32 {
        u32::from_le_bytes(self.raw[HEADER_VERSION.range()].try_into().unwrap())
    }

    fn name(&self) -> String {
        let mut len = HEADER_NAME.size;
        for (i, c) in self.raw[HEADER_NAME.range()].iter().enumerate() {
            if *c == 0 {
                len = i;
                break;
            }
        }
        
        String::from_utf8_lossy(&self.raw[HEADER_NAME.offset..HEADER_NAME.offset+len]).to_string()
    }

}
