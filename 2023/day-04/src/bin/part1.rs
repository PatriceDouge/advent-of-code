fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(&input);
    println!("{}", output);
}

fn process(input: &str) -> String {
    let mut point_sum = 0;

    for line in input.lines() {
        let mut points = 0;
        let card_info: Vec<&str> = line.split(":").collect();
        
        let nums = card_info[1].trim().split(" | ").collect::<Vec<&str>>();

        let winning_nums = nums[0].split(" ").collect::<Vec<&str>>();
        let actual_nums = nums[1].split(" ").collect::<Vec<&str>>();
        
        for num in actual_nums {
            if winning_nums.contains(&num) {
                points += 1;
            }
        }

    }

    return "1".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("1", process(input));
        Ok(())
    }
}