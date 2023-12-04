const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("../../input1.txt");
    let output: i32 = process(&input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let game_info: Vec<&str> = line.split(':').collect();
        let game = game_info[0].trim();
        let game_id: u32 = game.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap();
        let mut game_is_possible: bool = true;
        let game_cubes = game_info[1].trim();

        for turn in game_cubes.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let color_info = turn.trim().split(',').collect::<Vec<&str>>();
            
            for color in color_info  {
                let color_map = color.trim().split_whitespace().collect::<Vec<&str>>();

                let color_count: u32 = color_map[0].trim().parse::<u32>().unwrap();
                let color_name = color_map[1].trim();
                
                match color_name {
                    "red" => red = color_count,
                    "green" => green = color_count,
                    "blue" => blue = color_count,
                    _ => continue,
                };
            }

            if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
                game_is_possible = false;
            }
        }
        if game_is_possible {
            sum += game_id as i32;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            Game 1: 6 red, 1 blue, 13 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input));
        Ok(())
    }
}