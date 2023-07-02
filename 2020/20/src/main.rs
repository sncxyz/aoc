aoc::parts!(1, 2);

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use grid::{constants::*, v, Grid, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    let (tiles, _, minx, miny) = parse(input);
    let mut total = 1;
    for tiley in 0..12 {
        for tilex in 0..12 {
            if [(0, 0), (11, 0), (0, 11), (11, 11)].contains(&(tilex, tiley)) {
                total *= tiles
                    .get(&(tilex as i8 + minx, tiley as i8 + miny))
                    .unwrap()
                    .id as u64;
            }
        }
    }
    total
}

fn part_2(input: &[&str]) -> impl ToString {
    let (tiles, raw, minx, miny) = parse(input);
    let mut image = Grid::default(96, 96);
    for tiley in 0..12 {
        for tilex in 0..12 {
            let tile = tiles
                .get(&(tilex as i8 + minx, tiley as i8 + miny))
                .unwrap();
            for y in 1..9 {
                for x in 1..9 {
                    let (mut newx, mut newy) = (x, y);
                    if tile.flipped {
                        newy = 9 - newy;
                    }
                    for _ in 0..tile.rotation {
                        let temp = newx;
                        newx = newy;
                        newy = 9 - temp;
                    }
                    image[v!(tilex * 8 + newx - 1, tiley * 8 + newy - 1)] =
                        raw[&tile.id].data[v!(x, y)];
                }
            }
        }
    }
    let sea_monster: Vec<Vector> = vec![
        v!(0, 1),
        v!(1, 0),
        v!(4, 0),
        v!(5, 1),
        v!(6, 1),
        v!(7, 0),
        v!(10, 0),
        v!(11, 1),
        v!(12, 1),
        v!(13, 0),
        v!(16, 0),
        v!(17, 1),
        v!(18, 1),
        v!(18, 2),
        v!(19, 1),
    ];
    for r in 0..4 {
        if remove_sea_monsters(&mut image, orient_sea_monster(&sea_monster, false, r))
            || remove_sea_monsters(&mut image, orient_sea_monster(&sea_monster, true, r))
        {
            break;
        }
    }
    image.into_iter().filter(|x| *x).count()
}

fn parse(input: &[&str]) -> (HashMap<(i8, i8), Tile>, HashMap<u16, Raw>, i8, i8) {
    let mut ids: HashSet<u16> = HashSet::default();
    let mut raw: HashMap<u16, Raw> = HashMap::default();
    let mut tiles: HashMap<(i8, i8), Tile> = HashMap::default();
    let mut i = 0;
    while i < input.len() {
        let id = input[i][5..9].parse().unwrap();
        let tile = Raw::from(&input[i + 1..i + 11]);
        if i == 0 {
            tiles.insert(
                (0, 0),
                Tile {
                    id,
                    border: tile.border,
                    flipped: false,
                    rotation: 0,
                    exposed: [0, 1, 2, 3].iter().cloned().collect(),
                },
            );
        } else {
            ids.insert(id);
        }
        raw.insert(id, tile);
        i += 12;
    }
    let (mut minx, mut miny) = (0, 0);
    while !ids.is_empty() {
        let mut remove = Vec::new();
        for &id in &ids {
            let mut new = None;
            'matched: for (&(tilex, tiley), tile) in tiles.iter() {
                for &i in &tile.exposed {
                    for (j, &side) in raw.get(&id).unwrap().border.iter().enumerate() {
                        match tile.border[i].0 {
                            a if a == side.0 => {
                                new = Some((tilex, tiley, i, j, true));
                                break 'matched;
                            }
                            a if a == side.1 => {
                                new = Some((tilex, tiley, i, j, false));
                                break 'matched;
                            }
                            _ => (),
                        };
                    }
                }
            }
            if let Some((mut tilex, mut tiley, i, mut j, flipped)) = new {
                let (offx, offy) = [(0, -1), (1, 0), (0, 1), (-1, 0)][i];
                tilex += offx;
                tiley += offy;
                let mut old = raw[&id].border;
                if flipped {
                    old = [
                        (old[2].1, old[2].0),
                        (old[1].1, old[1].0),
                        (old[0].1, old[0].0),
                        (old[3].1, old[3].0),
                    ];
                    j = match j {
                        0 => 2,
                        2 => 0,
                        x => x,
                    }
                }
                let mut border = [(0, 0); 4];
                let rotation = ((6 + j - i) % 4) as u8;
                let mut k = rotation;
                for edge in &mut border {
                    *edge = old[k as usize];
                    k = (k + 1) % 4;
                }
                let mut exposed: HashSet<usize> = [0, 1, 2, 3].iter().cloned().collect();
                for (m, &(x, y)) in [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().enumerate() {
                    if let Some(adjacent) = tiles.get_mut(&(tilex + x, tiley + y)) {
                        adjacent.exposed.remove(&((m + 2) % 4));
                        exposed.remove(&m);
                    }
                }
                tiles.insert(
                    (tilex, tiley),
                    Tile {
                        id,
                        border,
                        flipped,
                        rotation,
                        exposed,
                    },
                );
                if tilex < minx {
                    minx = tilex;
                }
                if tiley < miny {
                    miny = tiley;
                }
                remove.push(id);
            }
        }
        for id in &remove {
            ids.remove(id);
        }
    }
    (tiles, raw, minx, miny)
}

fn orient_sea_monster(default: &[Vector], flipped: bool, rotation: u8) -> Vec<Vector> {
    default
        .iter()
        .map(|&pos| {
            let mut new = pos;
            if flipped {
                new.y = -new.y;
            }
            for _ in 0..rotation {
                let temp = new.y;
                new.y = -new.x;
                new.x = temp;
            }
            new
        })
        .collect()
}

fn remove_sea_monsters(image: &mut Grid<bool>, sea_monster: Vec<Vector>) -> bool {
    let (mut min, mut max) = (ZERO, ZERO);
    sea_monster.iter().for_each(|&pos| {
        match pos.x {
            a if a < min.x => min.x = a,
            a if a > max.x => max.x = a,
            _ => (),
        }
        match pos.y {
            a if a < min.y => min.y = a,
            a if a > max.y => max.y = a,
            _ => (),
        }
    });
    let mut correct = false;
    for y in -min.y..96 - max.y {
        for x in -min.x..96 - max.x {
            let pos = v!(x, y);
            let mut is_monster = true;
            for &offset in &sea_monster {
                if !image[pos + offset] {
                    is_monster = false;
                    break;
                }
            }
            if is_monster {
                correct = true;
                for &offset in &sea_monster {
                    image[pos + offset] = false;
                }
            }
        }
    }
    correct
}

struct Raw {
    data: Grid<bool>,
    border: [(u16, u16); 4],
}

impl Raw {
    fn from(input: &[&str]) -> Raw {
        let parse = input.iter().flat_map(|line| line.chars()).map(|c| c == '#');
        let data = Grid::from_iter(10, 10, parse);
        let mut border = [(0, 0); 4];
        for a in 0..10 {
            if data[v!(a, 0)] {
                border[0].0 += 1 << (9 - a);
                border[0].1 += 1 << a;
            }
            if data[v!(a, 9)] {
                border[2].0 += 1 << a;
                border[2].1 += 1 << (9 - a);
            }
            if data[v!(0, a)] {
                border[3].0 += 1 << a;
                border[3].1 += 1 << (9 - a);
            }
            if data[v!(9, a)] {
                border[1].0 += 1 << (9 - a);
                border[1].1 += 1 << a;
            }
        }
        Raw { data, border }
    }
}

struct Tile {
    id: u16,
    border: [(u16, u16); 4],
    flipped: bool,
    rotation: u8,
    exposed: HashSet<usize>,
}
