use std::fs;
use std::str;
use std::{thread, time};
use std::time::SystemTime;

mod d2s;

fn main() {
    let file_path = r"C:\Users\Martin\Saved Games\Diablo II Resurrected\Grail.d2s";
    // watch_file(file_path);
    let character = d2s::load_character(file_path);
    let s = character.signature();
    println!("Signature {:x?}", s);
    let v = character.version();
    println!("Version {:?}", v);
    let n = character.name();
    println!("Name {:?}", n);

    let current_checksum = character.checksum();
    let calculated_checksum = character.calculate_checksum();
    println!("Current checksum {:?}", current_checksum);
    println!("Calculated checksum {:?}", calculated_checksum);
}

fn get_modified_time(file_path: &str) -> SystemTime {
    let metadata = fs::metadata(file_path).unwrap();
    metadata.modified().unwrap()
}

fn watch_file(file_path: &str) -> ! {
    let mut modified = get_modified_time(file_path);
    loop {
        let new_modified = get_modified_time(file_path);
        if new_modified != modified {
            modified = new_modified;
            println!("{new_modified:?}");
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}
