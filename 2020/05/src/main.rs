aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    input.iter().map(seat_id).max().unwrap()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut occupied = [false; 1 << 10];
    for seat in input {
        occupied[seat_id(seat)] = true;
    }
    let mut i = 1;
    loop {
        if !occupied[i] && occupied[i - 1] && occupied[i + 1] {
            return i;
        }
        i += 1;
    }
}

fn seat_id(seat: impl AsRef<str>) -> usize {
    let mut id = 0;
    for (i, letter) in seat.as_ref().chars().enumerate() {
        if let 'B' | 'R' = letter {
            id |= 1 << (9 - i);
        }
    }
    id
}
