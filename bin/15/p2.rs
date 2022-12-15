static INPUT: &str = include_str!("input");

fn main() {
    let mut input = INPUT.lines()
        .map(|line| {
            let (sx, sy, bx, by): (isize, isize, isize, isize);
            text_io::scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by);

            ((sx, sy), (bx, by))
        })
        .collect::<Vec<_>>();
    input.sort_by_key(|((sx, _sy), _b)| *sx);

    for i in 0..4_000_000 {
        let mut ranges = input
            .iter()
            .copied()
            .filter_map(|(s, b)| get_range(i, s, b))
            .collect::<Vec<_>>();

        loop {
            let mut modified = false;

            ranges = ranges.into_iter().fold(vec![], |mut acc, x| {
                if let Some(last) = acc.last_mut() {
                    let (la, lb) = last;
                    let (xa, xb) = x;

                    if xa <= *lb + 1 {
                        if xb > *lb { last.1 = xb; }
                        if xa < *la { last.0 = xa; }

                        modified = true;
                    } else {
                        acc.push(x);
                    }

                    acc
                } else {
                    vec![x]
                }
            });

            if !modified { break; }
        }

        let diff = ranges
            .windows(2)
            .filter_map(|w| {
                let (_, a) = w[0];
                let (b, _) = w[1];

                (b == a + 2).then_some(1)
            })
            .sum::<isize>();

        if diff == 1 {
            println!("{i} {ranges:?}");
        }
    }
}

fn get_range(
    y: isize,
    (sx, sy): (isize, isize),
    (bx, by): (isize, isize),
) -> Option<(isize, isize)> {
    let r = (sy - by).abs() + (sx - bx).abs();
    let ydiff = (y - sy).abs();

    (ydiff <= r).then(|| {
        let level = r - ydiff;
        ((sx - level), (sx + level))
    })
}
