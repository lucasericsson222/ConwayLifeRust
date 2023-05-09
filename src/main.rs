use std::{mem};

use rand::{rngs::ThreadRng, RngCore};

fn main() {
    let mut rng = rand::thread_rng();
    let mut world = vec![vec!['.'; 20]; 20];
    
    random_generate(&mut world, &mut rng);

    display_world(&world);
    
    loop {
        run_step(&mut world);
        display_world(&world);
        let mut _line: String = Default::default();
        std::io::stdin().read_line(&mut _line).unwrap();
    }
}

fn random_generate(world: &mut Vec<Vec<char>>, rng: &mut ThreadRng) {
    for i in 0..20 {
        for j in 0..20 {
            if (rng.next_u64() % 2) == 0 {
                world[i][j] = '#';
            }
        }
    }
}

fn display_world(world: &Vec<Vec<char>>) {
    for i in 0..20 {
        for j in 0..20 {
            print!("{} ", world[i][j]);
        }
        println!("");
    }
}

fn run_step(world: &mut Vec<Vec<char>>) {
    let mut new_world = vec![vec!['.'; 20]; 20];
    for i in 0..20 {
        for j in 0..20 {
            apply_cell_life(&mut new_world, &world, i, j);
        }
    }
    mem::swap(world, &mut new_world);
}

fn apply_cell_life(out: &mut Vec<Vec<char>>, source: &Vec<Vec<char>>, i: usize, j: usize) {
    let is_alive = source[i][j] == '#';
    let num_neighbors = calc_num_neighbors(source, i, j);
    out[i][j] = source[i][j];
    if is_alive {
        if num_neighbors < 2 || num_neighbors > 3 {
            out[i][j] = '.';
        }
    } else {
        if num_neighbors == 3 {
            out[i][j] = '#';
        }
    }
}

fn calc_num_neighbors(source: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut num = 0;
    if (i == 0 || j == 0 || i >= 20 || j >= 20) {
        return 0;
    }
    if let Some(val) = source.get(i - 1) {
        if let Some(val) = val.get(j) {
            if *val == '#' {
                num += 1;
            }
        }
    }
    if let Some(val) = source.get(i) {
        if let Some(val) = val.get(j - 1) {
            if *val == '#' {
                num += 1;
            }
        }
    }
    if let Some(val) = source.get(i) {
        if let Some(val) = val.get(j + 1) {
            if *val == '#' {
                num += 1;
            }
        }
    }
    if let Some(val) = source.get(i + 1) {
        if let Some(val) = val.get(j) {
            if *val == '#' {
                num += 1;
            }
        }
    }
    return num; 
}

