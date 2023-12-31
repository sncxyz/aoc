use core::ops::Sub;

use aoc::{IterUnwrap, Parse};
use nd::{v, v3, Matrix, Vec2, Vec3};
use num::{BigRational, FromPrimitive, ToPrimitive};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let hailstones: Vec<_> = input.lines().map(Hailstone1::parse).collect();
    let mut total = 0;
    for (i, &a) in hailstones.iter().enumerate() {
        for &b in hailstones.iter().skip(i + 1) {
            total += a.paths_cross(b) as u32;
        }
    }
    total
}

fn part_2(input: aoc::Input) -> i64 {
    let [a, b, c, d] = input.lines().map(Hailstone2::parse).collect_n();
    let aa = a.pos.cross(a.vel);
    let bb = b.pos.cross(b.vel);
    let cc = c.pos.cross(c.vel);
    let dd = d.pos.cross(d.vel);
    let ab = a - b;
    let ac = a - c;
    let ad = a - d;
    let rhs = Matrix::col(
        [
            aa.x - bb.x,
            aa.y - bb.y,
            aa.x - cc.x,
            aa.y - cc.y,
            aa.x - dd.x,
            aa.y - dd.y,
        ]
        .map(|e| BigRational::from_i64(e).unwrap()),
    );
    let mat = Matrix::new(
        [
            [0, ab.vel.z, -ab.vel.y, 0, -ab.pos.z, ab.pos.y],
            [-ab.vel.z, 0, ab.vel.x, ab.pos.z, 0, -ab.pos.x],
            [0, ac.vel.z, -ac.vel.y, 0, -ac.pos.z, ac.pos.y],
            [-ac.vel.z, 0, ac.vel.x, ac.pos.z, 0, -ac.pos.x],
            [0, ad.vel.z, -ad.vel.y, 0, -ad.pos.z, ad.pos.y],
            [-ad.vel.z, 0, ad.vel.x, ad.pos.z, 0, -ad.pos.x],
        ]
        .map(|r| r.map(|e| BigRational::from_i64(e).unwrap())),
    );
    mat.solve(rhs)
        .unwrap()
        .into_iter_all()
        .map(|e| e.to_i64().unwrap())
        .take(3)
        .sum()
}

#[derive(Clone, Copy)]
struct Hailstone1 {
    pos: Vec2<i128>,
    vel: Vec2<i128>,
}

impl Hailstone1 {
    fn parse(line: &str) -> Self {
        let values: [_; 6] = line.ints();
        Self {
            pos: v(values[0], values[1]),
            vel: v(values[3], values[4]),
        }
    }

    fn paths_cross(self, other: Self) -> bool {
        const LOWER: i128 = 200_000_000_000_000;
        const UPPER: i128 = 400_000_000_000_000;

        let denom_a = other.vel.perp_dot(self.vel);
        if denom_a == 0 {
            // parallel
            return false;
        }
        let denom_b = -denom_a;
        let numer_a = other.vel.perp_dot(other.pos - self.pos);
        let numer_b = self.vel.perp_dot(self.pos - other.pos);
        if denom_a.signum() * numer_a.signum() == -1 {
            // in self's past
            return false;
        }
        if denom_b.signum() * numer_b.signum() == -1 {
            // in other's past
            return false;
        }
        let pos = self.pos * denom_a + self.vel * numer_a;
        let (lower, upper) = match (LOWER * denom_a, UPPER * denom_a) {
            (a, b) if a > b => (b, a),
            bounds => bounds,
        };
        pos.x >= lower && pos.x <= upper && pos.y >= lower && pos.y <= upper
    }
}

#[derive(Clone, Copy)]
struct Hailstone2 {
    pos: Vec3<i64>,
    vel: Vec3<i64>,
}

impl Hailstone2 {
    fn parse(line: &str) -> Self {
        let values: [_; 6] = line.ints();
        Self {
            pos: v3(values[0], values[1], values[2]),
            vel: v3(values[3], values[4], values[5]),
        }
    }
}

impl Sub for Hailstone2 {
    type Output = Hailstone2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            pos: self.pos - rhs.pos,
            vel: self.vel - rhs.vel,
        }
    }
}
