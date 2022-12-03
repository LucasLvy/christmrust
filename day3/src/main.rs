use std::fs;
fn main() {
    part2();
}

fn part1() {
    let sacks =
        fs::read_to_string("input/rucksacks.txt").expect("Should have been able to read the file");
    let mut prio: u32 = 0;
    sacks.split('\n').for_each(|sack| {
        let sack_vec: Vec<char> = sack.chars().collect();
        let mut already_seen: Vec<char> = vec![];
        sack_vec[..sack_vec.len() / 2].iter().for_each(|c| {
            if sack_vec[sack_vec.len() / 2..].to_vec().contains(c) && !already_seen.contains(c) {
                prio += if c.is_ascii_lowercase() {
                    *c as u32 - 64 - 32
                } else {
                    *c as u32 - 64 + 26
                };
            }
            already_seen.push(*c);
        });
    });
    println!("The sum of the priorities is: {:?}", prio);
}

fn part2() {
    let sacks =
        fs::read_to_string("input/rucksacks.txt").expect("Should have been able to read the file");
    let sacks: Vec<&str> = sacks.split('\n').into_iter().collect();
    let mut prio: u32 = 0;
    for i in 0..sacks.len() / 3 {
        let mut found = false;
        sacks[3 * i].to_owned().chars().for_each(|c| {
            if sacks[3 * i + 1].contains(c) && sacks[3 * i + 2].contains(c) && !found {
                found = true;
                prio += if c.is_ascii_lowercase() {
                    c as u32 - 64 - 32
                } else {
                    c as u32 - 64 + 26
                };
            }
        });
    }
    println!("The sum of the priorities is: {:?}", prio);
}
