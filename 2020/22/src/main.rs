aoc::parts!(1, 2);

use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;

fn part_1(input: &[&str]) -> impl ToString {
    let (mut player1, mut player2) = decks(input);
    while player1.len() * player2.len() > 0 {
        let (one, two) = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
        if one > two {
            player1.push_back(one);
            player1.push_back(two);
        } else {
            player2.push_back(two);
            player2.push_back(one);
        }
    }
    score(if player1.is_empty() { player2 } else { player1 })
}

fn part_2(input: &[&str]) -> impl ToString {
    let (player1, player2) = decks(input);
    play(player1, player2).abs()
}

fn play(mut player1: VecDeque<u8>, mut player2: VecDeque<u8>) -> i32 {
    let mut previous: HashSet<(VecDeque<u8>, VecDeque<u8>)> = HashSet::default();
    while player1.len() * player2.len() > 0 {
        let state = (player1.clone(), player2.clone());
        if previous.contains(&state) {
            return 1;
        } else {
            previous.insert(state);
        }
        let (one, two) = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
        let winner = if player1.len() as u8 >= one && player2.len() as u8 >= two {
            let (mut new1, mut new2) = (player1.clone(), player2.clone());
            new1.truncate(one as usize);
            new2.truncate(two as usize);
            play(new1, new2) > 0
        } else {
            one > two
        };
        if winner {
            player1.push_back(one);
            player1.push_back(two);
        } else {
            player2.push_back(two);
            player2.push_back(one);
        }
    }
    if player1.is_empty() {
        -score(player2)
    } else {
        score(player1)
    }
}

fn score(deck: VecDeque<u8>) -> i32 {
    deck.iter()
        .enumerate()
        .map(|(i, &val)| val as i32 * (deck.len() - i) as i32)
        .sum()
}

fn decks(input: &[&str]) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut decks = input.split(|line| line.is_empty());
    (
        decks
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect(),
        decks
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect(),
    )
}
