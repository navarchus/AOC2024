use crate::util::filetools::parse_file;

pub fn question2(input: &String) -> usize {
    let mut f = parse_file(input);
    f.compact_files();
    f.checksum()
}