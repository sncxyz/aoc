use aoc::IterUnwrap;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut graph = Multigraph::new();
    graph.parse(input);
    graph.get_3_cut()
}

struct Multigraph {
    nodes: HashMap<usize, Node>,
}

impl Multigraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::default(),
        }
    }

    fn parse(&mut self, input: aoc::Input) {
        let mut names = Names::new();
        for line in input {
            let u = names.get(&line[..3]);
            for name in line[5..].split(' ') {
                let v = names.get(name);
                self.add_edge(u, v);
                self.add_edge(v, u);
            }
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.nodes
            .entry(u)
            .or_insert(Node::new())
            .edges
            .insert(v, 1);
    }

    fn get_3_cut(&mut self) -> u32 {
        let total = self.nodes.len() as u32;
        let mut start = 0;
        for _ in 0..total - 1 {
            let mut nodes = HashSet::default();
            nodes.insert(start);
            let mut count = self.nodes[&start].count;
            let mut edges = self.nodes[&start].edges.clone();
            while nodes.len() + 1 < self.nodes.len() {
                if edges.values().sum::<u32>() == 3 {
                    return count * (total - count);
                }
                let &u = edges.iter().max_by_key(|&(_, &m)| m).unwrap().0;
                start = u;
                nodes.insert(u);
                let info = &self.nodes[&u];
                count += info.count;
                edges.remove(&u);
                for (&v, &m) in &info.edges {
                    if !nodes.contains(&v) {
                        *edges.entry(v).or_insert(0) += m;
                    }
                }
            }
            let (&v, &cut) = edges.iter().next_uw();
            if cut == 3 {
                return count * (total - count);
            }
            self.contract(start, v);
        }
        unreachable!()
    }

    fn contract(&mut self, u: usize, v: usize) {
        let mut node_u = self.nodes.remove(&u).unwrap();
        let node_v = self.nodes.remove(&v).unwrap();

        node_u.count += node_v.count;
        node_u.edges.remove(&v);

        for (w, m) in node_v.edges {
            if w != u {
                *node_u.edges.entry(w).or_insert(0) += m;
                let node_w = &mut self.nodes.get_mut(&w).unwrap().edges;
                node_w.remove(&v);
                *node_w.entry(u).or_insert(0) += m;
            }
        }

        self.nodes.insert(u, node_u);
    }
}

struct Node {
    count: u32,
    edges: HashMap<usize, u32>,
}

impl Node {
    fn new() -> Self {
        Self {
            count: 1,
            edges: HashMap::default(),
        }
    }
}

struct Names<'a> {
    names: HashMap<&'a str, usize>,
}

impl<'a> Names<'a> {
    fn new() -> Self {
        Self {
            names: HashMap::default(),
        }
    }

    fn get(&mut self, name: &'a str) -> usize {
        let len = self.names.len();
        *self.names.entry(name).or_insert(len)
    }
}
