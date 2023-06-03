use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// using the substring crate - odd why rust did make this a standard on string
use substring::Substring;

///
/// Day 3 advent solution 
/// Find the matching item in the rugsacks 
/// 
pub fn find_common_item_priority(part_two: bool) -> u64 {

    let mut total_priorty:u64 = 0;

    // Read the day1 input file
    if let Ok(lines) = read_lines("day3input.txt"){

        for line in lines {
            if let Ok(data) = line {
                // Find the items in each of the two compartments
                // Then find the matching item between the two compartments

                let data_trimed = data.trim();
                let compartment1 = data_trimed.substring(0, data_trimed.len() / 2);
                let compartment2 = data_trimed.substring(data_trimed.len() / 2, data_trimed.len());

                // look at each character in compartment1 and then see if it is in compartment2
                let mut matching_item_priority : u64 = 0;

                for c1 in compartment1.chars() {
                    match compartment2.find(c1) { 
                        Some(_) => {
                            //get the priority value
                            // for upper case the priority is the ascii value less 38 (27 - 52)
                            // for lower case the priority is the ascii value less 96 (1 - 26)
                            let ascii_value :u64 = u64::from(c1);
                            if c1.is_ascii_uppercase() {
                                matching_item_priority = ascii_value - 38;
                            } else {
                                matching_item_priority = ascii_value - 96;
                            }
                        },
                        None => {}
                    }
                }

                total_priorty += matching_item_priority;

            }
        }

   
    }


    total_priorty


}

///
/// Helper method to return an iterator over a file 
///
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

