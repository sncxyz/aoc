aoc::parts!(1, 2);

use regex::Regex;

fn part_1(input: &[&str]) -> impl ToString {
    get_passports(input).len()
}

fn part_2(input: &[&str]) -> impl ToString {
    let rules = Rules::init();
    get_passports(input)
        .into_iter()
        .filter(|p| p.is_valid(&rules))
        .count()
}

fn get_passports(input: &[&str]) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut current = Passport::default();
    for line in input {
        if line.is_empty() {
            if current.count == 7 {
                passports.push(current);
            }
            current = Passport::default();
        } else {
            for field in line.split(' ') {
                let mut parts = field.split(':');
                let name = parts.next().unwrap();
                let value = parts.next().unwrap().to_string();
                current.count += 1;
                match name {
                    "byr" => current.byr = value,
                    "iyr" => current.iyr = value,
                    "eyr" => current.eyr = value,
                    "hgt" => current.hgt = value,
                    "hcl" => current.hcl = value,
                    "ecl" => current.ecl = value,
                    "pid" => current.pid = value,
                    _ => current.count -= 1,
                }
            }
        }
    }
    if current.count == 7 {
        passports.push(current);
    }
    passports
}

#[derive(Clone, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    count: u8,
}

impl Passport {
    fn is_valid(&self, rules: &Rules) -> bool {
        if !Passport::four_range(rules, &self.byr, 1920, 2002) {
            return false;
        }
        if !Passport::four_range(rules, &self.iyr, 2010, 2020) {
            return false;
        }
        if !Passport::four_range(rules, &self.eyr, 2020, 2030) {
            return false;
        }

        if !rules.height.is_match(&self.hgt) {
            return false;
        }
        let number: u16 = self.hgt[..self.hgt.len() - 2].parse().unwrap();
        if match &self.hgt[self.hgt.len() - 2..] {
            "cm" => !(150..=193).contains(&number),
            "in" => !(59..=76).contains(&number),
            _ => true,
        } {
            return false;
        }

        if !rules.hair.is_match(&self.hcl) {
            return false;
        }

        if !rules.eye.is_match(&self.ecl) {
            return false;
        }

        if !rules.id.is_match(&self.pid) {
            return false;
        }

        true
    }

    fn four_range(rules: &Rules, value: &str, min: u16, max: u16) -> bool {
        if !rules.four.is_match(value) {
            return false;
        }
        let number: u16 = value.parse().unwrap();
        if number < min || number > max {
            return false;
        }
        true
    }
}

struct Rules {
    four: Regex,
    height: Regex,
    hair: Regex,
    eye: Regex,
    id: Regex,
}

impl Rules {
    fn init() -> Rules {
        Rules {
            four: Regex::new(r"^\d{4}$").unwrap(),
            height: Regex::new(r"^\d+\w\w$").unwrap(),
            hair: Regex::new(r"^#(\d|[a-f]){6}$").unwrap(),
            eye: Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap(),
            id: Regex::new(r"^\d{9}$").unwrap(),
        }
    }
}
