# D2R HolyGrail Tracker

Diablo 2 Resurrected HolyGrail Tracker based on character save file parsing

### Disclaimer

Personal project to experiment and learn Rust language. Comments and PRs regarding code style, idioms, efficiency, etc. are welcome!

Reference for *.d2s file parsing is this parser written in JS: https://github.com/dschu012/d2s


## TODO:
- ~~Watch save file for changes~~
- ~~Check if save file is saved on unidentify item~~ YES
- Parse Unique and Set items
- Save found items (SQLite)
- Minimal GUI
- Create transparent overlay (Qt, Windows specific lib?)
- Stretch goal: Research Modding for coloring already found items when you see them on the ground in the game

# D2S File Format

I will try to bring https://github.com/krisives/d2s-format up to date here.

**Integers are stored in little endian byte order.**

| Offset | Type     | Desc
|--------|----------|------------
|0       |u32       | Signature (0xaa55aa55)
|4       |u32       | [Version](#versions)
|8       |u32       | Size of the file in bytes
|12      |u32       | [Checksum](#checksum)
|16      |N/A       | Unknown
|36      |u8        | [Character Status](#character-status)
|267     |[u8, 16]  | [Character Name](#character-name)

### Versions

Save file version. Current version of the game uses `99`. In the future if version changes File Format should be versioned and changes documented.

* `<98` is pre D2R
* `98` is version ??
* `99` is current version `1.6.74264`

### Checksum

Needs to be re-calculated when editing save file otherwise it will not be valid anymore. Credit for algorithm goes to whoever reverse engineered it.

### Character Status

8-bit field of flags for statuses 

Bit | Desc
----|------
0   | ?
1   | ?
2   | Hardcore
3   | Died
4   | ?
5   | Expansion
6   | Ladder
7   | ?

### Character Name

Character names are stored as an array of 16 characters which contain
a null terminated string padded with `0x00` for the remaining bytes.
Characters are stored as 8-bit ASCII, but remember that valid names must
follow these rules:
 * Must be 2-15 in length
 * Must begin with a letter
 * May contain up to one hyphen (`-`) or underscore (`_`)



