aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    shuffle(input, 10007).apply(2019, 10007)
}

fn part_2(input: &[&str]) -> u64 {
    const N: u64 = 119315717514047;
    power(shuffle(input, N), 101741582076661, N)
        .inv(N)
        .apply(2020, N)
}

fn shuffle(input: &[&str], n: u64) -> Transform {
    let mut t = Transform::new(1, 0);
    for line in input {
        t = parse(line, n).mul_rem(t, n);
    }
    t
}

fn parse(line: &str, n: u64) -> Transform {
    if &line[5..6] == "i" {
        return Transform::new(n - 1, n - 1);
    }
    if &line[0..4] == "deal" {
        return Transform::new(line[20..].parse().unwrap(), 0);
    }
    if &line[4..5] == "-" {
        return Transform::new(1, line[5..].parse().unwrap());
    }
    Transform::new(1, (n - line[4..].parse::<u64>().unwrap()) % n)
}

fn power<T>(x: T, mut y: u64, n: u64) -> T
where
    T: Copy + MulRem<u64>,
{
    let mut c = x;
    while y & 1 == 0 {
        c = c.mul_rem(c, n);
        y >>= 1;
    }
    let mut r = c;
    y >>= 1;
    while y > 0 {
        c = c.mul_rem(c, n);
        if y & 1 == 1 {
            r = r.mul_rem(c, n);
        }
        y >>= 1;
    }
    r
}

#[derive(Clone, Copy)]
struct Transform {
    scale: u64,
    translation: u64,
}

impl Transform {
    fn new(scale: u64, translation: u64) -> Transform {
        Transform { scale, translation }
    }

    fn apply(self, value: u64, n: u64) -> u64 {
        (self.scale.mul_rem(value, n) + self.translation) % n
    }

    fn inv(self, n: u64) -> Transform {
        let scale = power(self.scale, n - 2, n);
        Transform {
            scale,
            translation: (n - scale.mul_rem(self.translation, n)) % n,
        }
    }
}

impl MulRem<u64> for Transform {
    fn mul_rem(self, other: Self, n: u64) -> Self {
        Transform {
            scale: self.scale.mul_rem(other.scale, n),
            translation: (self.scale.mul_rem(other.translation, n) + self.translation) % n,
        }
    }
}

impl MulRem<u64> for u64 {
    fn mul_rem(mut self, mut other: Self, n: u64) -> Self {
        let mut r = 0;
        while other > 0 {
            if other & 1 == 1 {
                r = (r + self) % n;
            }
            self = (self * 2) % n;
            other >>= 1;
        }
        r
    }
}

trait MulRem<M> {
    fn mul_rem(self, other: Self, n: M) -> Self;
}
