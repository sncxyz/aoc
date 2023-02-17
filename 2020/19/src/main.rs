aoc::parts!(1, 2);

use regex::Regex;

fn part_1(input: &[&str]) -> impl ToString {
    let (i, mut rules) = parse_rules(input);
    let regex = Regex::new(&format!("^{}$", resolve_rule(&mut rules, 0))).unwrap();
    input[i..]
        .iter()
        .map(|x| regex.is_match(x))
        .filter(|x| *x)
        .count()
}

fn part_2(input: &[&str]) -> impl ToString {
    let (i, mut rules) = parse_rules(input);
    rules[8] = Rule::Unresolved(vec![
        vec![42],
        vec![42, 42],
        vec![42, 42, 42],
        vec![42, 42, 42, 42],
        vec![42, 42, 42, 42, 42],
    ]);
    rules[11] = Rule::Unresolved(vec![
        vec![42, 31],
        vec![42, 42, 31, 31],
        vec![42, 42, 42, 31, 31, 31],
        vec![42, 42, 42, 42, 31, 31, 31, 31],
    ]);
    let regex = Regex::new(&format!("^{}$", resolve_rule(&mut rules, 0))).unwrap();
    input[i..]
        .iter()
        .map(|x| regex.is_match(x))
        .filter(|x| *x)
        .count()
}

fn parse_rules(input: &[&str]) -> (usize, Vec<Rule>) {
    let len = input.iter().copied().position(str::is_empty).unwrap();
    let mut rules = vec![Rule::Unresolved(Vec::new()); len];
    for line in &input[..len] {
        let mut parts = line.split(": ");
        let n: usize = parts.next().unwrap().parse().unwrap();
        rules[n] = Rule::new(parts.next().unwrap());
    }
    (len + 1, rules)
}

fn resolve_rule(rules: &mut [Rule], number: usize) -> String {
    match rules[number].clone() {
        Rule::Resolved(regex) => regex,
        Rule::Unresolved(definition) => {
            let mut regex = "(".to_string();
            for (i, set) in definition.iter().enumerate() {
                regex += "(";
                for &rule in set {
                    regex += &resolve_rule(rules, rule);
                }
                regex += ")";
                if i < definition.len() - 1 {
                    regex += "|";
                }
            }
            regex += ")";
            rules[number] = Rule::Resolved(regex.clone());
            regex
        }
    }
}

#[derive(Clone)]
enum Rule {
    Resolved(String),
    Unresolved(Vec<Vec<usize>>),
}

impl Rule {
    fn new(string: &str) -> Rule {
        if string.starts_with('"') {
            Rule::Resolved(string.chars().nth(1).unwrap().to_string())
        } else {
            Rule::Unresolved(
                string
                    .split(" | ")
                    .map(|g| g.split(' ').map(|n| n.parse().unwrap()).collect())
                    .collect(),
            )
        }
    }
}
