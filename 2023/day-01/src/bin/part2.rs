use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input2.txt");
    let output = process((&input));
    println!("{}", output);
}

fn fix_digits(input: &str) -> String {
    let digit_map = HashMap::from([
        ("one", "o1ne"),
        ("two", "tw2o"),
        ("three", "th3ree"),
        ("four", "fou4r"),
        ("five", "fiv5e"),
        ("six", "si6x"),
        ("seven", "sev7en"),
        ("eight", "ei8ght"),
        ("nine", "nin9e"),
    ]);

    let mut new_lines = Vec::new();
    for line in input.lines() {
        let mut new_line = line.to_string();
        for (key, value) in digit_map.iter() {
            if new_line.contains(key) {
                new_line = new_line.replace(key, value);
            }
        }
        new_lines.push(new_line);
    }
    return new_lines.join("\n").to_string();
}
            
        fn process(input: &str) -> String {
            let fixed_input = fix_digits(input);

            let mut sum = 0;
            for line in fixed_input.lines() {
                let mut vec = Vec::new();
                for c in line.chars() {
                    if c.is_numeric() {
                        vec.push(c);
                    }
                }
                if vec.len() > 1 {
                    sum += (vec[0].to_string() + &(vec[vec.len() - 1].to_string())).parse::<i32>().unwrap();
                    continue;
                } if vec.len() > 0 {
                    sum += (vec[0].to_string() + &(vec[0].to_string())).parse::<i32>().unwrap();
                    continue;
                } else {
                    continue;
                }
            }
            return sum.to_string()
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_process() -> miette::Result<()> {
                let input = "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen";
                assert_eq!("281", process(input));
                Ok(())
            }
        }
