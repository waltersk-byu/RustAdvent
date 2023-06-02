// Include the day modules
// The code for each daily puzzle is in its own file
mod day1;
mod day2;

//
// main entry point for the program.
// Call a method for each daily puzzle 
//
fn main() {
    println!("Advent 2022 solutions in Rust");

    // Day 1
    let mut day1_solution = day1::find_most_calories(false);
    println!("Day 1 part 1 solution is {day1_solution}");
    day1_solution = day1::find_most_calories(true);
    println!("Day 1 part 2 solution is {day1_solution}");

    // Day 2
    let mut day2_solution: u64 = day2::find_paper_rock_scissors_score(false);
    println!("Day 2 part 1 solution is {day2_solution}");
    day2_solution = day2::find_paper_rock_scissors_score(true);
    println!("Day 2 part 2 solution is {day2_solution}");
}
