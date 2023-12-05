use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut map = Map::parse(input);
    input[0].uints_iter().map(|s| map.apply(s)).min().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    // parse seed ranges
    let numbers: Vec<_> = input[0].uints_iter().collect();
    let mut seeds: Vec<_> = numbers
        .chunks(2)
        .map(|c| Range::new(c[0], c[0] + c[1] - 1))
        .collect();
    seeds.sort_unstable_by_key(|s| s.start);

    let mut offsets = Map::parse(input).offsets;
    offsets.sort_unstable_by_key(|o| o.dest.start);

    // fill gaps in destination with f(x) = x mappings (doesn't occur in inputs)
    let mut i = 0;
    while i < offsets.len() - 1 {
        let (a, b) = (offsets[i].dest.end, offsets[i + 1].dest.start);
        if a + 1 < b {
            offsets.insert(i + 1, Offset::simple(a + 1, b - 1));
            i += 1;
        }
        i += 1;
    }

    // return smallest seed if smaller than entire map (doesn't occur in inputs)
    if seeds[0].start < offsets[0].dest.start {
        return seeds[0].start;
    }

    // find smallest destination range with seed that maps into it
    for &offset in &offsets {
        let src = offset.src;
        for &range in &seeds {
            if range.start > src.end {
                break;
            }
            if src.start <= range.end {
                // return smallest mapped number
                return offset.apply(range.start.max(src.start));
            }
        }
    }

    // if none found, return smallest seed
    seeds[0].start
}

struct Map {
    offsets: Vec<Offset>,
}

impl Map {
    fn parse(input: aoc::Input) -> Self {
        input.as_lines()[2..]
            .split(|line| line.is_empty())
            .map(Self::parse_single)
            .reduce(Self::compose)
            .unwrap()
    }

    fn parse_single(lines: &[&str]) -> Self {
        let ranges: Vec<_> = lines.iter().skip(1).copied().map(Offset::parse).collect();
        Self { offsets: ranges }
    }

    /// produce map representing applying self then other
    fn compose(mut self, mut other: Self) -> Self {
        self.offsets.sort_unstable_by_key(|o| o.dest.start);
        other.offsets.sort_unstable_by_key(|o| o.src.start);

        let mut offsets = Vec::new();

        while let (Some(&a), Some(&b)) = (self.offsets.last(), other.offsets.last()) {
            if a.dest.start > b.src.end {
                self.offsets.pop();
                offsets.push(a);
            } else if b.src.start > a.dest.end {
                other.offsets.pop();
                offsets.push(b);
            } else if a.dest.end > b.src.end {
                self.offsets.pop();
                let (a1, a2) = a.split_dest(b.src.end);
                offsets.push(a2);
                self.offsets.push(a1);
            } else if b.src.end > a.dest.end {
                other.offsets.pop();
                let (b1, b2) = b.split_src(a.dest.end);
                offsets.push(b2);
                other.offsets.push(b1);
            } else {
                self.offsets.pop();
                other.offsets.pop();
                if a.dest.start > b.src.start {
                    let (b1, b2) = b.split_src(a.dest.start - 1);
                    offsets.push(a.compose(b2));
                    other.offsets.push(b1);
                } else if b.src.start > a.dest.start {
                    let (a1, a2) = a.split_dest(b.src.start - 1);
                    offsets.push(a2.compose(b));
                    self.offsets.push(a1);
                } else {
                    offsets.push(a.compose(b));
                }
            }
        }

        offsets.append(&mut self.offsets);
        offsets.append(&mut other.offsets);

        Self { offsets }
    }

    /// self must be sorted in increasing order of source range
    fn apply(&self, n: u64) -> u64 {
        for &offset in &self.offsets {
            if n >= offset.src.start && n <= offset.src.end {
                return offset.apply(n);
            }
        }
        n
    }
}

#[derive(Clone, Copy)]
struct Offset {
    src: Range,
    dest: Range,
}

impl Offset {
    #[inline(always)]
    fn new(src: Range, dest: Range) -> Self {
        Self { src, dest }
    }

    fn simple(start: u64, end: u64) -> Self {
        Self::new(Range::new(start, end), Range::new(start, end))
    }

    fn parse(line: &str) -> Self {
        let [dest, src, len] = line.uints();
        Self::new(
            Range::new(src, src + len - 1),
            Range::new(dest, dest + len - 1),
        )
    }

    fn split_src(self, at: u64) -> (Self, Self) {
        self.split(at, at + self.dest.start - self.src.start)
    }

    fn split_dest(self, at: u64) -> (Self, Self) {
        self.split(at + self.src.start - self.dest.start, at)
    }

    fn split(self, at_src: u64, at_dest: u64) -> (Self, Self) {
        let (src1, src2) = self.src.split(at_src);
        let (dest1, dest2) = self.dest.split(at_dest);
        (Self::new(src1, dest1), Self::new(src2, dest2))
    }

    fn compose(self, other: Self) -> Self {
        debug_assert_eq!(self.dest, other.src);
        Self::new(self.src, other.dest)
    }

    fn apply(self, number: u64) -> u64 {
        debug_assert!(number >= self.src.start);
        debug_assert!(number <= self.src.end);
        number + self.dest.start - self.src.start
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    #[inline(always)]
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    /// [start, end] -> [start, at], (at, end]
    fn split(self, at: u64) -> (Self, Self) {
        debug_assert!(at >= self.start);
        debug_assert!(at < self.end);
        (Self::new(self.start, at), Self::new(at + 1, self.end))
    }
}
