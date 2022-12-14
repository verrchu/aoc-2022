static INPUT: &str = include_str!("input");

fn main() {
    let (mut max_x, mut max_y) = (0, 0);

    let lines = INPUT
        .lines()
        .map(|line| {
            line.trim()
                .split(" -> ")
                .map(|point| {
                    let mut point = point.split(",");
                    let x = point.next().unwrap().parse::<usize>().unwrap();
                    let y = point.next().unwrap().parse::<usize>().unwrap();

                    if x > max_x { max_x = x; }
                    if y > max_y { max_y = y; }

                    (x, y)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    max_x *= 2;

    let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];
    grid.push(vec!['.'; max_x + 1]);
    grid.push(vec!['#'; max_x + 1]);

    for line in lines {
        for w in line.windows(2) {
            let (a, b) = (w[0], w[1]);

            if a.0 == b.0 {
                let x = a.0;
                let (y1, y2) = if a.1 > b.1 { (b.1, a.1) } else { (a.1, b.1) };

                for y in y1..=y2 { grid[y][x] = '#'; }
            } else if a.1 == b.1 {
                let y = a.1;
                let (x1, x2) = if a.0 > b.0 { (b.0, a.0) } else { (a.0, b.0) };

                for x in x1..=x2 { grid[y][x] = '#'; }
            } else {
                unreachable!()
            }
        }
    }

    let mut count = 0;
    loop {
        let (mut x, mut y) = (500, 0);

        if grid[y][x] != '.' { break; }

        loop {
            if grid[y + 1][x] == '.' {
                y += 1;
            } else if grid[y + 1][x - 1] == '.' {
                y += 1; x -= 1;
            } else if grid[y + 1][x + 1] == '.' {
                y += 1; x += 1;
            } else {
                grid[y][x] = '+'; count += 1; break;
            }
        }
    }

    for line in grid.iter() {
        println!("{}", line.into_iter().collect::<String>());
    }

    println!("{count}");
}
