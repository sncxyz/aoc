use aoc::{IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    part_n(input, false)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_n(input, true)
}

fn part_n(input: aoc::Input, jokers: bool) -> u32 {
    let mut hands: Vec<_> = input.lines().map(|l| Hand::parse(l, jokers)).collect();
    hands.sort_unstable_by_key(|h| (h.kind, h.cards));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| h.bet * (i as u32 + 1))
        .sum()
}

struct Hand {
    cards: [Card; 5],
    kind: Kind,
    bet: u32,
}

impl Hand {
    fn parse(line: &str, j: bool) -> Self {
        let cards = line.bytes().take(5).map(|c| Card::new(c, j)).collect_n();
        let kind = Kind::new(cards);
        let bet = line[6..].parse_uw();
        Self { cards, kind, bet }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Kind {
    fn new(hand: [Card; 5]) -> Self {
        let mut hand: Vec<_> = hand.into_iter().filter(|&c| c != Card::Joker).collect();
        let jokers = 5 - hand.len() as u32;
        hand.sort_unstable();
        let mut counts = [0; 5];
        if !hand.is_empty() {
            counts[0] = 1;
        }
        let mut i = 0;
        for w in hand.windows(2) {
            if w[0] == w[1] {
                counts[i] += 1;
            } else {
                i += 1;
                counts[i] = 1;
            }
        }
        counts.sort_unstable_by_key(|&c| std::cmp::Reverse(c));
        counts[0] += jokers;
        match counts {
            [5, 0, 0, 0, 0] => Self::FiveKind,
            [4, 1, 0, 0, 0] => Self::FourKind,
            [3, 2, 0, 0, 0] => Self::FullHouse,
            [3, 1, 1, 0, 0] => Self::ThreeKind,
            [2, 2, 1, 0, 0] => Self::TwoPair,
            [2, 1, 1, 1, 0] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(byte: u8, jokers: bool) -> Self {
        match byte {
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'T' => Self::Ten,
            b'J' if jokers => Self::Joker,
            b'J' => Self::Jack,
            b'Q' => Self::Queen,
            b'K' => Self::King,
            b'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}
