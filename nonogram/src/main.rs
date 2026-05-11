use concorde_rs::{Distance, LowerDistanceMatrix, Solution, solver};

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

    let (cost, tour) = solve_stsp(&dist);
    for v in tour {
        print!("{:?} ", grid[v]);
    }
    println!("{cost}");
}
