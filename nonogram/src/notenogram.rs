use std::collections::{HashMap, VecDeque};

mod tsp_solver;

use crate::tsp_solver::find_rotate;
use crate::tsp_solver::solve_set_atsp;

type NodeT = (u32, u32, char);

fn solve(grid: &str, start: NodeT) {
    println!("{grid}");

    let grid = grid
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = grid.len();
    let width = grid[0].len();
    for row in &grid {
        assert_eq!(row.len(), width);
    }

    let mut partition = vec![vec![0]];
    let mut nodes = vec![start];

    for (i, row) in grid.iter().enumerate() {
        let i = i as u32;
        for (j, c) in row.iter().enumerate() {
            let j = j as u32;
            if *c != '*' {
                continue;
            }
            let mut new_nodes = vec![];
            if i != 0 {
                new_nodes.push((i, j, 'D'));
            }
            if i + 1 != height as u32 {
                new_nodes.push((i, j, 'U'));
            }
            if j != 0 {
                new_nodes.push((i, j, 'R'));
            }
            if j + 1 != width as u32 {
                new_nodes.push((i, j, 'L'));
            }
            let n = nodes.len();
            partition.push((n..n + new_nodes.len()).collect::<Vec<_>>());
            nodes.extend(new_nodes);
        }
    }
    let n = nodes.len();

    let mut dist = vec![vec![0; n]; n];
    let mut paths = vec![vec!["".to_string(); n]; n];
    for i in 0..n {
        let snode = nodes[i];

        let mut bfs_ans = HashMap::<NodeT, (u32, NodeT)>::new();
        let mut queue = VecDeque::from_iter([snode]);
        bfs_ans.insert(snode, (0, snode));
        while let Some((i, j, d)) = queue.pop_front() {
            let cur_dist = bfs_ans[&(i, j, d)].0;

            let mut nexts = vec![(i, j, 'U'), (i, j, 'D'), (i, j, 'L'), (i, j, 'R')];
            if d == 'S' {
                nexts = vec![(i, j, 'Z')];
            }
            if i != 0 && d != 'U' {
                nexts.push((i - 1, j, 'U'));
            }
            if i + 1 != height as u32 && d != 'D' {
                nexts.push((i + 1, j, 'D'));
            }
            if j != 0 && d != 'L' {
                nexts.push((i, j - 1, 'L'));
            }
            if j + 1 != width as u32 && d != 'R' {
                nexts.push((i, j + 1, 'R'));
            }

            for tup in nexts {
                if bfs_ans.contains_key(&tup) {
                    continue;
                }
                bfs_ans.insert(tup, (cur_dist + 1, (i, j, d)));
                queue.push_back(tup);
            }
        }

        dist[i][0] = 0;
        for j in 1..n {
            dist[i][j] = bfs_ans[&nodes[j]].0;
            let mut path_stack = vec![];
            let mut last = nodes[j];
            while last != snode {
                let prev = bfs_ans[&last].1;
                if (prev.0, prev.1) == (last.0, last.1) {
                    path_stack.push('.');
                } else {
                    path_stack.push(last.2);
                }
                last = prev;
            }
            let path = path_stack.iter().rev().collect::<String>();
            paths[i][j] = path;
        }
    }

    let (cost, mut tour) = solve_set_atsp(&dist, &partition);
    find_rotate(&mut tour, 0);

    println!("======================");
    println!("{cost}");
    for w in tour.windows(2) {
        print!("{} ", paths[w[0]][w[1]]);
    }
    println!();
    for w in tour.windows(2) {
        let path = &paths[w[0]][w[1]];
        for (i, c) in path.chars().enumerate() {
            print!("|    0,    0,    0,    0,");
            for target_c in ['U', 'D', 'L', 'R'] {
                print!("{}", if c == target_c { target_c } else { '.' });
            }
            println!("...{}...||", if i + 1 == path.len() { 'A' } else { '.' });
        }
    }
}

fn main() {
    let levels = std::fs::read_to_string("levels/notenogram.txt").unwrap();
    let levels = levels
        .split("\n\n")
        .map(|block| block.split_once('\n').unwrap().1)
        .collect::<Vec<_>>();
    let start = (5, 5, 'S');

    let level_id = std::env::args().nth(1).unwrap().parse::<usize>().unwrap();
    solve(levels[level_id], start);
}
