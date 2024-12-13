use std::io::{stdin, BufRead, BufReader, Read};

fn to_i64(raw: &str, sep: &str) -> i64 {
    raw.split_once(sep).unwrap().1.parse::<i64>().unwrap()
}

#[derive(Debug)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    fn from_bits(bits: (&str, &str), sep: &str) -> Self {
        Coords {
            x: to_i64(bits.0, sep),
            y: to_i64(bits.1, sep),
        }
    }
}

#[derive(Debug)]
struct Solution {
    times_a: u64,
    times_b: u64,
}

impl Solution {
    fn cost(&self, tokens_a: u64, tokens_b: u64) -> u64 {
        self.times_a * tokens_a + self.times_b * tokens_b
    }
}

#[derive(Debug)]
struct EqSystem {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    s1: i64,
    s2: i64,
}

impl EqSystem {
    fn new(c_a: &Coords, c_b: &Coords, res: &Coords) -> Self {
        EqSystem {
            a: c_a.x,
            b: c_b.x,
            c: c_a.y,
            d: c_b.y,
            s1: res.x,
            s2: res.y
        }
    }

    fn solve(&self) -> Option<(u64, u64)> {
        let det_a = (self.a * self.d) - (self.c * self.b);
        let det1 = (self.s1 * self.d) - (self.s2 * self.b);
        let det2 = (self.a * self.s2) - (self.c * self.s1);

        if det_a == 0 {
            None
        } else {
            let r1 = det1 % det_a;
            let r2 = det2 % det_a;
            let a_presses = det1 / det_a;
            let b_presses = det2 / det_a;

            if r1 != 0 || r2 != 0 || a_presses < 0 || b_presses < 0 {
                None
            } else {
                Some((a_presses as u64, b_presses as u64))
            }
        }
    }
}

#[derive(Debug)]
struct Machine {
    button_a: Coords,
    button_b: Coords,
    prize_at: Coords,
}

static BUMP: i64 = 10000000000000;

impl Machine {
    fn find_solution(&self, max_presses: Option<u64>) -> Option<Solution> {
        let eqs = EqSystem::new(&self.button_a, &self.button_b, &self.prize_at);
        eqs.solve().and_then(|(times_a, times_b)| {
            if max_presses.is_some_and(|mp| times_a.max(times_b) > mp) {
                None
            } else {
                Some(Solution { times_a, times_b })
            }
        })
    }

    fn bump(self) -> Machine {
        Machine {
            prize_at: Coords { x: self.prize_at.x + BUMP, y: self.prize_at.y + BUMP },
            .. self
        }
    }
}

#[derive(Default)]
struct MachineBuilder {
    button_a: Option<Coords>,
    button_b: Option<Coords>,
    prize_at: Option<Coords>,
}

impl MachineBuilder {
    fn set_button_a(self, delta: Coords) -> MachineBuilder {
        MachineBuilder {
            button_a: Some(delta),
            .. self
        }
    }

    fn set_button_b(self, delta: Coords) -> MachineBuilder {
        MachineBuilder {
            button_b: Some(delta),
            .. self
        }
    }

    fn set_prize_at(self, pos: Coords) -> MachineBuilder {
        MachineBuilder {
            prize_at: Some(pos),
            .. self
        }
    }

    fn build(self) -> Machine {
        Machine {
            button_a: self.button_a.unwrap(),
            button_b: self.button_b.unwrap(),
            prize_at: self.prize_at.unwrap(),
        }
    }
}

fn get_bits<'a>(line: &'a str) -> (&'a str, &'a str)
{
    line.split_once(": ").unwrap().1.split_once(", ").unwrap()
}

fn read_problem<R>(stream: BufReader<R>) -> Vec<Machine>
where
    R: Read
{
    let lines = stream.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut machines = vec![];
    let mut builder = MachineBuilder::default();

    for line in lines {
        if line.starts_with("Button A:") {
            builder = builder.set_button_a(Coords::from_bits(get_bits(line.trim_end()), "+"));
        } else if line.starts_with("Button B:") {
            builder = builder.set_button_b(Coords::from_bits(get_bits(line.trim_end()), "+"));
        } else if line.starts_with("Prize:") {
            machines.push(builder
                .set_prize_at(Coords::from_bits(get_bits(line.trim_end()), "="))
                .build()
            );

            builder = MachineBuilder::default();
        }
    }

    machines
}

fn main() {
    let machines = read_problem(BufReader::new(stdin()));
    const COST_A_BUTTON: u64 = 3;
    const COST_B_BUTTON: u64 = 1;

    let mut most_prizes = 0usize;
    let mut total_tokens = 0u64;
    for machine in machines.iter() {
        if let Some(solution) = machine.find_solution(Some(100)) {
            most_prizes += 1;
            total_tokens += solution.cost(COST_A_BUTTON, COST_B_BUTTON);
        }
    }

    eprintln!("Total tokens to win {most_prizes} prizes: {total_tokens}");

    let mut most_prizes = 0usize;
    let mut total_tokens = 0u64;
    for machine in machines {
        if let Some(solution) = machine.bump().find_solution(None) {
            most_prizes += 1;
            total_tokens += solution.cost(COST_A_BUTTON, COST_B_BUTTON);
        }
    }

    eprintln!("Total tokens to win {most_prizes} prizes: {total_tokens}");
}
