use std::io::Write;
use std::path::Path;

mod tsp_solver;

/// Solves the given Sudoku problem by backtracking,
/// starting from the cell at coordinate (i, j).
fn solve_sudoku(board: &mut [Vec<u32>], i: usize, j: usize) -> bool {
    // wrap & win
    if j == 9 {
        return solve_sudoku(board, i + 1, 0);
    }
    if i == 9 {
        return true;
    }

    // already filled
    if board[i][j] != 0 {
        return solve_sudoku(board, i, j + 1);
    }

    // brute force
    let mut is_candidate = vec![true; 10];
    // row
    for nj in 0..9 {
        is_candidate[board[i][nj] as usize] = false;
    }
    // col
    for ni in 0..9 {
        is_candidate[board[ni][j] as usize] = false;
    }
    // subgrid
    let (gi, gj) = (i / 3 * 3, j / 3 * 3);
    for ni in gi..gi + 3 {
        for nj in gj..gj + 3 {
            is_candidate[board[ni][nj] as usize] = false;
        }
    }

    for d in 1..=9 {
        if !is_candidate[d] {
            continue;
        }
        board[i][j] = d as u32;
        if solve_sudoku(board, i, j + 1) {
            return true;
        }
        board[i][j] = 0;
    }

    false
}

/// Returns the number of cursor movements to reach from (i1, j1) to (i2, j2),
/// in a grid of dimension `n * n`.
///
/// Note that in Dr. Sudoku,
/// - D-pad restriction applies.
/// - Wraparound is supported.
/// - Diagonal is not supported.
fn grid_distance(n: usize, (i1, j1): (usize, usize), (i2, j2): (usize, usize)) -> usize {
    let di = i1.abs_diff(i2);
    let di = di.min(n - di);
    let dj = j1.abs_diff(j2);
    let dj = dj.min(n - dj);
    if di == dj { 2 * di } else { 2 * di.max(dj) - 1 }
}

/// Returns the number of frames from `node_fr` to `node_to`.
/// - `answer` is the grid of answers, 0 for non-blanks and `d` for answer digit `d`.
/// - `blanks` is the list of blank cells' coordinates,
///   with (99, 99) being the first in the list and denoting the starting position.
///   Starting position is (0, 0) with digit fixed to 5.
/// - `node_fr` is the index in `blanks` of the current position.
/// - `node_to` is the index in `blanks` of the destination.
fn tas_distance(
    answer: &[Vec<u32>],
    blanks: &[(usize, usize)],
    node_fr: usize,
    node_to: usize,
) -> usize {
    let i1 = if node_fr == 0 { 0 } else { blanks[node_fr].0 };
    let j1 = if node_fr == 0 { 0 } else { blanks[node_fr].1 };
    let d1 = if node_fr == 0 { 5 } else { answer[i1][j1] } as usize;
    let (i2, j2) = blanks[node_to];
    let d2 = answer[i2][j2] as usize;

    // sudoku board
    grid_distance(9, (i1, j1), (i2, j2)) +
        // digit selection
        // if the two digits are same, wait 1 frame
        grid_distance(3,
            ((d1-1)/3, (d1-1)%3),
            ((d2-1)/3, (d2-1)%3),
        ).max(1)
}

fn solve_tsp(answer: &[Vec<u32>]) -> (usize, Vec<usize>) {
    // (99, 99) is starting position; it's (0, 0) but initial digit is fixed to 5
    let mut blanks = vec![(99, 99)];
    for i in 0..9 {
        for j in 0..9 {
            if answer[i][j] != 0 {
                blanks.push((i, j));
            }
        }
    }
    let n = blanks.len();
    let mut dist_matrix = vec![vec![0; n]; n];

    for node_fr in 0..n {
        for node_to in 0..n {
            if node_to == 0 || node_fr == node_to {
                continue;
            }
            let dist = tas_distance(answer, &blanks, node_fr, node_to);
            dist_matrix[node_fr][node_to] = dist as u32;
        }
    }

    let (cost, mut solution) = tsp_solver::solve_atsp(&dist_matrix);
    let cost = cost as usize + 3 * blanks.len() - 4;
    tsp_solver::find_rotate(&mut solution, 0);

    let mut tkl_path = vec![];
    for id in solution[1..].into_iter() {
        let (i, j) = blanks[*id];
        tkl_path.push(i * 9 + j);
    }
    (cost, tkl_path)
}

fn read_grids(path: impl AsRef<Path>) -> Vec<Vec<Vec<u32>>> {
    let text = std::fs::read_to_string(path).unwrap();
    text.trim()
        .split("\n\n")
        .map(|block| block.split_once('\n').unwrap().1)
        .map(|text| {
            text.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|d| d.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();

    if mode == "sudoku" {
        let levels = read_grids("levels/dr-sudoku/problems.txt");

        // sove each level
        for (id, level) in levels.into_iter().enumerate() {
            let mut board = level.clone();
            assert!(solve_sudoku(&mut board, 0, 0));
            println!("{} {}", id / 50 + 1, id % 50 + 1);
            for (row, original_row) in board.into_iter().zip(level) {
                for (d, original_d) in row.into_iter().zip(original_row) {
                    print!("{} ", if original_d == 0 { d } else { 0 });
                }
                println!();
            }
            println!();
        }
        return;
    }

    if mode == "cost" {
        let level_id = std::env::args().nth(2).unwrap().parse::<usize>().unwrap();

        let answer = &read_grids("levels/dr-sudoku/answers.txt")[level_id];
        let mut blanks = vec![(99, 99)];
        for i in 0..9 {
            for j in 0..9 {
                if answer[i][j] != 0 {
                    blanks.push((i, j));
                }
            }
        }

        let text = std::fs::read_to_string("levels/dr-sudoku/new_output.txt").unwrap();
        let mut route = text
            // load `level_id`-th solution
            .lines()
            .nth(level_id)
            .unwrap()
            // extract route text from format `COST | POS1 POS2 ...`
            .split_once(" | ")
            .unwrap()
            .1
            .split_whitespace()
            // parse route text
            .map(|w| {
                let id = w.parse::<usize>().unwrap();
                blanks.iter().position(|b| *b == (id / 9, id % 9)).unwrap()
            })
            .collect::<Vec<_>>();
        // starting node
        route.insert(0, 0);

        for row in answer {
            for x in row {
                print!("{x} ");
            }
            println!();
        }
        let mut total_dist = 0;
        for window in route.windows(2) {
            let fr = window[0];
            let to = window[1];
            let dist = tas_distance(&answer, &blanks, fr, to);
            total_dist += dist;
            println!("{:?} -> {:?}: {}", blanks[fr], blanks[to], dist);
        }
        let extra = 3 * blanks.len() - 4;
        println!("total: {total_dist} + {extra} = {}", total_dist + extra);
        return;
    }

    if mode == "single" {
        let level_id = std::env::args().nth(2).unwrap().parse::<usize>().unwrap();
        let answer = &read_grids("levels/dr-sudoku/answers.txt")[level_id];

        let (cost, tkl_path) = solve_tsp(&answer);
        println!("{cost} | ");
        for x in tkl_path {
            print!("{x} ");
        }
        println!("");
        return;
    }

    let levels = read_grids("levels/dr-sudoku/answers.txt");
    let mut output = std::fs::File::create("levels/dr-sudoku/new_output.txt").unwrap();
    for level in levels {
        let (cost, tkl_path) = solve_tsp(&level);
        write!(&mut output, "{cost} | ").unwrap();
        for x in tkl_path {
            write!(&mut output, "{x} ").unwrap();
        }
        writeln!(&mut output, "").unwrap();
    }
}
