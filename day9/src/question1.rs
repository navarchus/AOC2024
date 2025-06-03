use crate::util::filetools::parse_file;

pub fn question1(input: &String) -> usize {
    let mut f = parse_file(input);
    f.compact_blocks();
    f.checksum()
}