aoc::parts!(1, 2);

use fxhash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    let reactions = parse(input);
    ore_cost_root("FUEL", 1, &reactions)
}

fn part_2(input: &[&str]) -> impl ToString {
    let reactions = parse(input);
    let mut fuel = 1;
    let mut last = 0;
    while last != fuel {
        last = fuel;
        fuel = fuel * 1_000_000_000_000 / ore_cost_root("FUEL", fuel, &reactions);
    }
    fuel
}

fn ore_cost_root(
    chemical: &str,
    frequency: usize,
    reactions: &HashMap<&str, (usize, Vec<(&str, usize)>)>,
) -> usize {
    let mut leftover = HashMap::default();
    reactions.keys().for_each(|&chemical| {
        leftover.insert(chemical, 0);
    });
    ore_cost((chemical, frequency), reactions, &mut leftover)
}

fn ore_cost(
    chemical: (&str, usize),
    reactions: &HashMap<&str, (usize, Vec<(&str, usize)>)>,
    leftover: &mut HashMap<&str, usize>,
) -> usize {
    if chemical.0 == "ORE" {
        return chemical.1;
    }
    if leftover[chemical.0] >= chemical.1 {
        *leftover.get_mut(chemical.0).unwrap() -= chemical.1;
        return 0;
    }
    let required = chemical.1 - leftover[chemical.0];
    let produced = reactions[chemical.0].0;
    let num_reactions = (required - 1) / produced + 1;
    *leftover.get_mut(chemical.0).unwrap() = num_reactions * produced - required;
    reactions[chemical.0]
        .1
        .iter()
        .map(|input| ore_cost((input.0, input.1 * num_reactions), reactions, leftover))
        .sum()
}

fn parse<'a>(input: &[&'a str]) -> HashMap<&'a str, (usize, Vec<(&'a str, usize)>)> {
    let mut reactions = HashMap::default();
    for line in input {
        let mut parts = line.split(" => ");
        let inputs = parts.next().unwrap().split(", ").map(parse_pair).collect();
        let output = parse_pair(parts.next().unwrap());
        reactions.insert(output.0, (output.1, inputs));
    }
    reactions
}

fn parse_pair(pair: &str) -> (&str, usize) {
    let mut parts = pair.split(' ');
    let frequency = parts.next().unwrap().parse().unwrap();
    let chemical = parts.next().unwrap();
    (chemical, frequency)
}
