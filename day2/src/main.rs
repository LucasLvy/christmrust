use std::fs;
fn main() {
    let start =
        fs::read_to_string("input/strat.txt").expect("Should have been able to read the file");

    // A = rock loses to C and is worth 1 point
    // B = paper loses to A and is worth 2 points
    // C = scissors loses to B and is worth 3 points
    let mut score = 0;
    // X lose 0 point
    // Y tie 3 points
    // Z win 6 points
    start.split('\n').for_each(|game| match game {
        "B X" => score += 1, // I lose to paper so get 0 point + 1 point
        "C Y" => score += 6, // I tie scissors so get 3 point + 3 point
        "A Z" => score += 8, // I win rock so get 6 point + 2 point
        "A X" => score += 3, // I lose to rock so get 0 point + 3 point
        "B Y" => score += 5, // I tie to paper so get 3 point + 2 point
        "C Z" => score += 7, // I win to scissors so get 6 point + 1 point
        "C X" => score += 2, // I lose to scissors so get 0 point + 2 point
        "A Y" => score += 4, // I tie to rock so get 3 point + 1 point
        "B Z" => score += 9, // I win to paper so get 6 point + 3 point
        _ => print!("here"),
    });

    println!("The contents of the file are: {:?}", score);
}
