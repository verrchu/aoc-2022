use serde::Deserialize;

static INPUT: &str = include_str!("input.yaml");
const ROUNDS: usize = 10_000;

#[derive(Debug, Clone, Deserialize)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    test: Test,
}

impl Monkey {
    fn wlevel(&self, item: usize, m: usize) -> usize {
        let tmp = match self.op {
            Op::Sqr => item * item,
            Op::Mul(n) => item * n,
            Op::Add(n) => item + n,
        };

        tmp % m
    }

    fn pass_to(&self, wlevel: usize) -> usize {
        if wlevel % self.test.div == 0 {
            self.test.t
        } else {
            self.test.f
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Op {
    Sqr,
    Mul(usize),
    Add(usize),
}

#[derive(Debug, Clone, Deserialize)]
struct Test {
    div: usize,
    t: usize,
    f: usize,
}

fn main() {
    let mut monkeys = serde_yaml::from_str::<Vec<Monkey>>(INPUT).unwrap();
    let m = monkeys.iter().map(|m| m.test.div).product::<usize>();

    let mut inspected = vec![0; monkeys.len()];

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            let items = monkeys[i].items.drain(..).collect::<Vec<usize>>();

            inspected[i] += items.len();

            for item in items {
                let wlevel = monkey.wlevel(item, m);
                let pass_to = monkey.pass_to(wlevel);

                monkeys[pass_to].items.push(wlevel);
            }
        }
    }

    inspected.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", inspected[0] * inspected[1]);
}
