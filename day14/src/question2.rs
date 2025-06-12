use super::utils::{Robot, Quadrant};

fn print_bots(bots: &Vec<Robot>, bounds: &(usize, usize)) {
    for yidx in 0..bounds.1 {
        for xidx in 0..bounds.0 {
            let b_count = bots.iter().filter(|b| b.pos.x == xidx.try_into().unwrap() && b.pos.y == yidx.try_into().unwrap()).count();
            
            if b_count == 0 {
                print!(".")
            } else {
                print!("{b_count}");
            }
        }
        println!()
    }
}

fn calc_safety(bots: &Vec<Robot>, bounds: &(usize, usize)) -> usize{
    bots.iter().filter(|b| b.get_quadrant(*bounds).is_some() && b.get_quadrant(*bounds).unwrap() == Quadrant::Q1).count() *
    bots.iter().filter(|b| b.get_quadrant(*bounds).is_some() && b.get_quadrant(*bounds).unwrap() == Quadrant::Q2).count() *
    bots.iter().filter(|b| b.get_quadrant(*bounds).is_some() && b.get_quadrant(*bounds).unwrap() == Quadrant::Q3).count() *
    bots.iter().filter(|b| b.get_quadrant(*bounds).is_some() && b.get_quadrant(*bounds).unwrap() == Quadrant::Q4).count()
}

fn get_min_score(robots: &Vec<Robot>, bounds: &(usize, usize)) -> (usize, usize) {
    let (mut min_idx, mut min_score) = (0 as usize, usize::MAX);
    let mut bots  = robots.clone();
    for sec in 0..10000 {
        bots = bots.iter().map(|b: &Robot| b.step(*bounds)).collect();
        if calc_safety(&bots, &bounds) < min_score {
            min_score = calc_safety(&bots, &bounds);
            min_idx = sec;
        }
    }
    (min_idx, min_score)
}

pub fn solve(robots: &Vec<Robot>, bounds: (usize, usize), _steps: usize) {
    let bots: Vec<Robot> = robots.clone();
    let (min_idx, _min_score) = get_min_score(&bots, &bounds);

    //reset bots
    let mut bots: Vec<Robot> = robots.clone();
    for _ in 0..min_idx+1 {
        bots = bots.iter().map(|b: &Robot| b.step(bounds)).collect();
    }
    print_bots(&bots, &bounds);
    println!("sec: {}", min_idx+1);

}