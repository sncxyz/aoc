aoc::parts!(1, 2);

use rustc_hash::FxHashSet as HashSet;

fn part_1(input: &[&str]) -> impl ToString {
    let mut i = 0;
    let fields = parse_fields(input, &mut i);
    i += 5;
    let mut total = 0;
    while i < input.len() {
        if let Some(x) = invalidity(&parse_ticket(&input[i]), &fields) {
            total += x;
        }
        i += 1;
    }
    total
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut i = 0;
    let fields = parse_fields(input, &mut i);
    let num_fields = fields.len();
    let all: HashSet<usize> = (0..num_fields).collect();
    let mut possibilities = vec![all; num_fields];
    i += 2;
    let yours = parse_ticket(&input[i]);
    i += 3;
    while i < input.len() {
        let ticket = parse_ticket(&input[i]);
        if invalidity(&ticket, &fields).is_none() {
            for (i, value) in ticket.into_iter().enumerate() {
                possibilities[i].retain(|&j| fields[j].valid(value));
            }
        }
        i += 1;
    }
    let mut ordered = vec![0; num_fields];
    while let Some((i, field)) = reduce(&possibilities) {
        ordered[i] = field;
        for fields in possibilities.iter_mut() {
            fields.remove(&field);
        }
    }
    let mut product = 1;
    for i in 0..num_fields {
        if &fields[ordered[i]].name[..2] == "de" {
            product *= yours[i] as u64;
        }
    }
    product
}

fn parse_fields<'a>(input: &'a [&str], i: &mut usize) -> Vec<Field<'a>> {
    let mut fields = Vec::new();
    while !input[*i].is_empty() {
        fields.push(Field::new(&input[*i]));
        *i += 1;
    }
    fields
}

fn parse_ticket(line: &str) -> Vec<u32> {
    line.split(',').map(|n| n.parse().unwrap()).collect()
}

fn reduce(possibilities: &[HashSet<usize>]) -> Option<(usize, usize)> {
    for (i, fields) in possibilities.iter().enumerate() {
        if fields.len() == 1 {
            return Some((i, *fields.iter().next().unwrap()));
        }
    }
    None
}

fn invalidity(ticket: &[u32], fields: &[Field]) -> Option<u32> {
    let mut total = 0;
    let mut ticket_valid = true;
    for &value in ticket {
        let mut valid = false;
        for field in fields {
            if field.valid(value) {
                valid = true;
                break;
            }
        }
        if !valid {
            total += value;
            ticket_valid = false;
        }
    }
    if ticket_valid {
        return None;
    }
    Some(total)
}

struct Field<'a> {
    name: &'a str,
    range1: (u32, u32),
    range2: (u32, u32),
}

impl<'a> Field<'a> {
    fn new(line: &'a str) -> Self {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        parts = parts.next().unwrap().split(" or ");
        let mut range1 = parts.next().unwrap().split('-');
        let mut range2 = parts.next().unwrap().split('-');
        Field {
            name,
            range1: (
                range1.next().unwrap().parse().unwrap(),
                range1.next().unwrap().parse().unwrap(),
            ),
            range2: (
                range2.next().unwrap().parse().unwrap(),
                range2.next().unwrap().parse().unwrap(),
            ),
        }
    }

    fn valid(&self, value: u32) -> bool {
        (value >= self.range1.0 && value <= self.range1.1)
            || (value >= self.range2.0 && value <= self.range2.1)
    }
}
