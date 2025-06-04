use std::{
    cmp::max,
    collections::HashMap,
    thread::{self, JoinHandle},
    vec,
};

#[derive(Debug, Clone)]
pub struct Stone {
    pub val: usize,
}

#[derive(Debug, Clone)]
pub struct StoneList {
    pub stones: Vec<Stone>,
    pub stone_map: HashMap<usize, usize>,
}

impl StoneList {
    pub fn iterate(&mut self, chunk_size: usize) {
        let chunk_size: usize = max(1, self.stones.len() / chunk_size);

        let chunks: std::slice::ChunksMut<'_, Stone> = self.stones.chunks_mut(chunk_size);

        let mut handles: Vec<JoinHandle<Vec<Stone>>> = vec![];

        for chunk in chunks {
            let mut curr_chunk = chunk.to_owned();
            handles.push(thread::spawn(move || {
                Self::iter_over_slice(&mut curr_chunk)
            }));
        }
        let mut res_vec: Vec<Stone> = vec![];
        for h in handles {
            let mut res_handle_vec = h.join().unwrap();
            res_vec.append(&mut res_handle_vec);
        }
        self.stones = res_vec;
    }

    fn iter_over_slice(sv: &mut Vec<Stone>) -> Vec<Stone> {
        let mut idx = 0;
        while idx < sv.len() {
            let curr_val = sv[idx].val;
            let curr_val_string = curr_val.to_string();

            if curr_val == 0 {
                sv[idx].val = 1;
            } else if curr_val_string.len() % 2 == 0 {
                let lhs = curr_val_string
                    .get(0..(curr_val_string.len() / 2))
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let rhs = curr_val_string
                    .get((curr_val_string.len() / 2)..)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                sv[idx].val = rhs;
                sv.insert(idx, Stone { val: lhs });
                idx += 2;
                continue;
            } else {
                sv[idx].val = sv[idx].val * 2024;
            }

            idx += 1;
        }
        sv.to_vec()
    }

    pub fn new(input: &String) -> Self {
        let mut list = StoneList {
            stones: Vec::new(),
            stone_map: HashMap::new(),
        };

        for s in input.split_whitespace() {
            list.stones.push(Stone {
                val: s.parse().unwrap(),
            });

            list.stone_map
                .entry(s.parse().unwrap())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        list
    }
}


