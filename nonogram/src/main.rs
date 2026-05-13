use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;

use std::process::Command;

fn find_rotate(values: &mut [usize], target: usize) {
    let i = values.iter().position(|x| *x == target).unwrap();
    values.rotate_left(i);
}

fn log_stsp_problem(distances: &[Vec<u32>]) -> std::io::Result<()> {
    let mut file = std::fs::File::create("concorde/input.txt")?;

    writeln!(&mut file, "NAME : p")?;
    writeln!(&mut file, "TYPE : TSP")?;
    writeln!(&mut file, "DIMENSION : {}", distances.len())?;
    writeln!(&mut file, "EDGE_WEIGHT_TYPE : EXPLICIT")?;
    writeln!(&mut file, "EDGE_WEIGHT_FORMAT : FULL_MATRIX")?;
    writeln!(&mut file, "EDGE_WEIGHT_SECTION")?;
    for row in distances {
        for x in row {
            write!(&mut file, "{x} ")?;
        }
        writeln!(&mut file)?;
    }
    writeln!(&mut file, "EOF")?;
    Ok(())
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
    log_stsp_problem(distances).unwrap();
    Command::new("./concorde")
        .current_dir("./concorde")
        .arg("input.txt")
        .status()
        .unwrap();
    let cost = std::fs::read_to_string("concorde/input.res")
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let tour = std::fs::read_to_string("concorde/input.sol")
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|w| w.parse::<usize>().unwrap())
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
const START: (u32, u32, char) = (5, 5, 'S');

fn main() {
    let mut partition = vec![vec![0]];
    let mut nodes = vec![START];

    for (i, row) in GRID.lines().enumerate() {
        let i = i as u32;
        for (j, c) in row.chars().enumerate() {
            let j = j as u32;
            if c != '#' {
                continue;
            }
            let mut new_nodes = vec![];
            if i != 0 {
                new_nodes.push((i, j, 'D'));
            }
            if i != 11 {
                new_nodes.push((i, j, 'U'));
            }
            if j != 0 {
                new_nodes.push((i, j, 'R'));
            }
            if j != 11 {
                new_nodes.push((i, j, 'L'));
            }
            let n = nodes.len();
            partition.push((n..n + new_nodes.len()).collect::<Vec<_>>());
            nodes.extend(new_nodes);
        }
    }
    let n = nodes.len();

    let mut dist = vec![vec![0; n]; n];
    for i in 0..n {
        let (si, sj, sd) = nodes[i];

        let mut bfs_ans = HashMap::<(u32, u32, char), u32>::new();
        let mut queue = VecDeque::from_iter([(si, sj, sd)]);
        bfs_ans.insert((si, sj, sd), 0);
        while let Some((i, j, d)) = queue.pop_front() {
            let cur_dist = bfs_ans[&(i, j, d)];

            let mut nexts = vec![(i, j, 'U'), (i, j, 'D'), (i, j, 'L'), (i, j, 'R')];
            if i != 0 && d != 'U' {
                nexts.push((i - 1, j, 'U'));
            }
            if i != 11 && d != 'D' {
                nexts.push((i + 1, j, 'D'));
            }
            if j != 0 && d != 'L' {
                nexts.push((i, j - 1, 'L'));
            }
            if j != 11 && d != 'R' {
                nexts.push((i, j + 1, 'R'));
            }

            for tup in nexts {
                if bfs_ans.contains_key(&tup) {
                    continue;
                }
                bfs_ans.insert(tup, cur_dist + 1);
                queue.push_back(tup);
            }
        }

        dist[i][0] = 0;
        for j in 1..n {
            dist[i][j] = bfs_ans[&nodes[j]];
        }
    }

    let (cost, mut tour) = solve_set_atsp(&dist, &partition);
    find_rotate(&mut tour, 0);

    for w in tour.windows(2) {
        let a = nodes[w[0]];
        let b = nodes[w[1]];
        println!("{a:?} -> {b:?} cost {}", dist[w[0]][w[1]]);
    }
    println!("{cost}");
}
