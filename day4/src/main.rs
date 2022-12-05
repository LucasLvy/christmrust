use std::fs;
fn main() {
    part2();
}

fn part1() {
    let zones =
        fs::read_to_string("input/zones.txt").expect("Should have been able to read the file");
    let mut total: u32 = 0;
    zones.split('\n').for_each(|zone| {
        let splitted: Vec<&str> = zone.split(',').collect();
        let first_zone: Vec<&str> = splitted[0].split('-').collect();
        let first_zone =
            first_zone[0].parse::<u32>().unwrap()..first_zone[1].parse::<u32>().unwrap() + 1;
        let second_zone: Vec<&str> = splitted[1].split('-').collect();
        let second_zone =
            second_zone[0].parse::<u32>().unwrap()..second_zone[1].parse::<u32>().unwrap() + 1;

        if first_zone
            .clone()
            .into_iter()
            .all(|item| second_zone.contains(&item))
            || second_zone
                .into_iter()
                .all(|item| first_zone.contains(&item))
        {
            total += 1;
        }
    });
    println!("Total: {}", total);
}

fn part2() {
    let zones =
        fs::read_to_string("input/zones.txt").expect("Should have been able to read the file");
    let mut total: u32 = 0;
    zones.split('\n').for_each(|zone| {
        let splitted: Vec<&str> = zone.split(',').collect();
        let first_zone: Vec<&str> = splitted[0].split('-').collect();
        let first_zone =
            first_zone[0].parse::<u32>().unwrap()..first_zone[1].parse::<u32>().unwrap() + 1;
        let second_zone: Vec<&str> = splitted[1].split('-').collect();
        let second_zone =
            second_zone[0].parse::<u32>().unwrap()..second_zone[1].parse::<u32>().unwrap() + 1;

        if first_zone
            .into_iter()
            .any(|item| second_zone.contains(&item))
        {
            total += 1;
        }
    });
    println!("Total: {}", total);
}
