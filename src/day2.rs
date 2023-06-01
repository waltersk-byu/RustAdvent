use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

///
/// Return the score of the paper rock scissors competition
/// if part_two is true, we are doing the second part of the day puzzle
/// 
pub fn find_paper_rock_scissors_score(part_two : bool) -> u64 {
    
    let mut total_score : u64 = 0;    
    // Read the day2 input file
    if let Ok(lines) = read_lines("day2input.txt"){

        for line in lines {
            if let Ok(data) = line {
                // split the input to get the different moves 
                // Put each move into a vec

                let moves_iterator = data.trim().split(" ");
                let moves = moves_iterator.collect::<Vec<&str>>();

                // There should always be 2 entries 
                debug_assert!(moves.len() == 2);

                if part_two {
                    // in part two my move value is actually how 
                    // the round should end 
                    // X I lose, Y tie, Z I win

                    let mut round_score = 0;

                    // compare the first move to the expected outcome.  The first move is 
                    // A - Rock, B Paper, C scissors
                    match moves[0] {
                        "A" => match moves[1] {
                            "X" => round_score += 3, // I lose with scissors 
                            "Y" => round_score += 4, // I tie with rock 
                            "Z" => round_score += 8, // I win with paper
                            _ => debug_assert!(false),  // we only expect X, Y, or Z
                            }
                        "B" => match moves[1] {
                            "X" => round_score += 1, // I lose with rock
                            "Y" => round_score += 5, // tie paper to paper
                            "Z" => round_score += 9, // I win with scissors
                            _ => debug_assert!(false),  // we only expect X, Y, or Z
                            }
                        "C" => match moves[1] {
                            "X" => round_score += 2, // I lose with paper
                            "Y" => round_score += 6, // I tie with scissors
                            "Z" => round_score += 7, // I win with Rock
                            _ => debug_assert!(false),  // we only expect X, Y, or Z
                            }
                        _ => debug_assert!(false),  // we only expect A, B, C

                    }

                    total_score += round_score;


                } else {
                    // Calculate score for this round
                    // add points for my move, plus points for the outcome of the round
                    let mut round_score = 0;

                    // first get the score for my move
                    // 1 for X (rock)  2 for paper (Y), 3 for scissors (Z)
                    match moves[1] {
                        "X" => round_score += 1,
                        "Y" => round_score += 2,
                        "Z" => round_score += 3,
                        _ => debug_assert!(false),  // we only expect X, Y, or Z
                    }

                    // Now find the winner 
                    // compare the first move to my move.  The first move is 
                    // A - Rock, B Paper, C scissors
                    match moves[0] {
                        "A" => match moves[1] {
                            "X" => round_score += 3, // tie since its rock to rock
                            "Y" => round_score += 6, // I win paper defeats rock
                            "Z" => round_score += 0, // I lose rock defeats scissors 
                            _ => debug_assert!(false),  // we only expect X, Y, or Z
                            }
                        "B" => match moves[1] {
                            "X" => round_score += 0, // I lose paper defeats rock
                            "Y" => round_score += 3, // tie paper to paper
                            "Z" => round_score += 6, // I win - scissors defeats paper
                            _ => debug_assert!(false),  // we only expect X, Y, or Z
                            }
                        "C" => match moves[1] {
                            "X" => round_score += 6, // I win rock defeats scissors
                            "Y" => round_score += 0, // I lose scissors defeats paper
                            "Z" => round_score += 3, // Tie, scissors to scissors
                            _ => debug_assert!(false),  // we only expect X, Y, or Z
                            }
                        _ => debug_assert!(false),  // we only expect A, B, C

                    }

                    //println!("Round score {},{} : {}", moves[0], moves[1], round_score);

                    total_score += round_score;


                }

            }
        }
   
    }


    total_score
}


//
// Helper method to return an iterator over a file 
//
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}