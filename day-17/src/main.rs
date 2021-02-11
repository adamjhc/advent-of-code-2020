use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    part_1();
}

#[derive(PartialEq, Clone, Copy)]
enum CubeState {
    Active,
    Inactive,
}

impl FromStr for CubeState {
    type Err = ();

    fn from_str(s: &str) -> Result<CubeState, ()> {
        match s {
            "." => Ok(CubeState::Inactive),
            "#" => Ok(CubeState::Active),
            _ => Err(()),
        }
    }
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let initial_state: Vec<Vec<CubeState>> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|state| state.to_string().parse::<CubeState>().unwrap())
                .collect()
        })
        .collect();

    let cycles = 6;

    // create pocket dimension at maximum size
    let cycle_expansion = cycles * 2;
    let max_x = initial_state.len() + cycle_expansion;
    let max_y = initial_state.len() + cycle_expansion;
    let max_z = 1 + cycle_expansion;
    let mut pocket_dimension: Vec<Vec<Vec<CubeState>>> = Vec::new();
    for x in 0..max_x {
        pocket_dimension.push(Vec::new());
        for y in 0..max_y {
            pocket_dimension[x].push(Vec::new());
            for _z in 0..max_z {
                pocket_dimension[x][y].push(CubeState::Inactive);
            }
        }
    }

    // insert initial state in centre of pocket dimension
    // in pocket dimension centre (0, 0, 0) will be (10, 10, 6) of vec
    // so the top-right of the initial state will be at (10 - (8 / 2))
    let pocket_dimension_centre_x = pocket_dimension.len() / 2;
    let pocket_dimension_centre_y = pocket_dimension[0].len() / 2;
    for initial_x in 0..initial_state.len() {
        for initial_y in 0..initial_state[initial_x].len() {
            pocket_dimension[pocket_dimension_centre_x - (initial_state.len() / 2) + initial_x]
                [pocket_dimension_centre_y - (initial_state[initial_x].len() / 2) + initial_y][6] =
                initial_state[initial_x][initial_y]
        }
    }

    // create vec of neighbour offsets
    // let mut neighbour_offsets: Vec<(usize, usize, usize)> = Vec::new();
    // for x in 0..=2 {
    //     for y in 0..=2 {
    //         for z in 0..=2 {
    //             if !(x == 0 && y == 0 && z == 0) {
    //                 neighbour_offsets.push((x, y, z));
    //             }
    //         }
    //     }
    // }

    // run simulation for 6 cycles
    for _ in 0..cycles {
        // clone previous state
        let mut pocket_dimension_update = pocket_dimension.clone();

        // loop through cubes in 3D
        for x in 0..pocket_dimension.len() {
            for y in 0..pocket_dimension[x].len() {
                for z in 0..pocket_dimension[x][y].len() {
                    // count active neighbours
                    let mut active_neighbours = 0;
                    // for (offset_x, offset_y, offset_z) in &neighbour_offsets {
                    //     if (x + offset_x) as isize - 1 < 0
                    //         || (y + offset_y) as isize - 1 < 0
                    //         || (z + offset_z) as isize - 1 < 0
                    //     {
                    //         continue;
                    //     }

                    //     if let Some(xs) = pocket_dimension.get(x + offset_x - 1) {
                    //         if let Some(ys) = xs.get(y + offset_y - 1) {
                    //             if let Some(CubeState::Active) = ys.get(z + offset_z - 1) {
                    //                 active_neighbours += 1;
                    //             }
                    //         }
                    //     }

                    //     // could break early since 4 neighbours and above are all the same
                    // }

                    for xi in max(x as isize - 1, 0) as usize..min(x + 2, pocket_dimension.len()) {
                        for yi in
                            max(y as isize - 1, 0) as usize..min(y + 2, pocket_dimension[x].len())
                        {
                            for zi in max(z as isize - 1, 0) as usize
                                ..min(z + 2, pocket_dimension[x][y].len())
                            {
                                if x == xi && y == yi && z == zi {
                                    continue;
                                }

                                if pocket_dimension[x][y][z] == CubeState::Active {
                                    active_neighbours += 1;
                                }
                            }
                        }
                    }

                    // apply change rules
                    match pocket_dimension[x][y][z] {
                        CubeState::Active if active_neighbours != 2 && active_neighbours != 3 => {
                            pocket_dimension_update[x][y][z] = CubeState::Inactive
                        }
                        CubeState::Inactive if active_neighbours == 3 => {
                            pocket_dimension_update[x][y][z] = CubeState::Active
                        }
                        _ => (),
                    }
                }
            }
        }

        // update state
        pocket_dimension = pocket_dimension_update;
    }

    // count active cubes
    let mut active_cubes = 0;
    for x in 0..pocket_dimension.len() {
        for y in 0..pocket_dimension[x].len() {
            for z in 0..pocket_dimension[x][y].len() {
                if pocket_dimension[x][y][z] == CubeState::Active {
                    active_cubes += 1;
                }
            }
        }
    }
    println!("{}", active_cubes);
}
