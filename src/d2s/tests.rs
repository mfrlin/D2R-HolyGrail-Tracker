use super::*;

#[test]
fn test() {
    let file_path = r"C:\Users\Martin\Downloads\d2s-master\examples\chars\99\Grail.d2s";
    let character = load_character_from_file(file_path);
    assert_eq!(format!("{:x}", character.signature()), "aa55aa55");
    assert_eq!(character.version() , 99);
    assert_eq!(character.size(), 2384);
    assert_eq!(character.checksum(), character.calculate_checksum());
    assert_eq!(character.hardcore(), false);
    assert_eq!(character.died(), true);
    assert_eq!(character.expansion(), true);
    assert_eq!(character.ladder(), false);
    assert_eq!(character.class(), "Sorceress");
    assert_eq!(character.name(), "Grail");
}
