use std::ops::Index;

use rustc_hash::FxHashMap as HashMap;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let network = Network::parse(input);
    network.simulate(network.aaa, |n| n == network.zzz)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let network = Network::parse(input);
    network
        .start
        .iter()
        .copied()
        .map(|n| network.simulate(n, |n| network.end.contains(&n)) / network.n)
        .product::<u64>()
        * network.n
}

struct Network {
    instrs: Vec<Instr>,
    n: u64,
    nodes: Vec<Node>,
    aaa: usize,
    zzz: usize,
    start: Vec<usize>,
    end: Vec<usize>,
}

impl Network {
    fn parse(input: aoc::Input) -> Self {
        let instrs: Vec<_> = input[0].bytes().map(Instr::parse).collect();
        let n = instrs.len() as u64;

        let mut aaa = 0;
        let mut zzz = 0;
        let mut start = Vec::new();
        let mut end = Vec::new();

        let mut nodes = Vec::with_capacity(input.len() - 2);
        let mut indices = HashMap::default();

        for (i, bytes) in input.lines().skip(2).map(str::as_bytes).enumerate() {
            let name = NodeName::parse(&bytes[0..3]);

            indices.insert(name, i);
            nodes.push(Paths::new(&bytes[7..10], &bytes[12..15]));

            match name.bytes {
                [b'A', b'A', b'A'] => {
                    aaa = i;
                    start.push(i)
                }
                [b'Z', b'Z', b'Z'] => {
                    zzz = i;
                    end.push(i);
                }
                [_, _, b'A'] => start.push(i),
                [_, _, b'Z'] => end.push(i),
                _ => (),
            }
        }

        let nodes = nodes
            .into_iter()
            .map(|paths| Node::new(paths, &indices))
            .collect();

        Self {
            instrs,
            n,
            nodes,
            aaa,
            zzz,
            start,
            end,
        }
    }

    fn simulate(&self, start: usize, stop: impl Fn(usize) -> bool) -> u64 {
        let mut steps = 0;
        let mut i = 0;
        let mut node = start;
        while !stop(node) {
            node = self.nodes[node][self.instrs[i]];
            steps += 1;
            i = (i + 1) % self.instrs.len();
        }
        steps
    }
}

#[derive(Clone, Copy)]
enum Instr {
    Left,
    Right,
}

impl Instr {
    #[inline(always)]
    fn parse(byte: u8) -> Self {
        match byte {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

struct Node {
    left: usize,
    right: usize,
}

impl Node {
    fn new(paths: Paths, indices: &HashMap<NodeName, usize>) -> Self {
        Self {
            left: indices[&paths.left],
            right: indices[&paths.right],
        }
    }
}

impl Index<Instr> for Node {
    type Output = usize;

    #[inline(always)]
    fn index(&self, index: Instr) -> &Self::Output {
        match index {
            Instr::Left => &self.left,
            Instr::Right => &self.right,
        }
    }
}

struct Paths {
    left: NodeName,
    right: NodeName,
}

impl Paths {
    #[inline(always)]
    fn new(left: &[u8], right: &[u8]) -> Self {
        Self {
            left: NodeName::parse(left),
            right: NodeName::parse(right),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NodeName {
    bytes: [u8; 3],
}

impl NodeName {
    #[inline(always)]
    fn parse(bytes: &[u8]) -> Self {
        Self {
            bytes: [bytes[0], bytes[1], bytes[2]],
        }
    }
}
