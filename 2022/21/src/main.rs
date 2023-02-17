aoc::parts!(1, 2);

use fxhash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    if let Value::Number(n) = get_root(input, false) {
        return n;
    }
    panic!()
}

fn part_2(input: &[&str]) -> impl ToString {
    if let Value::Operation { lhs, rhs, .. } = get_root(input, true) {
        let (mut n, mut expr) = match (*lhs, *rhs) {
            (Value::Number(n), expr) => (n, expr),
            (expr, Value::Number(n)) => (n, expr),
            _ => panic!(),
        };
        while let Value::Operation { lhs, op, rhs } = expr {
            let (m, new_expr, side) = match (*lhs, *rhs) {
                (Value::Number(m), expr) => (m, expr, false),
                (expr, Value::Number(m)) => (m, expr, true),
                _ => panic!(),
            };
            use Operator::*;
            n = match (op, side) {
                (Add, _) => n - m,
                (Mul, _) => n / m,
                (Sub, true) => n + m,
                (Sub, false) => m - n,
                (Div, true) => n * m,
                (Div, false) => m / n,
            };
            expr = new_expr;
        }
        return n;
    }
    panic!()
}

fn get_root(input: &[&str], check_human: bool) -> Value {
    let mut evaluated = HashMap::default();
    let mut monkeys: Vec<_> = input.iter().map(Monkey::parse).collect();
    loop {
        let mut i = 0;
        while i < monkeys.len() {
            let name = monkeys[i].name;
            match monkeys[i].yells {
                Yells::Number(n) => {
                    if check_human && name == "humn" {
                        evaluated.insert(name, Value::Human);
                    } else {
                        evaluated.insert(name, Value::Number(n));
                    }
                }
                Yells::Operation { lhs, op, rhs } => {
                    if evaluated.contains_key(lhs) && evaluated.contains_key(rhs) {
                        let lhs = evaluated.remove(lhs).unwrap();
                        let rhs = evaluated.remove(rhs).unwrap();
                        use Operator::*;
                        let value = if let (Value::Number(x), Value::Number(y)) = (&lhs, &rhs) {
                            Value::Number(match op {
                                Add => x + y,
                                Sub => x - y,
                                Mul => x * y,
                                Div => x / y,
                            })
                        } else {
                            Value::Operation {
                                lhs: Box::new(lhs),
                                op,
                                rhs: Box::new(rhs),
                            }
                        };
                        if name == "root" {
                            return value;
                        }
                        evaluated.insert(name, value);
                    } else {
                        i += 1;
                        continue;
                    }
                }
            }
            monkeys.swap_remove(i);
        }
    }
}

enum Value {
    Number(i64),
    Human,
    Operation {
        lhs: Box<Value>,
        op: Operator,
        rhs: Box<Value>,
    },
}

struct Monkey<'a> {
    name: &'a str,
    yells: Yells<'a>,
}

impl<'a> Monkey<'a> {
    fn parse(line: &'a impl AsRef<str>) -> Self {
        Self {
            name: &line.as_ref()[..4],
            yells: Yells::parse(&line.as_ref()[6..]),
        }
    }
}

enum Yells<'a> {
    Number(i64),
    Operation {
        lhs: &'a str,
        op: Operator,
        rhs: &'a str,
    },
}

impl<'a> Yells<'a> {
    fn parse(s: &'a str) -> Self {
        if let Ok(n) = s.parse() {
            Self::Number(n)
        } else {
            let mut parts = s.split(' ');
            let lhs = parts.next().unwrap();
            use Operator::*;
            let op = match parts.next().unwrap() {
                "+" => Add,
                "-" => Sub,
                "*" => Mul,
                "/" => Div,
                _ => panic!(),
            };
            let rhs = parts.next().unwrap();
            Self::Operation { lhs, op, rhs }
        }
    }
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
