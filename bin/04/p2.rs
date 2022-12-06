use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input");

fn parse_range(s: &str) -> (usize, usize) {
    let (from, to);
    text_io::scan!(s.bytes() => "{}-{}", from, to);

    (from, to)
}

fn main() {
    let result = INPUT
        .lines()
        .map(|line| {
            let ranges = line.split(",").collect::<Vec<_>>();
            let (lrange, rrange) = (parse_range(ranges[0]), parse_range(ranges[1]));

            (lrange, rrange)
        })
        .filter(|((lx, ly), (rx, ry))| ((ry >= lx) && (rx <= ly)))
        .count();

    println!("{result}");
}
