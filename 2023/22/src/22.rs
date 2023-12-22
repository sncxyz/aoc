use aoc::Parse;
use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let bricks = Bricks::new(input);
    (0..bricks.len)
        .filter(|&i| {
            for &j in &bricks.support[i].supports {
                if bricks.support[j].supported_by < 2 {
                    return false;
                }
            }
            true
        })
        .count()
}

fn part_2(input: aoc::Input) -> u32 {
    let bricks = Bricks::new(input);
    (0..bricks.len)
        .map(|i| bricks.disintegrate(i, &mut vec![0; bricks.len]))
        .sum()
}

struct Bricks {
    support: Vec<Support>,
    len: usize,
}

impl Bricks {
    fn new(input: aoc::Input) -> Self {
        // parse bricks
        let mut bricks: Vec<_> = input.lines().map(Brick::parse).collect();
        bricks.sort_unstable_by_key(|&brick| match brick {
            Brick::Horizontal { height, .. } => height,
            Brick::Vertical { bottom, .. } => bottom,
        });

        // calculate horizontal bounds
        let (mut min, mut max) = (Vector::MAX, Vector::MIN);
        for &brick in &bricks {
            match brick {
                Brick::Horizontal { start, end, .. } => {
                    min = min.min(start);
                    max = max.max(end);
                }
                Brick::Vertical { pos, .. } => {
                    min = min.min(pos);
                    max = max.max(pos);
                }
            }
        }

        // setup
        let mut heights = Grid::new(max.x - min.x + 1, max.y - min.y + 1, (0, 0));
        let mut support: Vec<Support> = Vec::with_capacity(bricks.len());

        // form a mapping from every brick to a set of bricks it supports and the number of bricks supporting it
        for (i, brick) in bricks.into_iter().enumerate() {
            match brick {
                Brick::Horizontal { start, end, .. } => {
                    let iter = BrickIter::new(start, end);
                    let max_height = iter.clone().map(|pos| heights[pos - min].0).max().unwrap();
                    let mut supported_by = Vec::new();
                    for pos in iter.clone() {
                        let (h, j) = heights[pos - min];
                        if h != 0 && h == max_height {
                            if supported_by.last() != Some(&j) {
                                supported_by.push(j);
                                support[j].supports.push(i);
                            }
                        }
                        heights[pos - min] = (max_height + 1, i);
                    }
                    support.push(Support::new(supported_by.len() as u32));
                }
                Brick::Vertical { bottom, top, pos } => {
                    let (h, j) = heights[pos - min];
                    heights[pos - min] = (h + 1 + top - bottom, i);
                    if h == 0 {
                        support.push(Support::new(0));
                    } else {
                        support[j].supports.push(i);
                        support.push(Support::new(1));
                    }
                }
            }
        }

        Self {
            len: support.len(),
            support,
        }
    }

    fn disintegrate(&self, i: usize, counts: &mut [u32]) -> u32 {
        let mut total = 0;
        for &j in &self.support[i].supports {
            counts[j] += 1;
            if counts[j] == self.support[j].supported_by {
                total += 1 + self.disintegrate(j, counts);
            }
        }
        total
    }
}

struct Support {
    supported_by: u32,
    supports: Vec<usize>,
}

impl Support {
    fn new(supported_by: u32) -> Self {
        Self {
            supported_by,
            supports: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
enum Brick {
    Horizontal {
        start: Vector,
        end: Vector,
        height: i64,
    },
    Vertical {
        bottom: i64,
        top: i64,
        pos: Vector,
    },
}

impl Brick {
    fn parse(line: &str) -> Self {
        let coords: [_; 6] = line.ints();
        if coords[0] != coords[3] || coords[1] != coords[4] {
            let (a, b) = (v(coords[0], coords[1]), v(coords[3], coords[4]));
            Self::Horizontal {
                start: a.min(b),
                end: a.max(b),
                height: coords[2],
            }
        } else {
            Self::Vertical {
                bottom: coords[2].min(coords[5]),
                top: coords[2].max(coords[5]),
                pos: v(coords[0], coords[1]),
            }
        }
    }
}

#[derive(Clone)]
struct BrickIter {
    pos: Vector,
    d: Vector,
    end: Vector,
}

impl BrickIter {
    fn new(start: Vector, end: Vector) -> Self {
        let d = (end - start).signum();
        Self {
            pos: start,
            d,
            end: end + d,
        }
    }
}

impl Iterator for BrickIter {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.end {
            return None;
        }
        let ret = self.pos;
        self.pos += self.d;
        Some(ret)
    }
}
