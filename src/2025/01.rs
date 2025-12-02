use utils::*;

// Symbols to replace: 01 3 6 1055 6386

#[cfg(test)]
mod tests {
    use utils::get_input;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = get_input!("01-example.txt");
        let result = crate::part1(&lines);
        if result == 3 {
            Ok(())
        } else {
            Err(format!(
                "01: Bad result for Part 1 example, expected 3 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = get_input!("01-example.txt");
        let result = crate::part2(&lines);
        if result == 6 {
            Ok(())
        } else {
            Err(format!(
                "01: Bad result for Part 2 example, expected 6 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = get_input!("01-full.txt");
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (1055, 6386) => Ok(()),
            (_, 6386) => Err(format!(
                "01: Bad result for Part 1, expected 1055 got {}",
                result1
            )),
            (1055, _) => Err(format!(
                "01: Bad result for Part 2, expected 6386 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "01: Bad result for Part 1 & 2, expected (1055, 6386) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = get_input!("01-full.txt");
    let lines1 = get_input!("01-example.txt");

    println!("01-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("01-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut dial: i32 = 50;
    let mut ret: i32 = 0;

    lines.iter().for_each(|s| {
        match (s.chars().next().unwrap(), s[1..].to_string()) {
            ('R', shift) => {
                dial += shift.parse::<i32>().unwrap();
                dial %= 100;
            }
            ('L', shift) => {
                dial -= shift.parse::<i32>().unwrap();
                dial %= 100;
            }
            _ => unreachable!(),
        }

        if dial == 0 {
            ret += 1;
        }
    });

    ret
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut dial: i32 = 50;
    let mut ret: i32 = 0;

    // lines.iter().for_each(|s| {
    //     debug!("{dial} ({ret}) ---- {s} ----> ");
    //     match (s.chars().next().unwrap(), s[1..].to_string()) {
    //         ('R', shift) => {
    //             let parsed = shift.parse::<i32>().unwrap();
    //             let newval = dial + parsed;

    //             if dial != 0 && newval == 0 {
    //                 ret += (dial / 100) + 1;
    //             } else if dial == 0  && newval != 0 {
    //                 ret += (newval / 100).abs();
    //             } else if newval != 0 && dial.signum() == newval.signum() {
    //                 // println!("({parsed} / 100).abs_diff({dial} / 100) == {}", (parsed / 100).abs_diff(dial / 100));
    //                 ret += (newval / 100).abs_diff(dial / 100) as i32;
    //                 // print!(" mi! ");
    //             } else if dial.signum() != newval.signum() {
    //                 ret += (newval / 100).abs_diff(dial / 100) as i32 + 1;
    //             }

    //             dial = if newval < 0 { 100 + newval % 100 } else { newval % 100 };
    //         }
    //         ('L', shift) => {
    //             let parsed = shift.parse::<i32>().unwrap();
    //             let newval = dial - parsed;

    //             if dial != 0 && newval == 0 {
    //                 ret += (dial / 100).abs() + 1;
    //             } else if dial == 0  && newval != 0 {
    //                 ret += (newval / 100).abs();
    //             } else if newval != 0 && dial.signum() == newval.signum() {
    //                 ret += (newval / 100).abs_diff(dial / 100) as i32 ;
    //             } else if dial.signum() != newval.signum() {
    //                 ret += (newval / 100).abs_diff(dial / 100) as i32 + 1;
    //             }

    //             dial = if newval < 0 { 100 + newval % 100 } else { newval % 100 };
    //         }
    //         _ => unreachable!(),
    //     };
    //     debugln!("{dial} ({ret})");
    // });

    lines.iter().for_each(|s| {
        match (s.chars().next().unwrap(), s[1..].to_string()) {
            ('R', shift) => {
                let mut parsed = shift.parse::<i32>().unwrap();

                while parsed > 0 {
                    dial += 1;
                    parsed -= 1;

                    if dial % 100 == 0 {
                        ret += 1;
                    }
                }
            }
            ('L', shift) => {
                let mut parsed = shift.parse::<i32>().unwrap();

                while parsed > 0 {
                    dial -= 1;
                    parsed -= 1;

                    if dial % 100 == 0 {
                        ret += 1;
                    }
                }
            }
            _ => unreachable!(),
        };
    });

    ret
}
