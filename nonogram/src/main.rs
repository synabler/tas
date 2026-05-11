use std::collections::HashSet;

use concorde_rs::{Distance, LowerDistanceMatrix, Solution, solver};

fn find_rotate(values: &mut [usize], target: usize) {
    let i = values.iter().position(|x| *x == target).unwrap();
    values.rotate_left(i);
}

fn solve_stsp(distances: &[Vec<u32>]) -> (u32, Vec<usize>) {
    // preliminary verification
    let n = distances.len();
    for row in distances {
        assert_eq!(row.len(), n);
    }
    for i in 0..n {
        assert_eq!(distances[i][i], 0);
        for j in i + 1..n {
            assert_eq!(distances[i][j], distances[j][i]);
        }
    }

    // solution
    let values = distances
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row[..=i].iter().copied())
        .collect::<Vec<_>>();
    assert_eq!(values.len(), n * (n + 1) / 2);
    let matrix = LowerDistanceMatrix {
        num_nodes: n as u32,
        values,
    };
    let solution = solver::tsp_lk(&matrix).unwrap();
    let cost = solution.length;
    let tour = solution
        .tour
        .into_iter()
        .map(|v| v as usize)
        .collect::<Vec<_>>();

    // solution verification
    let checked_cost = tour.windows(2).map(|w| distances[w[0]][w[1]]).sum::<u32>()
        + distances[tour[0]][*tour.last().unwrap()];
    assert_eq!(cost, checked_cost);

    // return
    (cost, tour)
}

fn solve_atsp(distances: &[Vec<u32>]) -> (u32, Vec<usize>) {
    // preliminary verification
    let n = distances.len();
    for row in distances {
        assert_eq!(row.len(), n);
    }
    for i in 0..n {
        assert_eq!(distances[i][i], 0);
    }

    // solution
    // i: i-in
    // i+n: i-out
    let bound = 10_000u32;
    let mut split_distances = vec![vec![10_000_000u32; 2 * n]; 2 * n];
    for i in 0..2 * n {
        split_distances[i][i] = 0;
    }
    for i in 0..n {
        // i-in -- i-out cost 0
        split_distances[i][i + n] = 0;
        split_distances[i + n][i] = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            // i-out -- j-in cost cij + B
            split_distances[i + n][j] = distances[i][j] + bound;
            split_distances[j][i + n] = distances[i][j] + bound;
        }
    }
    let (cost, mut tour) = solve_stsp(&split_distances);
    find_rotate(&mut tour, 0);
    if tour[1] != n {
        tour.reverse();
        find_rotate(&mut tour, 0);
    }
    let cost = cost - bound * (n as u32);
    let decoupled_tour = tour.chunks(2).map(|chunk| chunk[0]).collect::<Vec<_>>();
    println!("ATSP solution: {decoupled_tour:?}");

    // solution verification
    for chunk in tour.chunks(2) {
        assert_eq!(chunk[0] + n, chunk[1]);
    }
    assert_eq!(decoupled_tour.len(), n);
    let checked_cost = decoupled_tour
        .windows(2)
        .map(|w| distances[w[0]][w[1]])
        .sum::<u32>()
        + distances[*decoupled_tour.last().unwrap()][decoupled_tour[0]];
    assert_eq!(cost, checked_cost);

    // return
    (cost, decoupled_tour)
}

fn solve_set_atsp(distances: &[Vec<u32>], partition: &[Vec<usize>]) -> (u32, Vec<usize>) {
    // preliminary verification
    let n = distances.len();
    for row in distances {
        assert_eq!(row.len(), n);
    }
    for i in 0..n {
        assert_eq!(distances[i][i], 0);
    }
    let mut union = partition.iter().flatten().copied().collect::<Vec<_>>();
    union.sort_unstable();
    assert_eq!(union, (0..n).collect::<Vec<_>>());

    // index of partition that contains i
    let mut part_index = vec![0usize; n];
    for (i, part) in partition.iter().enumerate() {
        for p in part {
            part_index[*p] = i;
        }
    }

    // solution
    // for each partition [p1 p2 ... pk]:
    //     p1 -> p2 -> ... -> pk -> p1, each cost 0
    //     for each outgoing pi -> q:
    //         p[i-1] -> q, cost copied
    let bound = 10_000u32;
    let mut new_distances = vec![vec![10_000_000u32; n]; n];
    for i in 0..n {
        new_distances[i][i] = 0;
    }
    for part in partition {
        let part_set: HashSet<usize> = HashSet::from_iter(part.iter().copied());
        let mut part_cycle = part.clone();
        part_cycle.push(part[0]);
        for w in part_cycle.windows(2) {
            new_distances[w[0]][w[1]] = 0;
        }
        for (i, p) in part_cycle.iter().enumerate().skip(1) {
            for q in 0..n {
                if part_set.contains(&q) {
                    continue;
                }
                let prev_p = part_cycle[i - 1];
                new_distances[prev_p][q] = distances[*p][q] + bound;
            }
        }
    }
    let (cost, tour) = solve_atsp(&new_distances);
    let cost = cost - bound * (partition.len() as u32);
    let mut degrouped_tour = vec![];
    let mut i = 0;
    while i < tour.len() {
        degrouped_tour.push(tour[i]);
        i += partition[part_index[tour[i]]].len();
    }
    println!("SetATSP solution: {cost} {degrouped_tour:?}");

    // solution verification
    let mut i = 0;
    while i < tour.len() {
        let pi = part_index[tour[i]];
        let mut got_part = tour[i..i + partition[pi].len()].to_vec();
        let mut actual_part = partition[pi].clone();
        got_part.sort_unstable();
        actual_part.sort_unstable();
        assert_eq!(got_part, actual_part);
        i += partition[part_index[tour[i]]].len();
    }
    assert_eq!(degrouped_tour.len(), partition.len());
    let checked_cost = degrouped_tour
        .windows(2)
        .map(|w| distances[w[0]][w[1]])
        .sum::<u32>()
        + distances[*degrouped_tour.last().unwrap()][degrouped_tour[0]];
    assert_eq!(cost, checked_cost);

    // return
    (cost, degrouped_tour)
}

const GRID: &str = "..########..
..#......#..
..#.####.#..
..#.####.#..
..#.####.#..
..#......#..
..#.#....#..
..####..##..
..#.#..#.#..
..#......#..
..#.####.#..
..#######...";
const START: (u32, u32) = (5, 5);
/*
const GRID: &str = ".#.
#.#
..#";
const START: (u32, u32) = (1, 1);
*/

fn main() {
    let grid = {
        let mut grid = vec![START];
        for (i, row) in GRID.lines().enumerate() {
            for (j, c) in row.chars().enumerate() {
                if c == '#' {
                    grid.push((i as u32, j as u32));
                }
            }
        }
        grid
    };

    let n = grid.len();
    let mut dist = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let a = grid[i];
            let b = grid[j];
            dist[i][j] = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
        }
    }

    let mut partition = vec![vec![0]];
    for i in (1..n).step_by(2) {
        partition.push(vec![i, i + 1]);
    }

    let (cost, mut tour) = solve_set_atsp(&dist, &partition);
    find_rotate(&mut tour, 0);
    for v in tour {
        print!("{:?} ", grid[v]);
    }
    println!("{cost}");
}
