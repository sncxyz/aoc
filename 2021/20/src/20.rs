aoc::parts!(1, 2);

use grid::prelude::*;

fn part_1(input: &[&str]) -> impl ToString {
    let mut image = Image::new(input, 4);
    for _ in 0..2 {
        image.enhance();
    }
    image.count()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut image = Image::new(input, 100);
    for _ in 0..50 {
        image.enhance();
    }
    image.count()
}

struct Image {
    algorithm: [bool; 512],
    image: Grid<bool>,
}

impl Image {
    fn new(input: &[&str], pad: i64) -> Image {
        let mut algorithm = [false; 512];
        for (i, c) in input[0].chars().enumerate() {
            if c == '#' {
                algorithm[i] = true;
            }
        }
        let dim = Grid::new(input[2].len() as i64, input.len() as i64 - 2, ());
        let mut image = Grid::default(dim.width() + pad * 2, dim.height() + pad * 2);
        let mut pos = dim.positions().map(|pos| pos + v(pad, pad));
        for line in &input[2..] {
            for c in line.chars() {
                image[pos.next().unwrap()] = c == '#';
            }
        }
        Image { algorithm, image }
    }

    fn enhance(&mut self) {
        self.image = Grid::from_fn(self.image.width() - 2, self.image.height() - 2, |pos| {
            self.new_value(pos)
        });
    }

    fn new_value(&self, pos: Vector) -> bool {
        let mut i = 0;
        for y in 0..3 {
            for x in 0..3 {
                i <<= 1;
                if self.image[pos + v(x, y)] {
                    i += 1;
                }
            }
        }
        self.algorithm[i]
    }

    fn count(self) -> usize {
        self.image.into_iter().filter(|x| *x).count()
    }
}
