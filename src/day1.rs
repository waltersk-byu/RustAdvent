use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

///
/// Day 1 advent solution 
/// Find the elf with the most calories 
/// 
pub fn find_most_calories() -> u64 {

    let mut max_elf_calorie_count : u64 = 0;

    // Read the day1 input file
    if let Ok(lines) = read_lines("day1input.txt"){
        let mut elf_calorie_count = 0;

        for line in lines {
            if let Ok(data) = line {
                // break when there is an empty line or it is the last line

                if data.trim().len() == 0 {
                    // its a break check to see if this elf has the most currently
                    if elf_calorie_count > max_elf_calorie_count {
                        max_elf_calorie_count = elf_calorie_count;
                    }

                    elf_calorie_count = 0;
                } else {
                    // add to the current elf count 
                    elf_calorie_count += data.trim().parse::<u64>().expect("invalid number in file");
                }

            }
        }

        // check the last one
        if elf_calorie_count > max_elf_calorie_count {
            max_elf_calorie_count = elf_calorie_count;
        }
   
    }
    max_elf_calorie_count

}

//
// Helper method to return an iterator over a file 
//
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}