use std::fs;
fn main() {
    let contents =
        fs::read_to_string("input/cal.txt").expect("Should have been able to read the file");
    let mut cals: Vec<i128> = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split('\n')
                .into_iter()
                .map(|s| s.parse::<i128>().unwrap())
                .sum()
        })
        .collect();
    cals.sort_unstable();
    println!(
        "The contents of the file are: {:?}",
        cals[cals.len() - 1] + cals[cals.len() - 2] + cals[cals.len() - 3]
    );
}
