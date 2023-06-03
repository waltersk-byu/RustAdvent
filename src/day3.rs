use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// using the substring crate - odd why rust did make this a standard on string
use substring::Substring;

///
/// Day 3 advent solution 
/// Find the matching item in the rugsacks 
/// 
#[allow(unused_assignments)]
pub fn find_common_item_priority(part_two: bool) -> u64 {

    let mut total_priorty:u64 = 0;

    // Read the day1 input file
    if let Ok(lines) = read_lines("day3input_test.txt"){

        let mut rucksack1 = String::from("");
        let mut rucksack2 = String::from("");
        let mut rucksack3 = String::from("");

        let mut items_in_group = 0;

        for line in lines {
            if let Ok(data) = line {
                // Find the items in each of the two compartments
                // Then find the matching item between the two compartments

                let data_trimed = data.trim();

                if part_two {
                    // part two need to process in groups of three rucksacks
                    if items_in_group == 0 {
                        rucksack1 = data_trimed.to_string();
                        items_in_group += 1;
                    } else if items_in_group == 1 {
                        rucksack2 = data_trimed.to_string();
                        items_in_group += 1;
                    } else {
                        rucksack3 = data_trimed.to_string();

                        // reset group count
                        items_in_group = 0;

                        // We have all three so now find the common item between them
                        for c1 in rucksack1.chars() {
                            match rucksack2.find(c1) { 
                                Some(_) => {
                                    // the value is in the second one, now check the third
                                    match rucksack3.find(c1) {
                                        Some(_) => {
                                            // the item matched in all three, find the priorty
                                            let ascii_value :u64 = u64::from(c1);
                                            if c1.is_ascii_uppercase() {
                                                total_priorty += ascii_value - 38;
                                            } else {
                                                total_priorty += ascii_value - 96;
                                            }
                                        
                                            break;

                                        }
                                        None => {}
                                    }
                                },
                                None => {}
                            }
                        }
                    }
                }
                else {
                    // part 1 we find the item that is similar in each compartment
                    // and get its priority 
                    let compartment1 = data_trimed.substring(0, data_trimed.len() / 2);
                    let compartment2 = data_trimed.substring(data_trimed.len() / 2, data_trimed.len());
    
                    // look at each character in compartment1 and then see if it is in compartment2
                    let mut matching_item_priority : u64 = 0;
    
                    for c1 in compartment1.chars() {
                        match compartment2.find(c1) { 
                            Some(_) => {
                                //get the priority value
                                let ascii_value :u64 = u64::from(c1);
                                if c1.is_ascii_uppercase() {
                                    matching_item_priority = ascii_value - 38;
                                } else {
                                    matching_item_priority = ascii_value - 96;
                                }
                                break;
                            },
                            None => {}
                        }
                    }
    
                    total_priorty += matching_item_priority;
    
                }

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

