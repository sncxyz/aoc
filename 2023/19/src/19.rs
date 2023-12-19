use aoc::{IterUnwrap, Parse};
use rustc_hash::FxHashMap as HashMap;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> u64 {
    let (workflows, i) = parse(input);
    input
        .lines()
        .skip(i)
        .map(Part::parse)
        .filter_map(|part| workflows.sort(part).then_some(part.total()))
        .sum()
}

fn part_2(input: aoc::Input) -> impl ToString {
    parse(input).0.sort_all()
}

fn parse(input: aoc::Input) -> (Workflows, usize) {
    let i = input.lines().position(str::is_empty).unwrap();
    let workflows = Workflows::parse(&input.as_lines()[..i]);
    (workflows, i + 1)
}

struct Workflows {
    workflows: Vec<Workflow>,
    entry: usize,
}

impl Workflows {
    fn parse(lines: &[&str]) -> Self {
        let mut names = Names::new();
        let mut workflows = vec![Workflow::default(); lines.len()];
        let mut entry = 0;
        for line in lines {
            let (name, rules) = line.split_once('{').unwrap();
            let i = names.get(name);
            if name == "in" {
                entry = i;
            }
            workflows[i] = Workflow::parse(&rules[..rules.len() - 1], &mut names);
        }
        Self { workflows, entry }
    }

    fn sort(&self, part: Part) -> bool {
        let mut current = self.entry;
        loop {
            match self.workflows[current].sort(part) {
                Dest::Accept => return true,
                Dest::Reject => return false,
                Dest::Workflow(w) => current = w,
            }
        }
    }

    fn sort_all(&self) -> u64 {
        self.sort_ranges(Dest::Workflow(self.entry), Ranges([Range(1, 4000); 4]))
    }

    fn sort_ranges(&self, dest: Dest, mut ranges: Ranges) -> u64 {
        let workflow = match dest {
            Dest::Accept => return ranges.size(),
            Dest::Reject => return 0,
            Dest::Workflow(w) => w,
        };
        let workflow = &self.workflows[workflow];
        let mut total = 0;
        for &rule in &workflow.rules {
            let [passed, failed] = ranges.split(rule);
            if let Some(passed) = passed {
                total += self.sort_ranges(rule.dest, passed);
            }
            if let Some(failed) = failed {
                ranges = failed;
            } else {
                return total;
            }
        }
        total + self.sort_ranges(workflow.last, ranges)
    }
}

#[derive(Default, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    last: Dest,
}

impl Workflow {
    fn parse<'a>(s: &'a str, names: &mut Names<'a>) -> Self {
        let mut parts = s.split(',').peekable();
        let mut rules = Vec::new();
        let mut next = parts.next_uw();
        while parts.peek().is_some() {
            rules.push(Rule::parse(next, names));
            next = parts.next_uw();
        }
        let last = Dest::parse(next, names);
        Self { rules, last }
    }

    fn sort(&self, part: Part) -> Dest {
        for rule in &self.rules {
            if let Some(dest) = rule.sort(part) {
                return dest;
            }
        }
        self.last
    }
}

#[derive(Default, Clone, Copy)]
struct Rule {
    cat: usize,
    value: u64,
    comp: Comp,
    dest: Dest,
}

impl Rule {
    fn parse<'a>(s: &'a str, names: &mut Names<'a>) -> Self {
        let cat = match s.idx(0) {
            b'x' => 0,
            b'm' => 1,
            b'a' => 2,
            b's' => 3,
            _ => unreachable!(),
        };
        let comp = match s.idx(1) {
            b'>' => Comp::Greater,
            b'<' => Comp::Less,
            _ => unreachable!(),
        };
        let (value, dest) = s[2..].split_once(':').unwrap();
        let value = value.parse_uw();
        let dest = Dest::parse(dest, names);
        Self {
            cat,
            value,
            comp,
            dest,
        }
    }

    fn sort(self, part: Part) -> Option<Dest> {
        let rating = part.ratings[self.cat];
        match self.comp {
            Comp::Greater => rating > self.value,
            Comp::Less => rating < self.value,
        }
        .then_some(self.dest)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Comp {
    #[default]
    Greater,
    Less,
}

#[derive(Default, Clone, Copy)]
enum Dest {
    #[default]
    Accept,
    Reject,
    Workflow(usize),
}

impl Dest {
    fn parse<'a>(s: &'a str, names: &mut Names<'a>) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            w => Self::Workflow(names.get(w)),
        }
    }
}

#[derive(Clone, Copy)]
struct Ranges([Range; 4]);

impl Ranges {
    fn split(mut self, rule: Rule) -> [Option<Self>; 2] {
        self.0[rule.cat].split(rule.value, rule.comp).map(|r| {
            r.map(|r| {
                self.0[rule.cat] = r;
                self
            })
        })
    }

    fn size(self) -> u64 {
        self.0.into_iter().map(Range::size).product()
    }
}

/// (start, end) inclusive
#[derive(Clone, Copy)]
struct Range(u64, u64);

impl Range {
    fn split(self, mut value: u64, comp: Comp) -> [Option<Self>; 2] {
        if comp == Comp::Less {
            value -= 1;
        }
        let [below, above] = if value < self.0 {
            [None, Some(self)]
        } else if value < self.1 {
            [Some(Self(self.0, value)), Some(Self(value + 1, self.1))]
        } else {
            [Some(self), None]
        };
        match comp {
            Comp::Greater => [above, below],
            Comp::Less => [below, above],
        }
    }

    fn size(self) -> u64 {
        self.1 - self.0 + 1
    }
}

#[derive(Clone, Copy)]
struct Part {
    ratings: [u64; 4],
}

impl Part {
    fn parse(line: &str) -> Self {
        Self {
            ratings: line.uints(),
        }
    }

    fn total(self) -> u64 {
        self.ratings.iter().sum()
    }
}

struct Names<'a> {
    indices: HashMap<&'a str, usize>,
    next: usize,
}

impl<'a> Names<'a> {
    fn new() -> Self {
        Self {
            indices: HashMap::default(),
            next: 0,
        }
    }

    fn get(&mut self, name: &'a str) -> usize {
        *self.indices.entry(name).or_insert_with(|| {
            self.next += 1;
            self.next - 1
        })
    }
}
