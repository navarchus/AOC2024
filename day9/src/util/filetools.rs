use std::fmt::{Debug};

pub struct PodBlock {
    id: Option<usize>,
}

pub struct PodFileSystem {
    data: Vec<PodBlock>,
}

impl Debug for PodFileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp: String = self.data.iter().fold(String::from(""), |i, g| match g.id {
            Some(val) => i + &format!("{}, ", val),
            None => i + "NONE, ",
        });

        tmp.pop();
        tmp.pop();

        write!(f, "{}", tmp)
    }
}

impl PodFileSystem {
    pub fn new(input: &String) -> Self {
        let mut new_f = Self { data: Vec::new() };

        //block ID
        let mut block_id: usize = 0;
        //assume all chars are nums
        for (idx, char) in input.char_indices() {
            let curr = char.to_digit(10).unwrap() as usize;
            if idx % 2 != 0 {
                // we are adding spaces
                for _ in 0..curr {
                    new_f.data.push(PodBlock { id: None });
                }
            } else {
                // append blocks with IDs
                for _ in 0..curr {
                    new_f.data.push(PodBlock { id: Some(block_id) });
                }
                block_id += 1;
            }
        }
        new_f
    }

    pub fn compact_blocks(&mut self) {
        let mut first_empty_idx: usize = self.data.iter().position(|b| b.id == None).unwrap();

        for idx in (0..self.data.len()).rev() {
            if self.data.get(idx).unwrap().id != None {
                //only swap if we need to
                if first_empty_idx < idx {
                    self.data.swap(idx, first_empty_idx);

                    while self.data.get(first_empty_idx).unwrap().id != None {
                        first_empty_idx += 1;
                    }
                }
            }
        }
    }

    //find start and end indices (non-inclusive) of first valid contiguous empty space location
    fn find_contiguous_space(data: &Vec<PodBlock>, min_size: usize) -> Option<(usize, usize)> {
        //find first empty idx and iter from there
        let mut start_idx: usize = data.iter().position(|b| b.id == None).unwrap();
        let mut end_idx: usize = start_idx;

        while start_idx < data.len() && end_idx < data.len() {
            while end_idx < data.len() && data[end_idx].id == None {
                end_idx += 1;
            }

            if (end_idx - start_idx) >= min_size {
                return Some((start_idx, end_idx));
            }

            start_idx = end_idx + 1;
            end_idx = start_idx;
        }

        None
    }

    pub fn compact_files(&mut self) {
        let data = &mut self.data;

        let mut end_idx = data.len();
        while end_idx > 0 {
            let mut start_idx: usize = end_idx - 1;

            while start_idx != 0
                && data[start_idx - 1].id == data[end_idx - 1].id
                && data[start_idx - 1].id != None
            {
                start_idx = start_idx - 1;
            }

            let size = end_idx - start_idx;
            let space_indices = Self::find_contiguous_space(data, size);
            if space_indices.is_some() {
                let (mut space_start_idx, space_end_idx) = space_indices.unwrap();
                if space_start_idx < start_idx {
                    for val_idx in start_idx..end_idx {
                        if space_start_idx >= space_end_idx {
                            panic!("Out of Bounds Error!")
                        }
                        data.swap(space_start_idx, val_idx);
                        space_start_idx += 1;
                    }
                }

            }

            end_idx = start_idx;
        }
    }

    pub fn checksum(&self) -> usize {
        let mut sum = 0;
        for (idx, b) in self.data.iter().enumerate() {
            if b.id != None {
                sum += idx * b.id.unwrap();
            }
        }
        sum
    }
}

pub fn parse_file(input: &String) -> PodFileSystem {
    PodFileSystem::new(input)
}
