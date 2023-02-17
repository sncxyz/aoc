aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    marker(input, 4)
}

fn part_2(input: &[&str]) -> impl ToString {
    marker(input, 14)
}

fn marker(input: &[&str], len: usize) -> usize {
    const A: u8 = b'a';
    let mut ms = Multiset::<26>::new();
    input[0].bytes().take(len).for_each(|b| ms.insert(b - A));
    if ms.len == len {
        return len;
    }
    input[0]
        .as_bytes()
        .windows(len + 1)
        .scan(ms, |s, b| {
            s.insert(b[len] - A);
            s.remove(b[0] - A);
            Some(s.len)
        })
        .position(|l| l == len)
        .unwrap()
        + len
        + 1
}

struct Multiset<const N: usize> {
    set: [u8; N],
    len: usize,
}

impl<const N: usize> Multiset<N> {
    fn new() -> Self {
        Self {
            set: [0; N],
            len: 0,
        }
    }

    fn insert(&mut self, value: impl Into<usize>) {
        let count = &mut self.set[value.into()];
        if *count == 0 {
            self.len += 1;
        }
        *count += 1;
    }

    fn remove(&mut self, value: impl Into<usize>) {
        let count = &mut self.set[value.into()];
        *count -= 1;
        if *count == 0 {
            self.len -= 1;
        }
    }
}
