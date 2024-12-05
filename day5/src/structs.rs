#[derive(Debug)]
pub struct Rule {
    pub first: u64,
    pub second: u64,
}

impl Rule {
    pub fn is_correctly_ordered(&self, vals: &Vec<u64>) -> bool {
        //if first, then second cannot be before
        for (idx, item) in vals.iter().cloned().enumerate() {
            if item == self.first {
                //must come after, not before
                for curr_idx in idx..=0 {
                    if vals[curr_idx] == self.second {
                        return false;
                    }
                }
            }
        }

        //if second, then first cannot be after
        for (idx, item) in vals.iter().cloned().enumerate() {
            if item == self.second {
                //must come after, not before
                for curr_idx in idx..vals.len() - 1 {
                    if vals[curr_idx] == self.first {
                        return false;
                    }
                }
            }
        }

        true
    }
}
