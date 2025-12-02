use std::io::Write;

use itertools::concat;
use utils::*;

// Symbols to replace: 12 21 525152 7922 SOLVE2

#[cfg(test)]
mod tests {
    use utils::get_input;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = get_input!("12-example.txt");
        let result = crate::part1(&lines);
        if result == 21 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 1 example, expected 21 got {}",
                result
            ))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = get_input!("12-example.txt");
        let result = crate::part2(&lines);
        if result == 525152 {
            Ok(())
        } else {
            Err(format!("12: Bad result for Part 2 example, expected 525152 got {}", result))
        }
    }
    */
    #[test]
    fn full() -> Result<(), String> {
        let lines = get_input!("12-full.txt");
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);

        if result1 == 7922 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 1, expected 7922 got {}",
                result1
            ))
        }
        /*
        match (result1, result2) {
            (7922, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("12: Bad result for Part 1, expected 7922 got {}", result1)),
            (7922, _) => Err(format!("12: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("12: Bad result for Part 1 & 2, expected (7922, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
}

fn main() {
    let linesfull = get_input!("12-full.txt");
    let lines1 = get_input!("12-example.txt");
    //let lines2 = get_input!("12-2-example.txt");

    println!("12-full.txt");
    println!("{}", part1(&linesfull));
    // println!("{}\n", part2(&linesfull));

    println!("12-1-example.txt");
    println!("{}", part1(&lines1));
    // println!("{}\n", part2(&lines1));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Spring {
        match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Malformed input data"),
        }
    }
}

#[derive(Clone, Debug)]
struct Row {
    springs: Vec<Spring>,
    pattern: Vec<u8>,
}

fn validate_row(springs: &Vec<Spring>, pattern: &Vec<u8>) -> bool {
    // TODO: Rewrite
    // let mut counters: Vec<u8> = pattern.iter().rev().cloned().collect();

    // let mut looking_for_delimeter = false;
    
    // let mut started_damaged_block = false;
    
    // let mut counting_remaining = false;
    // let mut remainining_slots = 0;

    // let mut cur_pattern = counters.pop().unwrap();

    // let exit_early = false;

    // for spring in springs.iter() {

    //     match spring {
    //         Spring::Operational => {
    //             if looking_for_delimeter {
    //                 looking_for_delimeter = false;
    //             } else if started_damaged_block {
    //                 if cur_pattern == 0 {
    //                     started_damaged_block = false;
    //                 } else {
    //                     return false;
    //                 }
    //             }
    //         }
    //         Spring::Damaged => {
    //             if !started_damaged_block { 
    //                 started_damaged_block = true;
    //             }

    //             if counting_remaining || looking_for_delimeter {
    //                 return false;
    //             } else {
    //                 cur_pattern -= 1;
    //             }
    //         }
    //         Spring::Unknown => {
    //             counting_remaining = true;
    //             if looking_for_delimeter {
    //                 looking_for_delimeter = false;
    //             } else {
    //                 remainining_slots += 1;
    //             }
    //         }
    //     }

    //     if cur_pattern == 0 {
    //         if started_damaged_block {
    //             started_damaged_block = false;
    //         }

    //         cur_pattern = match counters.pop() {
    //             Some(new_pattern) => {
    //                 looking_for_delimeter = true;
    //                 new_pattern
    //             }
    //             None => {
    //                 counting_remaining = true;
    //                 continue;
    //             }
    //         }
    //     }
    // }

    // // Check if count is smaller than remaining
    // if springs.iter().filter(|&&s| s == Spring::Unknown).count() == 0 {
    //     println!("returning validate\n{:?} ({:?})\nneeded slots {}, remaining {} (ret {})", springs.iter().map(|s| match s {
    //         Spring::Damaged => "#",
    //         Spring::Operational => ".",
    //         Spring::Unknown => "?"
    //     }).collect::<String>(), pattern, cur_pattern + counters.iter().sum::<u8>(), remainining_slots, counters.iter().sum::<u8>() <= remainining_slots);
    // }
    // return cur_pattern + counters.iter().sum::<u8>() <= remainining_slots;
    unimplemented!()
}

fn iterate_pattern(row: &Row) -> u128 {
    let mut ret = 0;
    // let mut springs = row.springs.clone();
    let mut queue = vec![row.springs.clone()];

    'outer: while let Some(springs) = queue.pop() {
        let validated = validate_row(&springs, &row.pattern);
        
        // println!("{row:?}\n{validated}");

        if !validated {
            continue 'outer;
        }

        for idx in 0..springs.len() {
            let spring = springs[idx];
            match spring {
                Spring::Unknown => {
                    let mut new_damaged = springs.clone();
                    new_damaged[idx] = Spring::Damaged;
                    queue.push(new_damaged);

                    
                    let mut new_operational = springs.clone();
                    new_operational[idx] = Spring::Operational;
                    queue.push(new_operational);


                    // println!("{queue:#?}");
                    continue 'outer;
                },
                _ => continue
            }
        }

        // No break
        ret += 1;
        // println!("{:?} ({:?})", springs.iter().map(|s| match s {
        //     Spring::Damaged => "#",
        //     Spring::Operational => ".",
        //     Spring::Unknown => "?"
        // }).collect::<String>(), row.pattern)
    }

    ret
}

fn part1(lines: &Vec<String>) -> u128 {
    let rows = lines
        .iter()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut spl| {
            let springs_str = spl.next().unwrap();
            let pattern_str = spl.next().unwrap();

            Row {
                springs: springs_str.chars().map(|c| Spring::from(c)).collect(),
                pattern: pattern_str.split(',').map(|s| s.parse().unwrap()).collect(),
            }
        });

    rows.map(|row| iterate_pattern(&row)).sum()
}

// fn part2(lines: &Vec<String>) -> u128 {
//     let parsed = lines.iter().map(|s| {
//         let split = s.split(" ").collect::<Vec<_>>();
//         let criteria: Vec<u128> = split[1].split(',').map(|s| s.parse().unwrap()).collect();
//         //return (split[0], criteria);

//         return (vec![split[0]].repeat(5).join("?"), criteria.repeat(5));
//     }).collect::<Vec<_>>();
// }
