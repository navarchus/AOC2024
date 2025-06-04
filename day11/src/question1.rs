use super::util;

pub fn solve(input: &String) -> usize {


    let mut stone_list = util::StoneList::new(input);

    for _ in 0..25 {
        stone_list.iterate(1000);
    }

    stone_list.stones.len()
}