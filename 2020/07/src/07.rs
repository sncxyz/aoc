aoc::parts!(1, 2);

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn part_1(input: &[&str]) -> impl ToString {
    let mut bags: HashMap<&str, Vec<&str>> = HashMap::default();
    for line in input {
        let (parent, children) = parse(line);
        for bag in children {
            bags.entry(bag.name).or_default().push(parent);
        }
    }
    let mut parents = HashSet::default();
    get_parents(&bags, &mut parents, "shiny gold");
    parents.len() - 1
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut bags: HashMap<&str, Vec<Bag>> = HashMap::default();
    for line in input {
        let (parent, children) = parse(line);
        bags.insert(parent, children);
    }
    let mut contains = HashMap::default();
    get_children(&bags, &mut contains, "shiny gold");
    contains["shiny gold"]
}

fn get_parents<'a>(
    bags: &HashMap<&'a str, Vec<&'a str>>,
    parents: &mut HashSet<&'a str>,
    bag: &'a str,
) {
    if parents.insert(bag) {
        if let Some(others) = bags.get(bag) {
            for bag in others {
                get_parents(bags, parents, bag);
            }
        }
    }
}

fn get_children<'a>(
    bags: &HashMap<&'a str, Vec<Bag<'a>>>,
    children: &mut HashMap<&'a str, u32>,
    bag: &'a str,
) {
    if !children.contains_key(bag) {
        let mut count = 0;
        if let Some(others) = bags.get(bag) {
            for bag in others {
                get_children(bags, children, bag.name);
                count += bag.freq * (1 + children[bag.name]);
            }
        }
        children.insert(bag, count);
    }
}

struct Bag<'a> {
    name: &'a str,
    freq: u32,
}

fn parse(line: &str) -> (&str, Vec<Bag>) {
    let mut parts = line.split(" bags contain ");
    let parent = parts.next().unwrap();
    let parts = parts.next().unwrap();
    let mut children = Vec::new();
    for part in parts[..parts.len() - 1].split(", ") {
        let start = part.find(' ').unwrap();
        let end = part.find(" bag").unwrap();
        if let Ok(freq) = part[..start].parse() {
            let name = &part[start + 1..end];
            children.push(Bag { name, freq })
        }
    }
    (parent, children)
}
