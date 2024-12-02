pub fn question1(parsed_data: &Vec<Vec<u32>>) {
    let safe_count = parsed_data
        .iter()
        .filter(|val| {
            let mut decrease = false;
            let mut increase = false;
            for el in val.into_iter().zip(val.into_iter().skip(1)) {
                if el.0.abs_diff(*el.1) < 1 || el.0.abs_diff(*el.1) > 3 {
                    return false;
                }

                if *el.0 < *el.1 {
                    decrease = true;
                }

                if *el.0 > *el.1 {
                    increase = true;
                }
            }
            return increase ^ decrease;
        })
        .count();

    println!("Safe: {}", safe_count);
}
