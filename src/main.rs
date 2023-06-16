use std::fs;
use std::str;
use std::{thread, time};
use std::time::SystemTime;

mod d2s;
mod overlay;


fn main() {

    //overlay::show_overlay();

    let file_path = r"C:\Users\Martin\Downloads\d2s-master\examples\chars\99\Grail.d2s";
    // watch_file(file_path);
    let character = d2s::load_character(file_path);
    println!("Signature {:x?}", character.signature());
    println!("Version {:?}", character.version());
    println!("Size {:?} bytes", character.size());
    println!("Current checksum {:?}", character.checksum());
    println!("Calculated checksum {:?}", character.calculate_checksum());
    println!("Hardcore {:?}", character.hardcore());
    println!("Died {:?}", character.died());
    println!("Expansion {:?}", character.expansion());
    println!("Ladder {:?}", character.ladder());

    println!("Name {:?}", character.name());
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
