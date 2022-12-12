static INPUT: &str = include_str!("input");

const START: (usize, usize) = (20, 0);
const TARGET: (usize, usize) = (20, 43);

fn main() {
    let mut grid = Vec::<Vec<u8>>::new();
    for line in INPUT.lines() {
        grid.push(line.chars().map(|c| c as u8).collect());
    }

    let mut progress = vec![vec![None::<usize>; grid[0].len()]; grid.len()];
    progress[START.0][START.1] = Some(0);

    let mut queue = vec![START];

    loop {
        if queue.is_empty() {
            break;
        }

        let cells = queue.drain(..).collect::<Vec<_>>();
        for cell in cells {
            let neighbours = get_neighbours(&grid, cell);
            let c = grid[cell.0][cell.1];
            let s = progress[cell.0][cell.1].unwrap() + 1;

            for n in neighbours {
                let nc = grid[n.0][n.1];

                if nc <= c + 1 {
                    if let Some(ns) = progress[n.0][n.1] {
                        if ns > s {
                            progress[n.0][n.1] = Some(s);

                            queue.push(n);
                        }
                    } else {
                        progress[n.0][n.1] = Some(s);

                        queue.push(n);
                    }
                }
            }
        }
    }

    println!("{}", progress[TARGET.0][TARGET.1].unwrap());
}

fn get_neighbours(grid: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if row > 0 {
        neighbours.push((row - 1, col));
    }
    if col > 0 {
        neighbours.push((row, col - 1));
    }
    if col < (grid[0].len() - 1) {
        neighbours.push((row, col + 1));
    }
    if row < (grid.len() - 1) {
        neighbours.push((row + 1, col));
    }

    neighbours
}
