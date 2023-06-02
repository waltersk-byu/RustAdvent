use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

///
/// Day 1 advent solution 
/// Find the elf with the most calories, or the top three, depending on which part
/// 
pub fn find_most_calories(part_two: bool) -> u64 {

    let mut top_counts :TopElfs = TopElfs::new();

    // Read the day1 input file
    if let Ok(lines) = read_lines("day1input.txt"){
        let mut elf_calorie_count = 0;

        for line in lines {
            if let Ok(data) = line {
                // break when there is an empty line

                if data.trim().len() == 0 {
                    // its a break add the value to the top counts instance
                    top_counts.new_count(elf_calorie_count);

                    elf_calorie_count = 0;
                } else {
                    // add to the current elf count 
                    // Keep error handling simple, program will crash with expect
                    // if the value cannot be parsed.
                    elf_calorie_count += data.trim().parse::<u64>().expect("invalid number in file");
                }

            }
        }

        // check the last one
        top_counts.new_count(elf_calorie_count);
   
    }

    if part_two {
        // in part two we need to return the sum of the top three 
        top_counts.one + top_counts.two + top_counts.three

    } else {
        // for part one we return the value of the top count
        top_counts.one
    }

}

///
/// Helper method to return an iterator over a file 
///
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

///
/// Hold the top three elfs calorie counts 
/// 
struct TopElfs {
    one:u64,
    two:u64,
    three:u64
}

///
/// Methods for Top_Elfs
impl TopElfs {
    /// new
    /// 
    fn new() -> Self {
        Self{one:0, two:0, three: 0}
    }
    ///
    /// get a new count, if it is in the top three save its value
    fn new_count(&mut self, count:u64) {
        if count > self.one {
            self.three = self.two;
            self.two = self.one;
            self.one = count;
        } else if count > self.two {
            self.three = self.two;
            self.two = count;
        } else if count > self.three {
            self.three = count;
        }
    }

}
